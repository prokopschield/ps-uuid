use rand::RngCore;

use crate::UUID;

impl UUID {
    /// Generates a random (v4) UUID using the provided random number generator.
    #[must_use]
    pub fn new_v4<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        let mut uuid = Self::nil();

        rng.fill_bytes(&mut uuid.bytes);

        uuid.with_version(4)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rand::{rngs::StdRng, SeedableRng};

    use crate::UUID;

    #[test]
    fn version_and_variant_are_set() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            let uuid = UUID::new_v4(&mut rng);
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
    fn produces_unique_uuids() {
        let mut rng = StdRng::seed_from_u64(12345);
        let mut set = HashSet::new();
        for _ in 0..1000 {
            let uuid = UUID::new_v4(&mut rng);
            assert!(set.insert(uuid.bytes), "Duplicate UUID generated!");
        }
    }

    #[test]
    fn all_other_bits_are_random() {
        let mut rng = StdRng::seed_from_u64(98765);
        let mut seen = [0u8; 16];
        let mut seen_inv = [0xFFu8; 16];

        for _ in 0..1000 {
            let uuid = UUID::new_v4(&mut rng);
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
        let mut rng = StdRng::seed_from_u64(5555);
        let uuid = UUID::new_v4(&mut rng);
        assert_eq!(uuid.get_version(), Some(4));
        // If you have a variant() method, you can check it here as well.
    }

    #[test]
    fn deterministic_with_seeded_rng() {
        let mut rng1 = StdRng::seed_from_u64(1);
        let mut rng2 = StdRng::seed_from_u64(1);

        let uuid1 = UUID::new_v4(&mut rng1);
        let uuid2 = UUID::new_v4(&mut rng2);

        assert_eq!(
            uuid1.bytes, uuid2.bytes,
            "Same seed should produce same UUID"
        );
    }
}
