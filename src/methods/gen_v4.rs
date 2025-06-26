use rand::fill;

use crate::UUID;

impl UUID {
    /// Generates a random (v4) UUID.
    #[must_use]
    pub fn gen_v4() -> Self {
        let mut uuid = Self::nil();

        fill(&mut uuid.bytes);

        uuid.with_version(4)
    }
}

#[cfg(test)]
mod tests {
    use super::UUID;
    use std::collections::HashSet;

    #[test]
    fn version_and_variant_are_set() {
        for _ in 0..100 {
            let uuid = UUID::gen_v4();
            // Version: high nibble of byte 6 must be 0b0100
            assert_eq!(uuid.bytes[6] >> 4, 0b0100, "Version must be 4");
            // Variant: two MSBs of byte 8 must be 0b10
            assert_eq!(
                uuid.bytes[8] & 0b1100_0000,
                0b1000_0000,
                "Variant must be RFC 4122"
            );
        }
    }

    #[test]
    fn randomness_produces_unique_uuids() {
        let mut set = HashSet::new();
        for _ in 0..1000 {
            let uuid = UUID::gen_v4();
            assert!(set.insert(uuid.bytes), "Duplicate UUID generated!");
        }
    }

    #[test]
    fn all_other_bits_are_random() {
        // Generate a bunch of UUIDs and check that at least one bit in each
        // non-fixed field is both 0 and 1 across the sample.
        let mut seen = [0u8; 16];
        let mut seen_inv = [0xFFu8; 16];

        for _ in 0..1000 {
            let uuid = UUID::gen_v4();
            for i in 0..16 {
                seen[i] |= uuid.bytes[i];
                seen_inv[i] &= uuid.bytes[i];
            }
        }

        // Byte 6: lower 4 bits are random, upper 4 bits are version
        assert_ne!(
            seen[6] & 0x0F,
            0,
            "At least one lower bit in byte 6 should be 1"
        );
        assert_ne!(
            seen_inv[6] & 0x0F,
            0x0F,
            "At least one lower bit in byte 6 should be 0"
        );
        // Byte 8: lower 6 bits are random, upper 2 bits are variant
        assert_ne!(
            seen[8] & 0x3F,
            0,
            "At least one lower bit in byte 8 should be 1"
        );
        assert_ne!(
            seen_inv[8] & 0x3F,
            0x3F,
            "At least one lower bit in byte 8 should be 0"
        );
        // All other bytes: all bits are random
        for i in 0..16 {
            if i == 6 {
                // upper 4 bits fixed
                assert_eq!(seen[6] & 0xF0, 0x40, "Version bits must be set to 4");
            } else if i == 8 {
                // upper 2 bits fixed
                assert_eq!(seen[8] & 0xC0, 0x80, "Variant bits must be set to RFC 4122");
            } else {
                assert_ne!(seen[i], 0, "At least one bit in byte {i} should be 1");
                assert_ne!(
                    seen_inv[i], 0xFF,
                    "At least one bit in byte {i} should be 0"
                );
            }
        }
    }

    #[test]
    fn version_and_variant_methods_report_correctly() {
        let uuid = UUID::gen_v4();
        assert_eq!(uuid.version(), Some(4));
        // If you have a variant() method, you can check it here as well.
    }
}
