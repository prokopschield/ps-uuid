use crate::Variant;

impl Variant {
    #[must_use]
    pub const fn bitmask(self) -> u8 {
        match self {
            Self::NSC => 0x0F,
            Self::OSF => 0xBF,
            Self::DCOM => 0xDF,
            Self::Reserved => 0xFF,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Variant;

    #[test]
    fn test_variant_bitmask_returns_correct_value_for_each_variant() {
        // The `bitmask` method is an exhaustive match over the enum variants.
        // A comprehensive test must therefore verify the output for each
        // possible input variant.

        // 1. Test the NSC variant
        assert_eq!(
            Variant::NSC.bitmask(),
            0x0F,
            "The bitmask for Variant::NSC must be 0x0F"
        );

        // 2. Test the OSF variant
        assert_eq!(
            Variant::OSF.bitmask(),
            0xBF,
            "The bitmask for Variant::OSF must be 0xBF"
        );

        // 3. Test the DCOM variant
        assert_eq!(
            Variant::DCOM.bitmask(),
            0xDF,
            "The bitmask for Variant::DCOM must be 0xDF"
        );

        // 4. Test the Reserved variant
        assert_eq!(
            Variant::Reserved.bitmask(),
            0xFF,
            "The bitmask for Variant::Reserved must be 0xFF"
        );
    }
}
