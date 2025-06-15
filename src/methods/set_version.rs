use crate::UUID;

impl UUID {
    pub fn set_version(&mut self, version: u8) {
        self.bytes[6] &= 0x0F;
        self.bytes[6] |= version << 4;

        self.set_variant(crate::Variant::OSF);
    }
}

#[cfg(test)]
mod tests {
    use super::UUID;

    // Helper function to create a UUID with known bytes for testing
    const fn create_test_uuid(bytes: [u8; 16]) -> UUID {
        UUID { bytes }
    }

    // Helper function to check if a UUID has the OSF variant (10xx_xxxx in byte 8)
    const fn has_osf_variant(uuid: &UUID) -> bool {
        let byte8 = uuid.bytes[8];
        (byte8 & 0b1100_0000) == 0b1000_0000
    }

    #[test]
    fn test_set_valid_version() {
        // Test setting versions 1–5 (common UUID versions)
        let original_bytes = [
            0x12, 0x34, 0x56, 0x78, // time_low
            0x9A, 0xBC, // time_mid
            0xDE, 0xF0, // time_hi_and_version, clock_seq_hi_and_reserved
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, // node
        ];
        let mut uuid = create_test_uuid(original_bytes);

        for version in 1..=5 {
            uuid.set_version(version);
            // Check version is set correctly using version()
            assert_eq!(
                uuid.version(),
                Some(version),
                "Version should be set to {version}"
            );
            // Check byte 6: upper 4 bits = version, lower 4 bits preserved (0xE from 0xDE)
            assert_eq!(
                uuid.bytes[6],
                (version << 4) | (original_bytes[6] & 0x0F),
                "Byte 6 incorrect for version {version}"
            );
            // Verify OSF variant
            assert!(
                has_osf_variant(&uuid),
                "OSF variant not set for version {version}"
            );
            // Check other bytes are unchanged
            for (i, _) in original_bytes.iter().enumerate() {
                if i != 6 && i != 8 {
                    assert_eq!(
                        uuid.bytes[i], original_bytes[i],
                        "Byte {i} changed unexpectedly for version {version}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_set_version_zero() {
        // Test setting version 0 (valid but uncommon)
        let original_bytes = [0xFF; 16];
        let mut uuid = create_test_uuid(original_bytes);

        uuid.set_version(0);

        assert_eq!(uuid.version(), Some(0), "Version should be set to 0");
        assert_eq!(
            uuid.bytes[6],
            original_bytes[6] & 0x0F,
            "Byte 6 should preserve lower bits with version 0"
        );
        assert!(has_osf_variant(&uuid), "OSF variant not set for version 0");
        // Check other bytes unchanged except bytes 6 and 8
        for (i, _) in original_bytes.iter().enumerate() {
            if i != 6 && i != 8 {
                assert_eq!(
                    uuid.bytes[i], original_bytes[i],
                    "Byte {i} changed unexpectedly"
                );
            }
        }
    }

    #[test]
    fn test_set_max_version() {
        // Test setting version 15 (maximum possible value)
        let original_bytes = [0x00; 16];
        let mut uuid = create_test_uuid(original_bytes);

        uuid.set_version(15);

        assert_eq!(uuid.version(), Some(15), "Version should be set to 15");
        assert_eq!(uuid.bytes[6], 0xF0, "Byte 6 should be 0xF0 for version 15");
        assert!(has_osf_variant(&uuid), "OSF variant not set for version 15");
        // Check other bytes unchanged except bytes 6 and 8
        for (i, _) in original_bytes.iter().enumerate() {
            if i != 6 && i != 8 {
                assert_eq!(
                    uuid.bytes[i], original_bytes[i],
                    "Byte {i} changed unexpectedly"
                );
            }
        }
    }

    #[test]
    fn test_preserve_lower_nibble_byte6() {
        // Test that lower 4 bits of byte 6 are preserved
        let original_bytes = [
            0x12, 0x34, 0x56, 0x78, // time_low
            0x9A, 0xBC, // time_mid
            0x5F, 0xFF, // time_hi_and_version (0x5F), clock_seq
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, // node
        ];
        let mut uuid = create_test_uuid(original_bytes);

        uuid.set_version(3);

        assert_eq!(uuid.version(), Some(3), "Version should be set to 3");
        assert_eq!(
            uuid.bytes[6], 0x3F,
            "Byte 6 should have version 3 (0x3) in upper nibble and preserve 0xF in lower nibble"
        );
        assert!(has_osf_variant(&uuid), "OSF variant not set");
    }

    #[test]
    fn test_idempotence() {
        // Test that calling set_version multiple times yields consistent results
        let original_bytes = [0xAA; 16];
        let mut uuid = create_test_uuid(original_bytes);

        uuid.set_version(1);
        uuid.set_version(1);

        assert_eq!(
            uuid.bytes, uuid.bytes,
            "Repeated calls with same version should be idempotent"
        );
        assert_eq!(uuid.version(), Some(1), "Version should remain 1");

        uuid.set_version(2);

        assert_eq!(uuid.version(), Some(2), "Version should change to 2");
        assert!(has_osf_variant(&uuid), "OSF variant should persist");
    }

    #[test]
    fn test_version_change_preserves_other_fields() {
        // Test changing version preserves all fields except version and variant
        let original_bytes = [
            0x11, 0x22, 0x33, 0x44, // time_low
            0x55, 0x66, // time_mid
            0x7A, 0x88, // time_hi_and_version (0x7A), clock_seq
            0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, // node
        ];
        let mut uuid = create_test_uuid(original_bytes);

        uuid.set_version(4);

        assert_eq!(uuid.version(), Some(4), "Version should be set to 4");
        // Check all bytes except 6 (version) and 8 (variant)
        let unchanged_indices = [0, 1, 2, 3, 4, 5, 7, 9, 10, 11, 12, 13, 14, 15];
        for i in &unchanged_indices {
            assert_eq!(
                uuid.bytes[*i], original_bytes[*i],
                "Byte {i} should remain unchanged"
            );
        }
        // Check byte 6 preserves lower nibble (0xA from 0x7A)
        assert_eq!(uuid.bytes[6], 0x4A, "Byte 6 should be 0x4A");
        assert!(has_osf_variant(&uuid), "OSF variant not set");
    }

    #[test]
    fn test_version_after_variant_change() {
        // Test that setting version after variant change still works
        let original_bytes = [0x00; 16];
        let mut uuid = create_test_uuid(original_bytes);

        uuid.set_version(5);
        uuid.set_version(3);

        assert_eq!(uuid.version(), Some(3), "Version should be updated to 3");
        assert!(has_osf_variant(&uuid), "OSF variant should be set");
        assert_eq!(
            uuid.bytes[6], 0x30,
            "Byte 6 should reflect version 3 with zero lower nibble"
        );
    }
}
