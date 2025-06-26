use crate::UUID;

impl UUID {
    /// Constructs a Version 8 (custom) UUID from 16 bytes.
    ///
    /// The version and variant fields are set according to RFC 4122.
    #[must_use]
    pub const fn from_parts_v8(bytes: [u8; 16]) -> Self {
        Self::from_bytes(bytes).with_version(8)
    }
}

#[allow(clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests {
    use crate::{Variant, UUID};

    #[test]
    fn sets_version_and_variant_correctly() {
        // All zeros
        let uuid = UUID::from_parts_v8([0u8; 16]);
        assert_eq!(uuid.bytes[6] >> 4, 0b1000, "Version must be 8");
        assert_eq!(
            uuid.bytes[8] & 0b1100_0000,
            0b1000_0000,
            "Variant must be RFC 4122"
        );

        // All ones
        let uuid = UUID::from_parts_v8([0xFFu8; 16]);
        assert_eq!(uuid.bytes[6] >> 4, 0b01000, "Version must be 8");
        assert_eq!(
            uuid.bytes[8] & 0b1100_0000,
            0b1000_0000,
            "Variant must be RFC 4122"
        );

        // Random pattern
        let mut input = [0xABu8; 16];
        input[6] = 0x12;
        input[8] = 0x34;
        let uuid = UUID::from_parts_v8(input);
        assert_eq!(uuid.bytes[6] >> 4, 0b1000, "Version must be 8");
        assert_eq!(
            uuid.bytes[8] & 0b1100_0000,
            0b1000_0000,
            "Variant must be RFC 4122"
        );
    }

    #[test]
    fn preserves_other_bytes() {
        let mut input = [0u8; 16];

        for (i, item) in input.iter_mut().enumerate() {
            *item = i as u8;
        }

        let uuid = UUID::from_parts_v8(input);

        // Only bytes 6 and 8 are changed
        for i in 0..16 {
            if i == 6 {
                assert_eq!(uuid.bytes[6] & 0xF0, 0x80, "Version bits must be set");
            } else if i == 8 {
                assert_eq!(uuid.bytes[8] & 0xC0, 0x80, "Variant bits must be set");
            } else {
                assert_eq!(uuid.bytes[i], i as u8, "Other bytes must be unchanged");
            }
        }
    }

    #[test]
    fn version_and_variant_are_reported() {
        let uuid = UUID::from_parts_v8([0u8; 16]);
        assert_eq!(uuid.version(), Some(8));
        assert_eq!(uuid.variant(), Variant::OSF);
    }

    #[test]
    fn preserves_all_payload_bits() {
        let mut src = [0u8; 16];
        for (i, b) in src.iter_mut().enumerate() {
            *b = i as u8;
        }

        let uuid = UUID::from_parts_v8(src);

        for i in 0..16 {
            match i {
                6 => {
                    // High nibble overwritten, low nibble kept
                    assert_eq!(uuid.bytes[6] & 0x0F, src[6] & 0x0F);
                    assert_eq!(uuid.bytes[6] >> 4, 0x8);
                }
                8 => {
                    // Top two bits overwritten, lower six preserved
                    assert_eq!(uuid.bytes[8] & 0x3F, src[8] & 0x3F);
                    assert_eq!(uuid.bytes[8] >> 6, 0b10);
                }
                _ => assert_eq!(uuid.bytes[i], src[i], "byte {i} must be unchanged"),
            }
        }
    }

    // -----------------------------------------------------------
    // Public getters
    // -----------------------------------------------------------
    #[test]
    fn version_and_variant_helpers_report_correctly() {
        let uuid = UUID::from_parts_v8([0u8; 16]);
        assert_eq!(uuid.version(), Some(8));
        assert_eq!(uuid.variant(), Variant::OSF);
    }

    // -----------------------------------------------------------
    // Compile-time construction
    // -----------------------------------------------------------
    const CONST_V8: UUID = UUID::from_parts_v8([1; 16]);

    #[test]
    fn const_construction_matches_runtime() {
        let rt = UUID::from_parts_v8([1; 16]);
        assert_eq!(CONST_V8.bytes, rt.bytes);
    }
}
