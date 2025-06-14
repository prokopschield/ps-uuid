use crate::{Variant, UUID};

impl UUID {
    #[must_use]
    pub const fn with_variant(self, variant: Variant) -> Self {
        let mut uuid = self;

        uuid.bytes[8] &= variant.bitmask();
        uuid.bytes[8] |= variant.prefix();

        uuid
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
    fn with_variant_nsc() {
        let uuid = make_uuid_with_byte_8(0xFF);
        let result = uuid.with_variant(Variant::NSC);
        assert_eq!(result.variant(), Variant::NSC);
        assert_eq!(result.bytes[8] & 0x80, 0x00);
    }

    #[test]
    fn with_variant_osf() {
        let uuid = make_uuid_with_byte_8(0x00);
        let result = uuid.with_variant(Variant::OSF);
        assert_eq!(result.variant(), Variant::OSF);
        assert_eq!(result.bytes[8] & 0xC0, 0x80);
    }

    #[test]
    fn with_variant_dcom() {
        let uuid = make_uuid_with_byte_8(0x3F);
        let result = uuid.with_variant(Variant::DCOM);
        assert_eq!(result.variant(), Variant::DCOM);
        assert_eq!(result.bytes[8] & 0xE0, 0xC0);
    }

    #[test]
    fn with_variant_reserved() {
        let uuid = make_uuid_with_byte_8(0x00);
        let result = uuid.with_variant(Variant::Reserved);
        assert_eq!(result.variant(), Variant::Reserved);
        assert_eq!(result.bytes[8] & 0xE0, 0xE0);
    }

    #[test]
    fn with_variant_preserves_other_bytes() {
        let original_bytes = [1, 2, 3, 4, 5, 6, 7, 8, 0xFF, 10, 11, 12, 13, 14, 15, 16];
        let uuid = UUID {
            bytes: original_bytes,
        };

        let result = uuid.with_variant(Variant::OSF);

        for (i, &byte) in result.bytes.iter().enumerate() {
            if i != 8 {
                assert_eq!(byte, original_bytes[i]);
            }
        }
    }

    #[test]
    fn with_variant_leaves_original_unchanged() {
        let original_bytes = [1, 2, 3, 4, 5, 6, 7, 8, 0xFF, 10, 11, 12, 13, 14, 15, 16];
        let uuid = UUID {
            bytes: original_bytes,
        };

        let _result = uuid.with_variant(Variant::OSF);

        assert_eq!(uuid.bytes, original_bytes);
    }

    #[test]
    fn with_variant_can_chain_variants() {
        let uuid = make_uuid_with_byte_8(0x00);

        let result = uuid
            .with_variant(Variant::NSC)
            .with_variant(Variant::OSF)
            .with_variant(Variant::DCOM)
            .with_variant(Variant::Reserved);

        assert_eq!(result.variant(), Variant::Reserved);
    }

    #[test]
    fn with_variant_idempotent() {
        let uuid = make_uuid_with_byte_8(0xA5);

        let first_result = uuid.with_variant(Variant::OSF);
        let second_result = first_result.with_variant(Variant::OSF);

        assert_eq!(first_result.bytes[8], second_result.bytes[8]);
    }
}
