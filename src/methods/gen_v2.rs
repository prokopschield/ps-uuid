use std::time::SystemTime;

use crate::{UuidConstructionError, STATE, UUID};

impl UUID {
    /// Generate v2 UUID (DCE Security)
    ///
    /// The version-2 layout replaces `time_low` with the local ID and
    /// `clock_seq_low` with the domain, so only `time_mid`/`time_hi`
    /// (granularity 2³² ticks, about 429 seconds) and six clock-sequence bits
    /// distinguish UUIDs sharing a domain, local ID, and node. The clock
    /// sequence is drawn from the shared [`STATE`] via [`State::next_v2`],
    /// which advances those six bits on every call, so consecutive calls
    /// yield distinct UUIDs. At most 64 distinct version-2 UUIDs exist per
    /// timestamp window; the sequence is shared with the other time-based
    /// generators, so sustained generation within one window wraps around and
    /// repeats earlier results. This capacity limit is inherent to the
    /// version-2 format.
    ///
    /// Construction is delegated to [`UUID::new_v2`], using the timestamp,
    /// clock sequence, and node ID drawn from the shared [`STATE`].
    ///
    /// [`State::next_v2`]: crate::State::next_v2
    ///
    /// # Errors
    /// - `TimestampBeforeEpoch` is returned if the current time predates 1582-10-15.
    /// - `TimestampOverflow` is returned if the current time exceeds 5236-03-31.
    pub fn gen_v2(domain: u8, local_id: u32) -> Result<Self, UuidConstructionError> {
        let mut guard = STATE.lock();

        let (timestamp, clock_seq) = guard.next_v2(SystemTime::now());
        let node_id = guard.node_id;

        drop(guard);

        Self::new_v2(domain, local_id, timestamp, clock_seq, *node_id)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use super::*;

    // ---------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------

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
        for &domain in &[0, 1, 2, 42] {
            let u = v2(domain, 0xDEAD_BEEF);
            assert_eq!(u.get_version(), Some(2));
        }
    }

    #[test]
    fn variant_bits_remain_rfc4122() {
        let u = v2(1, 123);
        assert!(is_rfc4122_variant(&u));
    }

    #[test]
    fn local_id_is_encoded_big_endian() {
        for &id in &[0, 1, 0x1234_5678, u32::MAX] {
            let u = v2(2, id);
            assert_eq!(local_id(&u), id);
        }
    }

    #[test]
    fn domain_is_written_to_clock_seq_low() {
        for domain in 0u8..=10 {
            let u = v2(domain, 7);
            assert_eq!(u.bytes[9], domain);
        }
    }

    /// Consecutive calls must differ via the surviving clock-sequence bits.
    ///
    /// Concurrent tests share the global [`STATE`](crate::STATE) and may
    /// advance the clock sequence between the two calls; an interleaved
    /// advancement congruent to −2⁸ modulo 2¹⁴ realigns the surviving bits
    /// and legitimately yields an equal pair. A small tolerance keeps the
    /// test deterministic without masking a wiring regression: drawing from
    /// `State::next` instead of `State::next_v2` would leave nearly every
    /// pair equal.
    #[test]
    fn consecutive_calls_produce_distinct_uuids() {
        let mut equal_pairs = 0;

        for _ in 0..100 {
            let first = v2(1, 42);
            let second = v2(1, 42);

            if first == second {
                equal_pairs += 1;
            }
        }

        assert!(
            equal_pairs < 3,
            "consecutive UUIDs must differ via the surviving clock-sequence bits \
             ({equal_pairs} equal pairs out of 100)"
        );
    }

    #[test]
    fn different_local_ids_produce_distinct_uuids() {
        let u1 = v2(1, 0xAAAA_BBBB);
        let u2 = v2(1, 0xCCCC_DDDD);
        assert_ne!(
            u1.bytes, u2.bytes,
            "UUIDs must differ when the Local-ID field changes"
        );
    }

    #[test]
    fn different_domains_produce_distinct_uuids() {
        let u1 = v2(0, 42);
        let u2 = v2(7, 42);
        assert_ne!(
            u1.bytes, u2.bytes,
            "UUIDs must differ when the Domain field changes"
        );
    }
}
