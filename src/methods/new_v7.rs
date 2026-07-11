use std::time::Duration;

use crate::UUID;

impl UUID {
    /// Build an RFC-4122 **Version 7** (Unix-epoch, time-ordered) UUID.
    ///
    /// Layout (big-endian, RFC 9562 §5.7):
    ///
    /// - bytes 0‥=5 – 48-bit Unix epoch timestamp in **milliseconds**
    /// - bytes 6‥=7 – 12 extra timestamp bits: the sub-millisecond fraction
    ///   scaled to 4096 steps (RFC 9562 §6.2 Method 3, ≈244 ns granularity)
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

        // Sub-millisecond nanoseconds -> 12 extra timestamp bits (RFC 9562
        // §6.2 Method 3: the sub-millisecond fraction scaled to 4096 steps).
        // The maximum numerator, 999_999 * 4096, cannot overflow a u64.
        let extra = u64::from(timestamp.subsec_nanos() % 1_000_000) * 4096 / 1_000_000;

        // Byte 6: version (0x7_) + the 4 high bits
        uuid.bytes[6] = 0x70 | (((extra >> 8) & 0x0F) as u8);

        // Byte 7: the 8 low bits
        uuid.bytes[7] = (extra & 0xFF) as u8;

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
        // 987 654 ns inside the millisecond:
        // floor(987_654 * 4096 / 1_000_000) = 4045 = 0xFCD
        let dur = Duration::from_millis(123) + Duration::from_nanos(987_654);
        let uuid = UUID::new_v7(dur, [0; 8]);
        let b = uuid.as_bytes();

        assert_eq!(b[6] & 0x0F, 0x0F);
        assert_eq!(b[7], 0xCD);
    }

    #[test]
    fn extra_timestamp_bits_span_the_full_range() {
        // 999 999 ns scales to 4095 = 0xFFF; the version nibble survives.
        let top = UUID::new_v7(Duration::from_nanos(999_999), [0; 8]);

        assert_eq!(top.as_bytes()[6], 0x7F);
        assert_eq!(top.as_bytes()[7], 0xFF);

        // 0 ns scales to 0x000.
        let bottom = UUID::new_v7(Duration::from_nanos(0), [0; 8]);

        assert_eq!(bottom.as_bytes()[6], 0x70);
        assert_eq!(bottom.as_bytes()[7], 0x00);
    }

    #[test]
    fn a_256_ns_step_always_advances_the_encoding() {
        // The generator's self-advance is 256 ns; each step must strictly
        // increase the encoded (millisecond, extra-bits) pair.
        let base = Duration::from_millis(123);

        let mut previous = UUID::new_v7(base, [0; 8]);

        for step in 1..3907u64 {
            let current = UUID::new_v7(base + Duration::from_nanos(step * 256), [0; 8]);

            assert!(
                current.bytes > previous.bytes,
                "step {step} did not advance the encoding"
            );

            previous = current;
        }
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
