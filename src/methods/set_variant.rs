use crate::{Variant, UUID};

impl UUID {
    pub const fn set_variant(&mut self, variant: Variant) {
        self.bytes[8] &= variant.bitmask();
        self.bytes[8] |= variant.prefix();
    }
}

#[cfg(test)]
mod tests {
    use super::{Variant, UUID};

    const fn make_uuid_with_byte_8(byte: u8) -> UUID {
        let mut bytes = [0u8; 16];
        bytes[8] = byte;
        UUID { bytes }
    }

    #[test]
    fn set_variant_ncs() {
        let mut uuid = make_uuid_with_byte_8(0xFF);
        uuid.set_variant(Variant::NCS);
        assert_eq!(uuid.get_variant(), Variant::NCS);
        assert_eq!(uuid.bytes[8] & 0x80, 0x00);
    }

    #[test]
    fn set_variant_osf() {
        let mut uuid = make_uuid_with_byte_8(0x00);
        uuid.set_variant(Variant::OSF);
        assert_eq!(uuid.get_variant(), Variant::OSF);
        assert_eq!(uuid.bytes[8] & 0xC0, 0x80);
    }

    #[test]
    fn set_variant_dcom() {
        let mut uuid = make_uuid_with_byte_8(0x3F);
        uuid.set_variant(Variant::DCOM);
        assert_eq!(uuid.get_variant(), Variant::DCOM);
        assert_eq!(uuid.bytes[8] & 0xE0, 0xC0);
    }

    #[test]
    fn set_variant_reserved() {
        let mut uuid = make_uuid_with_byte_8(0x00);
        uuid.set_variant(Variant::Reserved);
        assert_eq!(uuid.get_variant(), Variant::Reserved);
        assert_eq!(uuid.bytes[8] & 0xE0, 0xE0);
    }

    #[test]
    fn set_variant_preserves_other_bytes() {
        let original_bytes = [1, 2, 3, 4, 5, 6, 7, 8, 0xFF, 10, 11, 12, 13, 14, 15, 16];
        let mut uuid = UUID {
            bytes: original_bytes,
        };

        uuid.set_variant(Variant::OSF);

        for (i, &byte) in uuid.bytes.iter().enumerate() {
            if i != 8 {
                assert_eq!(byte, original_bytes[i]);
            }
        }
    }

    #[test]
    fn set_variant_can_change_variants() {
        let mut uuid = make_uuid_with_byte_8(0x00);

        uuid.set_variant(Variant::NCS);
        assert_eq!(uuid.get_variant(), Variant::NCS);

        uuid.set_variant(Variant::OSF);
        assert_eq!(uuid.get_variant(), Variant::OSF);

        uuid.set_variant(Variant::DCOM);
        assert_eq!(uuid.get_variant(), Variant::DCOM);

        uuid.set_variant(Variant::Reserved);
        assert_eq!(uuid.get_variant(), Variant::Reserved);
    }

    #[test]
    fn set_variant_idempotent() {
        let mut uuid = make_uuid_with_byte_8(0xA5);

        uuid.set_variant(Variant::OSF);
        let first_result = uuid.bytes[8];

        uuid.set_variant(Variant::OSF);
        let second_result = uuid.bytes[8];

        assert_eq!(first_result, second_result);
    }
}
