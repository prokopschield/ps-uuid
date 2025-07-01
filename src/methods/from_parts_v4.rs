use crate::UUID;

impl UUID {
    /// Constructs a Version 4 (random) UUID from 16 bytes.
    ///
    /// The version and variant fields are set according to RFC 4122.
    #[must_use]
    pub const fn from_parts_v4(bytes: [u8; 16]) -> Self {
        Self::from_bytes(bytes).with_version(4)
    }
}

#[allow(clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests {
    use crate::UUID;

    #[test]
    fn sets_version_and_variant_correctly() {
        // All zeros
        let uuid = UUID::from_parts_v4([0u8; 16]);
        assert_eq!(uuid.bytes[6] >> 4, 0b0100, "Version must be 4");
        assert_eq!(
            uuid.bytes[8] & 0b1100_0000,
            0b1000_0000,
            "Variant must be RFC 4122"
        );

        // All ones
        let uuid = UUID::from_parts_v4([0xFFu8; 16]);
        assert_eq!(uuid.bytes[6] >> 4, 0b0100, "Version must be 4");
        assert_eq!(
            uuid.bytes[8] & 0b1100_0000,
            0b1000_0000,
            "Variant must be RFC 4122"
        );

        // Random pattern
        let mut input = [0xABu8; 16];
        input[6] = 0x12;
        input[8] = 0x34;
        let uuid = UUID::from_parts_v4(input);
        assert_eq!(uuid.bytes[6] >> 4, 0b0100, "Version must be 4");
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

        let uuid = UUID::from_parts_v4(input);

        // Only bytes 6 and 8 are changed
        for i in 0..16 {
            if i == 6 {
                assert_eq!(uuid.bytes[6] & 0xF0, 0x40, "Version bits must be set");
            } else if i == 8 {
                assert_eq!(uuid.bytes[8] & 0xC0, 0x80, "Variant bits must be set");
            } else {
                assert_eq!(uuid.bytes[i], i as u8, "Other bytes must be unchanged");
            }
        }
    }

    #[test]
    fn version_and_variant_are_reported() {
        let uuid = UUID::from_parts_v4([0u8; 16]);
        assert_eq!(uuid.get_version(), Some(4));
        // If you have a variant() method, you can also check that here.
    }
}
