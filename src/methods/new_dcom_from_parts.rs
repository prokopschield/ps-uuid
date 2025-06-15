use crate::{Variant, UUID};

impl UUID {
    /// Creates a new DCOM UUID with the specified `time_low`, `time_mid`, `time_hi_and_version`,
    /// `clock_seq`, and node fields.
    ///
    /// # Arguments
    /// * `time_low` - The low field of the timestamp (32 bits)
    /// * `time_mid` - The middle field of the timestamp (16 bits)
    /// * `time_hi_and_version` - The high field of the timestamp and version (16 bits)
    /// * `clock_seq` - The clock sequence (14 bits, but passed as 16 bits)
    /// * `node` - The node ID (48 bits, passed as 6 bytes)
    ///
    /// # Returns
    /// A UUID with the DCOM variant (0b110) set
    #[must_use]
    pub fn new_dcom_from_parts(
        time_low: u32,
        time_mid: u16,
        time_hi_and_version: u16,
        clock_seq: u16,
        node: [u8; 6],
    ) -> Self {
        let mut uuid = Self::nil();

        // Set time_low (first 4 bytes, little-endian)
        uuid.bytes[0..4].copy_from_slice(&time_low.to_le_bytes());

        // Set time_mid (next 2 bytes, little-endian)
        uuid.bytes[4..6].copy_from_slice(&time_mid.to_le_bytes());

        // Set time_hi_and_version (next 2 bytes, little-endian)
        uuid.bytes[6..8].copy_from_slice(&time_hi_and_version.to_le_bytes());

        // Set clock_seq (next 2 bytes, little-endian)
        uuid.bytes[8..10].copy_from_slice(&clock_seq.to_be_bytes());

        // Set node (last 6 bytes)
        uuid.bytes[10..16].copy_from_slice(&node);

        // Set DCOM variant (0b110 in bits 7-5 of byte 8)
        uuid.with_variant(Variant::DCOM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dcom_from_parts_uuid() {
        let uuid = UUID::new_dcom_from_parts(
            0x1234_5678,
            0x9ABC,
            0xDEF0,
            0x1234,
            [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
        );

        let expected_bytes = [
            0x78, 0x56, 0x34, 0x12, // time_low (little-endian)
            0xBC, 0x9A, // time_mid (little-endian)
            0xF0, 0xDE, // time_hi_and_version (little-endian)
            0xD2, 0x34, // clock_seq (big-endian, with variant 0b110)
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, // node
        ];

        assert_eq!(uuid.bytes, expected_bytes);
    }

    #[test]
    fn test_dcom_variant() {
        let uuid = UUID::new_dcom_from_parts(
            0,      // time_low
            0,      // time_mid
            0,      // time_hi_and_version
            0,      // clock_seq
            [0; 6], // node
        );

        // Check variant bits (byte 8, bits 7-5 should be 0b110)
        let variant_bits = (uuid.bytes[8] & 0xE0) >> 5;
        assert_eq!(variant_bits, 0x06); // 0b110
    }

    #[test]
    fn test_nil_uuid() {
        let nil_uuid = UUID::nil();
        assert_eq!(nil_uuid.bytes, [0; 16]);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_dcom_uuid_fields_preservation() {
        let time_low = 0x1234_5678;
        let time_mid = 0x9ABC;
        let time_hi_and_version = 0xDEF0;
        let clock_seq = 0x1234;
        let node = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let uuid =
            UUID::new_dcom_from_parts(time_low, time_mid, time_hi_and_version, clock_seq, node);

        // Verify each field
        assert_eq!(
            u32::from_le_bytes(uuid.bytes[0..4].try_into().unwrap()),
            time_low
        );
        assert_eq!(
            u16::from_le_bytes(uuid.bytes[4..6].try_into().unwrap()),
            time_mid
        );
        assert_eq!(
            u16::from_le_bytes(uuid.bytes[6..8].try_into().unwrap()),
            time_hi_and_version
        );
        assert_eq!(uuid.bytes[10..16], node);
    }

    #[test]
    fn test_dcom_uuid_variant_preservation() {
        // Create a UUID with a clock_seq that might interfere with variant bits
        let uuid = UUID::new_dcom_from_parts(
            0, 0, 0, 0xFFFF, // clock_seq with all bits set
            [0; 6],
        );

        // Verify variant is still DCOM (0b110)
        let variant_bits = (uuid.bytes[8] & 0xE0) >> 5;
        assert_eq!(variant_bits, 0x06); // 0b110
    }

    #[test]
    fn test_dcom_uuid_zero_values() {
        let uuid = UUID::new_dcom_from_parts(0, 0, 0, 0, [0; 6]);

        // Should be all zeros except for variant bits
        let mut expected = [0; 16];
        expected[8] = 0xC0; // Variant bits set to 0b110

        assert_eq!(uuid.bytes, expected);
    }

    #[test]
    fn test_dcom_uuid_max_values() {
        let uuid = UUID::new_dcom_from_parts(u32::MAX, u16::MAX, u16::MAX, u16::MAX, [0xFF; 6]);

        let expected = [
            0xFF, 0xFF, 0xFF, 0xFF, // time_low
            0xFF, 0xFF, // time_mid
            0xFF, 0xFF, // time_hi_and_version
            0xDF, 0xFF, // clock_seq (variant will modify MSB to 0b110)
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // node
        ];

        assert_eq!(uuid.bytes, expected);
    }

    #[test]
    fn test_new_dcom_from_parts_valid_input() {
        let time_low = 0x1234_5678;
        let time_mid = 0xABCD;
        let time_hi_and_version = 0x1FFF; // Version 1 for DCOM
        let clock_seq = 0xEFFF;
        let node = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

        let uuid =
            UUID::new_dcom_from_parts(time_low, time_mid, time_hi_and_version, clock_seq, node);

        // Expected byte layout for DCOM UUID (little-endian for first three fields)
        let expected_bytes = [
            0x78, 0x56, 0x34, 0x12, // time_low
            0xCD, 0xAB, // time_mid
            0xFF, 0x1F, // time_hi_and_version
            0xCF, 0xFF, // clock_seq
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, // node
        ];

        assert_eq!(uuid.bytes, expected_bytes);
    }

    #[test]
    fn test_new_dcom_from_parts_zero_input() {
        let time_low = 0;
        let time_mid = 0;
        let time_hi_and_version = 0;
        let clock_seq = 0;
        let node = [0; 6];

        let uuid =
            UUID::new_dcom_from_parts(time_low, time_mid, time_hi_and_version, clock_seq, node);

        assert_eq!(uuid, UUID::nil().with_variant(Variant::DCOM));
    }

    #[test]
    fn test_new_dcom_from_parts_max_input() {
        let time_low = u32::MAX;
        let time_mid = u16::MAX;
        let time_hi_and_version = u16::MAX;
        let clock_seq = u16::MAX;
        let node = [0xFF; 6];

        let uuid =
            UUID::new_dcom_from_parts(time_low, time_mid, time_hi_and_version, clock_seq, node);

        assert_eq!(uuid, UUID::max().with_variant(Variant::DCOM));
    }

    #[test]
    fn test_new_dcom_from_parts_endianness() {
        let time_low = 0x1122_3344;
        let time_mid = 0x5566;
        let time_hi_and_version = 0x7788;
        let clock_seq = 0x99AA;
        let node = [0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00];

        let uuid =
            UUID::new_dcom_from_parts(time_low, time_mid, time_hi_and_version, clock_seq, node);

        // Check little-endian for time_low, time_mid, time_hi_and_version
        assert_eq!(uuid.bytes[0..4], [0x44, 0x33, 0x22, 0x11]); // time_low
        assert_eq!(uuid.bytes[4..6], [0x66, 0x55]); // time_mid
        assert_eq!(uuid.bytes[6..8], [0x88, 0x77]); // time_hi_and_version
                                                    // Check big-endian for clock_seq and node
        assert_eq!(uuid.bytes[8..10], [0xD9, 0xAA]); // clock_seq
        assert_eq!(uuid.bytes[10..16], [0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00]); // node
    }
}
