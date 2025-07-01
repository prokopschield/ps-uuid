use crate::{Variant, UUID};

impl UUID {
    #[must_use]
    pub const fn variant(&self) -> Variant {
        match self.bytes[8] {
            0x00..=0x7F => Variant::NCS,
            0x80..=0xBF => Variant::OSF,
            0xC0..=0xDF => Variant::DCOM,
            0xE0..=0xFF => Variant::Reserved,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Variant, UUID};

    /// A helper function to create a `UUID` instance where only the 8th byte,
    /// which determines the variant, is significant for our tests.
    /// The other bytes are initialized to zero.
    const fn create_uuid_with_variant_byte(variant_byte: u8) -> UUID {
        let mut bytes = [0u8; 16];
        bytes[8] = variant_byte;
        UUID { bytes }
    }

    #[test]
    fn test_variant_ncs() {
        // The variant bits are 0xx...
        // This corresponds to the byte range 0x00..=0x7F.

        // Test lower boundary
        let uuid_lower = create_uuid_with_variant_byte(0x00);
        assert_eq!(uuid_lower.variant(), Variant::NCS);

        // Test a value within the range
        let uuid_middle = create_uuid_with_variant_byte(0x42);
        assert_eq!(uuid_middle.variant(), Variant::NCS);

        // Test upper boundary
        let uuid_upper = create_uuid_with_variant_byte(0x7F);
        assert_eq!(uuid_upper.variant(), Variant::NCS);
    }

    #[test]
    fn test_variant_osf() {
        // The variant bits are 10x...
        // This corresponds to the byte range 0x80..=0xBF.
        // This is the standard variant for RFC 4122 UUIDs.

        // Test lower boundary
        let uuid_lower = create_uuid_with_variant_byte(0x80);
        assert_eq!(uuid_lower.variant(), Variant::OSF);

        // Test a value within the range (e.g., from a v4 UUID)
        let uuid_middle = create_uuid_with_variant_byte(0xA9);
        assert_eq!(uuid_middle.variant(), Variant::OSF);

        // Test upper boundary
        let uuid_upper = create_uuid_with_variant_byte(0xBF);
        assert_eq!(uuid_upper.variant(), Variant::OSF);
    }

    #[test]
    fn test_variant_dcom() {
        // The variant bits are 110...
        // This corresponds to the byte range 0xC0..=0xDF.

        // Test lower boundary
        let uuid_lower = create_uuid_with_variant_byte(0xC0);
        assert_eq!(uuid_lower.variant(), Variant::DCOM);

        // Test a value within the range
        let uuid_middle = create_uuid_with_variant_byte(0xCB);
        assert_eq!(uuid_middle.variant(), Variant::DCOM);

        // Test upper boundary
        let uuid_upper = create_uuid_with_variant_byte(0xDF);
        assert_eq!(uuid_upper.variant(), Variant::DCOM);
    }

    #[test]
    fn test_variant_reserved() {
        // The variant bits are 111...
        // This corresponds to the byte range 0xE0..=0xFF.

        // Test lower boundary
        let uuid_lower = create_uuid_with_variant_byte(0xE0);
        assert_eq!(uuid_lower.variant(), Variant::Reserved);

        // Test a value within the range
        let uuid_middle = create_uuid_with_variant_byte(0xF0);
        assert_eq!(uuid_middle.variant(), Variant::Reserved);

        // Test upper boundary
        let uuid_upper = create_uuid_with_variant_byte(0xFF);
        assert_eq!(uuid_upper.variant(), Variant::Reserved);
    }
}
