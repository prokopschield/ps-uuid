use crate::UUID;

// ────────────────────────────────────────────────────────────────────────────
// v6 constructor from individual wire-format fields
// ────────────────────────────────────────────────────────────────────────────

impl UUID {
    /// Builds an RFC-4122 *Version 6* (time-ordered) UUID from its
    /// constituent fields.
    ///
    /// Timestamp layout (big-endian, network order):
    /// - `time_high` – most-significant 32 bits of the 60-bit timestamp
    /// - `time_mid`  – next 16 bits of the timestamp
    /// - `time_low`  – least-significant 12 bits of the timestamp
    ///
    /// Remaining fields:
    /// - `clock_seq` – 14-bit clock sequence (high-to-low order)
    /// - `node_id`   – 48-bit node identifier (usually a MAC address)
    ///
    /// The function performs all bit manipulation internally, then calls
    /// `.with_version(6)` to patch in the *version* nibble (0b0110) and
    /// the RFC-4122 variant bits (0b10xxxxxx). It never fails.
    #[inline]
    #[must_use]
    pub fn from_parts_v6(
        time_high: u32,
        time_mid: u16,
        time_low: u16,
        clock_seq: u16,
        node_id: [u8; 6],
    ) -> Self {
        let mut uuid = Self::nil();

        // Timestamp ---------------------------------------------------------
        uuid.bytes[0..4].copy_from_slice(&time_high.to_be_bytes());
        uuid.bytes[4..6].copy_from_slice(&time_mid.to_be_bytes());
        uuid.bytes[6..8].copy_from_slice(&time_low.to_be_bytes());

        // Clock sequence ----------------------------------------------------
        uuid.bytes[8..10].copy_from_slice(&clock_seq.to_be_bytes());

        // Node identifier ---------------------------------------------------
        uuid.bytes[10..16].copy_from_slice(&node_id);

        // Insert version + variant bits -------------------------------------
        uuid.with_version(6)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Variant, UUID};

    #[test]
    fn builds_correct_static_example() {
        // Timestamp: 0x0123456789abcdef
        //   high 32 → 0x01234567
        //   mid  16 → 0x89ab
        //   low  12 → 0xcdef
        // Clock-seq: 0x1234
        // Node ID  : 00-01-02-03-04-05
        let uuid = UUID::from_parts_v6(
            0x0123_4567,
            0x89ab,
            0xcdef,
            0x1234,
            [0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
        );
        let b = uuid.as_bytes();

        // time_high (bytes 0-3)
        assert_eq!(&b[0..4], &[0x01, 0x23, 0x45, 0x67]);
        // time_mid  (bytes 4-5)
        assert_eq!(&b[4..6], &[0x89, 0xab]);

        // time_low & version (bytes 6-7)
        // original 0xcd ef  →  version nibble patched ⇒ 0x6d ef
        assert_eq!(b[6] & 0x0F, 0x0D);
        assert_eq!(b[6] >> 4, 0x6);
        assert_eq!(b[7], 0xEF);

        // clock_seq & variant (bytes 8-9)
        // original 0x12 34  →  variant patch ⇒ 0x92 34
        assert_eq!(b[9], 0x34);
        assert_eq!(b[8] & 0x3F, 0x12);
        assert_eq!(b[8] >> 6, 0b10);

        // node_id (bytes 10-15)
        assert_eq!(&b[10..16], &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);

        // High-level helpers
        assert_eq!(uuid.version(), Some(6));
        assert_eq!(uuid.variant(), Variant::OSF);
    }

    #[test]
    fn nil_timestamp_yields_valid_uuid() {
        let uuid = UUID::from_parts_v6(0, 0, 0, 0, [0; 6]);
        assert_eq!(uuid.version(), Some(6));
        assert_eq!(uuid.variant(), Variant::OSF);

        // Only version & variant bits should be non-zero.
        let mut expected = [0u8; 16];
        expected[6] = 0x60; // version 6 nibble
        expected[8] = 0x80; // variant 10xxxxxx
        assert_eq!(uuid.as_bytes(), &expected);
    }

    #[test]
    fn byte_order_is_big_endian() {
        let uuid = UUID::from_parts_v6(1, 1, 1, 1, [0; 6]);
        let b = uuid.as_bytes();

        // Each field must appear in network order.
        assert_eq!(&b[0..4], &[0x00, 0x00, 0x00, 0x01]); // time_high
        assert_eq!(&b[4..6], &[0x00, 0x01]); // time_mid
        assert_eq!(b[6], 0x60); // version nibble
        assert_eq!(b[7], 0x01); // low byte of time_low
        assert_eq!(b[9], 0x01); // low byte of clock_seq
    }
}
