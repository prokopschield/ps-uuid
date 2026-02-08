use std::time::Duration;

use crate::UUID;

impl UUID {
    /// Build an RFC-4122 **Version 7** (Unix-epoch, time-ordered) UUID.
    ///
    /// Layout (big-endian, draft RFC “UUID Version 7”, 2022-07-07):
    ///
    /// - bytes 0‥=5 – 48-bit Unix epoch timestamp in **milliseconds**
    /// - bytes 6‥=7 – 12 extra timestamp bits derived from the current
    ///   sub-millisecond nanoseconds
    ///   - upper nibble of byte 6 is the version \(7\)
    /// - bytes 8‥=15 – 64 bits of caller-supplied randomness
    ///   - two MSBs of byte 8 are the RFC-4122 variant
    ///
    /// The `timestamp` argument expresses the elapsed time since
    /// 1970-01-01 00:00:00 UTC (`Duration::as_millis()` must fit into
    /// 48 bits).\
    /// `random_bytes` supplies the final eight random bytes that complete the
    /// 128-bit UUID.
    ///
    /// The function never fails; any excess upper bits in the timestamp are
    /// truncated, and the *version* \(0b0111\) and *variant* \(0b10xxxxxx\)
    /// fields are fixed automatically.
    #[must_use]
    pub fn new_v7(timestamp: Duration, random_bytes: [u8; 8]) -> Self {
        let mut uuid = Self::nil();

        // 48-bit Unix-epoch milliseconds (network order)
        uuid.bytes[0..6].copy_from_slice(&timestamp.as_millis().to_be_bytes()[10..16]);

        // Sub-millisecond nanoseconds -> 18 extra timestamp bits
        let nanos = (timestamp.subsec_nanos() % 1_000_000).to_be_bytes();

        // Byte 6: Version (0x7_)  + 4 timestamp bits
        uuid.bytes[6] = 0x70 | nanos[1];

        // Byte 7: next 8 timestamp bits
        uuid.bytes[7] = nanos[2];

        // Caller-provided random payload
        uuid.bytes[8..].copy_from_slice(&random_bytes);

        // Overwrite version & variant once more to guarantee correctness
        uuid.with_version(7)
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Test-suite for `UUID::new_v7`
// ────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Helper – extract version & variant
    const fn version(b: &[u8; 16]) -> u8 {
        b[6] >> 4
    }
    const fn variant(b: &[u8; 16]) -> u8 {
        b[8] >> 6
    }

    #[test]
    fn version_and_variant_are_correct() {
        let uuid = UUID::new_v7(Duration::from_nanos(0), [0; 8]);
        let bytes = uuid.as_bytes();
        assert_eq!(version(bytes), 0b0111, "version must be 7");
        assert_eq!(variant(bytes), 0b10, "variant must be RFC-4122");
    }

    #[test]
    fn timestamp_is_encoded_big_endian() {
        // 0x0123_4567_89AB milliseconds  (≈ 2023-03-18 01:32:21 UTC)
        let ms = 0x0123_4567_89ABu64;
        let dur = Duration::from_millis(ms);
        let uuid = UUID::new_v7(dur, [0; 8]);
        let b = uuid.as_bytes();

        assert_eq!(
            &b[0..6],
            &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB],
            "48-bit millisecond timestamp must be big-endian"
        );
    }

    #[test]
    fn extra_timestamp_bits_are_encoded() {
        // Choose 987 654 ns (0x0F 1206) inside the millisecond
        let dur = Duration::from_millis(123) + Duration::from_nanos(987_654);
        let uuid = UUID::new_v7(dur, [0; 8]);
        let b = uuid.as_bytes();

        let nanos = (987_654u32).to_be_bytes();

        // byte 6 low nibble & byte 7 must match nanos[1] / nanos[2]
        assert_eq!(b[6] & 0x0F, nanos[1]);
        assert_eq!(b[7], nanos[2]);
    }

    #[test]
    fn random_payload_is_inserted_verbatim() {
        let rnd = [0x81, 2, 3, 4, 5, 6, 7, 8];
        let uuid = UUID::new_v7(Duration::from_secs(0), rnd);
        assert_eq!(&uuid.as_bytes()[8..16], &rnd);
    }

    #[test]
    fn identical_input_is_deterministic() {
        let ts = Duration::from_secs(42);
        let rnd = [9, 8, 7, 6, 5, 4, 3, 2];

        let a = UUID::new_v7(ts, rnd);
        let b = UUID::new_v7(ts, rnd);

        assert_eq!(a.bytes, b.bytes, "same input must yield same UUID");
    }

    #[test]
    fn different_random_payloads_differ() {
        let ts = Duration::from_secs(1);
        let a = UUID::new_v7(ts, [0; 8]);
        let b = UUID::new_v7(ts, [1; 8]);
        assert_ne!(a.bytes, b.bytes, "different random bytes must change UUID");
    }
}
