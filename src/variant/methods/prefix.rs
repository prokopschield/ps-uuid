use crate::Variant;

impl Variant {
    #[must_use]
    pub const fn prefix(self) -> u8 {
        match self {
            Self::NSC => 0x00,
            Self::OSF => 0x80,
            Self::DCOM => 0xC0,
            Self::Reserved => 0xE0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Variant;

    #[test]
    fn nsc_prefix_is_zero() {
        assert_eq!(Variant::NSC.prefix(), 0x00);
    }

    #[test]
    fn osf_prefix_is_0x80() {
        assert_eq!(Variant::OSF.prefix(), 0x80);
    }

    #[test]
    fn dcom_prefix_is_0xc0() {
        assert_eq!(Variant::DCOM.prefix(), 0xC0);
    }

    #[test]
    fn reserved_prefix_is_0xe0() {
        assert_eq!(Variant::Reserved.prefix(), 0xE0);
    }

    #[test]
    fn prefix_values_are_distinct() {
        let prefixes = [
            Variant::NSC.prefix(),
            Variant::OSF.prefix(),
            Variant::DCOM.prefix(),
            Variant::Reserved.prefix(),
        ];

        for (i, &prefix_a) in prefixes.iter().enumerate() {
            for &prefix_b in prefixes.iter().skip(i + 1) {
                assert_ne!(prefix_a, prefix_b);
            }
        }
    }

    #[test]
    fn prefix_preserves_variant_bit_pattern() {
        // NSC: 0b0xxxxxxx
        assert_eq!(Variant::NSC.prefix() & 0x80, 0x00);

        // OSF: 0b10xxxxxx
        assert_eq!(Variant::OSF.prefix() & 0xC0, 0x80);

        // DCOM: 0b110xxxxx
        assert_eq!(Variant::DCOM.prefix() & 0xE0, 0xC0);

        // Reserved: 0b111xxxxx
        assert_eq!(Variant::Reserved.prefix() & 0xE0, 0xE0);
    }
}
