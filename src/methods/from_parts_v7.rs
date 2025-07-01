use crate::UUID;

// ────────────────────────────────────────────────────────────────────────────
// v7 constructor from wire-format fields
// ────────────────────────────────────────────────────────────────────────────
//
// Layout (big-endian, RFC-4122-draft, 2022-07-07):
//
//   0‥47   unix_ts_ms   48-bit Unix-epoch time in *milliseconds*
//   48‥51  version      0b0111
//   52‥63  rand_a       12 random bits
//   64‥65  variant      0b10          (inserted by `.with_version`)
//   66‥127 rand_b       62 random bits
//
impl UUID {
    /// Build a Version-7 (time-ordered) UUID from its constituent fields.
    ///
    /// Arguments
    /// - `unix_ts_ms` – milliseconds since 1970-01-01 00:00:00 UTC  
    ///   (only the least-significant 48 bits are used)
    /// - `rand_a` – 12 bits of random data
    /// - `rand_b` – 62 additional random bits
    ///
    /// The function never fails; it masks super-fluous upper bits and then
    /// calls `.with_version(7)` which patches in both the version nibble
    /// **and** the RFC-4122 variant.
    #[inline]
    #[must_use]
    pub fn from_parts_v7(unix_ts_ms: u64, rand_a: u16, rand_b: u64) -> Self {
        let mut uuid = Self::nil();

        // -----------------------------------------------------------------
        // 1. 48-bit Unix-timestamp (bytes 0-5)
        // -----------------------------------------------------------------
        let ts = unix_ts_ms & 0xFFFF_FFFF_FFFF; // keep 48 bits
        let ts_be = ts.to_be_bytes(); // 8-byte big-endian
        uuid.bytes[0..6].copy_from_slice(&ts_be[2..8]);

        // -----------------------------------------------------------------
        // 2. 12 bits of random data (“rand_a”, bytes 6-7)
        // -----------------------------------------------------------------
        uuid.bytes[6..8].copy_from_slice(&rand_a.to_be_bytes());

        // -----------------------------------------------------------------
        // 3. 62 additional random bits (“rand_b”, bytes 8-15)
        // -----------------------------------------------------------------
        uuid.bytes[8..16].copy_from_slice(&rand_b.to_be_bytes());

        // -----------------------------------------------------------------
        // 4. Patch in version & variant bits
        // -----------------------------------------------------------------
        uuid.with_version(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    // Helper ---------------------------------------------------------------
    const fn version(b: &[u8; 16]) -> u8 {
        b[6] >> 4
    }
    const fn variant(b: &[u8; 16]) -> u8 {
        b[8] >> 6
    }

    #[test]
    fn static_example_is_encoded_correctly() {
        // ts = 0x0123_4567_89AB (≈ 2023-03-18 01:32:21 UTC)
        let ts = 0x0123_4567_89ABu64;
        let ra = 0x0CDEu16; // 12 random bits
        let rb = 0x0023_4567_89AB_CDEFu64; // 62 random bits

        let uuid = UUID::from_parts_v7(ts, ra, rb);
        let b = uuid.as_bytes();

        // Timestamp (first 6 bytes)
        assert_eq!(&b[0..6], &[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]);

        // rand_a
        //   high nibble patched later by version ⇒ 0x7C
        assert_eq!(b[6] & 0x0F, 0x0C);
        assert_eq!(b[7], 0xDE);

        // rand_b  (bytes 8-15) – only lower 62 bits should match
        let mut rb_expected = [0u8; 8];
        rb_expected.copy_from_slice(&rb.to_be_bytes());
        // upper 2 bits will be replaced by variant “10”
        assert_eq!(b[8] & 0x3F, rb_expected[0] & 0x3F);
        assert_eq!(&b[9..16], &rb_expected[1..8]);

        // Version / Variant
        assert_eq!(uuid.get_version(), Some(7));
        assert_eq!(uuid.get_variant(), Variant::OSF);
        assert_eq!(version(b), 0b0111);
        assert_eq!(variant(b), 0b10);
    }

    #[test]
    fn nil_timestamp_and_zero_random_yields_valid_uuid() {
        let uuid = UUID::from_parts_v7(0, 0, 0);
        let mut expected = [0u8; 16];
        expected[6] = 0x70; // version nibble
        expected[8] = 0x80; // variant bits
        assert_eq!(uuid.as_bytes(), &expected);
        assert_eq!(uuid.get_version(), Some(7));
        assert_eq!(uuid.get_variant(), Variant::OSF);
    }

    #[test]
    fn byte_order_is_big_endian() {
        let ts = 1u64; // 0x0000_0000_0001
        let uuid = UUID::from_parts_v7(ts, 0, 0);
        let b = uuid.as_bytes();
        assert_eq!(
            &b[0..6],
            &[0x00, 0x00, 0x00, 0x00, 0x00, 0x01],
            "timestamp must be big-endian"
        );
    }
}
