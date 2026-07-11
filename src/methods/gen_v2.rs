use std::time::SystemTime;

use crate::{UuidConstructionError, STATE, UUID};

impl UUID {
    /// Generate v2 UUID (DCE Security)
    ///
    /// The version-2 layout replaces `time_low` with the local ID and
    /// `clock_seq_low` with the domain, so only `time_mid`/`time_hi`
    /// (granularity 2³² ticks, about 429 seconds) and six clock-sequence bits
    /// distinguish UUIDs sharing a domain, local ID, and node. The clock
    /// sequence is drawn from [`STATE`] via [`State::next_v2`], which advances
    /// a dedicated version-2 counter on every call, so consecutive calls yield
    /// distinct UUIDs regardless of concurrent version-1, version-6, or DCOM
    /// generation. At most 64 distinct version-2 UUIDs exist per timestamp
    /// window; the dedicated counter yields all 64 before wrapping around and
    /// repeating earlier results. This capacity limit is inherent to the
    /// version-2 format.
    ///
    /// Construction is delegated to [`UUID::new_v2`], using the timestamp,
    /// clock sequence, and node ID drawn from the shared [`STATE`].
    ///
    /// [`State::next_v2`]: crate::State::next_v2
    ///
    /// # Errors
    /// - `TimestampOverflow` is returned once the shared generator state has
    ///   exhausted the 60-bit timestamp range, which ends 5236-03-31.
    ///
    /// A clock reading before 1582-10-15 or beyond the representable range is
    /// never adopted; generation continues from the last issued tick.
    pub fn gen_v2(domain: u8, local_id: u32) -> Result<Self, UuidConstructionError> {
        let mut guard = STATE.lock();

        let (timestamp, clock_seq) = guard.next_v2(SystemTime::now());
        let node_id = guard.node_id();

        drop(guard);

        Self::new_v2(domain, local_id, timestamp, clock_seq, *node_id)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use std::sync::{Mutex, MutexGuard, PoisonError};

    use super::*;

    // ---------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------

    /// Serializes the tests in this module. The dedicated version-2 counter
    /// is advanced only by version-2 generation, and this module holds the
    /// only version-2 call sites in the test binary, so excluding concurrent
    /// callers makes the clock-sequence assertions deterministic.
    static V2_TRAFFIC: Mutex<()> = Mutex::new(());

    /// Takes the module lock, recovering it if a failed test poisoned it.
    fn exclusive_v2_traffic() -> MutexGuard<'static, ()> {
        V2_TRAFFIC.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Convenience wrapper that panics on construction failure.
    fn v2(domain: u8, local_id: u32) -> UUID {
        UUID::gen_v2(domain, local_id).expect("failed to build v2 UUID")
    }

    /// Extract the 32-bit value stored in the first four bytes
    /// (\( \text{time\_low} \) / Local-ID field).
    fn local_id(u: &UUID) -> u32 {
        u32::from_be_bytes(
            u.bytes[0..4]
                .try_into()
                .expect("UUID timestamp slice should be exactly 4 bytes"),
        )
    }

    /// True iff the RFC-4122 variant bits are `10`.
    const fn is_rfc4122_variant(u: &UUID) -> bool {
        (u.bytes[8] & 0b1100_0000) == 0b1000_0000
    }

    // ---------------------------------------------------------------------
    // Tests
    // ---------------------------------------------------------------------

    #[test]
    fn version_is_always_2() {
        let _guard = exclusive_v2_traffic();

        for &domain in &[0, 1, 2, 42] {
            let u = v2(domain, 0xDEAD_BEEF);
            assert_eq!(u.get_version(), Some(2));
        }
    }

    #[test]
    fn variant_bits_remain_rfc4122() {
        let _guard = exclusive_v2_traffic();

        let u = v2(1, 123);
        assert!(is_rfc4122_variant(&u));
    }

    #[test]
    fn local_id_is_encoded_big_endian() {
        let _guard = exclusive_v2_traffic();

        for &id in &[0, 1, 0x1234_5678, u32::MAX] {
            let u = v2(2, id);
            assert_eq!(local_id(&u), id);
        }
    }

    #[test]
    fn domain_is_written_to_clock_seq_low() {
        let _guard = exclusive_v2_traffic();

        for domain in 0u8..=10 {
            let u = v2(domain, 7);
            assert_eq!(u.bytes[9], domain);
        }
    }

    /// Consecutive calls must differ via the surviving clock-sequence bits.
    ///
    /// The version-2 clock sequence is a dedicated counter stepped by one on
    /// every call, so with no other version-2 traffic interleaved (guaranteed
    /// by the module lock) the six surviving bits of consecutive calls are
    /// never equal: every pair must differ, with no statistical tolerance.
    #[test]
    fn consecutive_calls_produce_distinct_uuids() {
        let _guard = exclusive_v2_traffic();

        for _ in 0..100 {
            let first = v2(1, 42);
            let second = v2(1, 42);

            assert_ne!(
                first, second,
                "consecutive UUIDs must differ via the surviving clock-sequence bits"
            );
        }
    }

    #[test]
    fn different_local_ids_produce_distinct_uuids() {
        let _guard = exclusive_v2_traffic();

        let u1 = v2(1, 0xAAAA_BBBB);
        let u2 = v2(1, 0xCCCC_DDDD);
        assert_ne!(
            u1.bytes, u2.bytes,
            "UUIDs must differ when the Local-ID field changes"
        );
    }

    #[test]
    fn different_domains_produce_distinct_uuids() {
        let _guard = exclusive_v2_traffic();

        let u1 = v2(0, 42);
        let u2 = v2(7, 42);
        assert_ne!(
            u1.bytes, u2.bytes,
            "UUIDs must differ when the Domain field changes"
        );
    }
}
