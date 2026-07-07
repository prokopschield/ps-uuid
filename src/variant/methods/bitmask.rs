use crate::Variant;

impl Variant {
    /// Returns the bitmask that preserves the payload bits of the variant byte,
    /// clearing the bits reserved for the variant itself.
    #[must_use]
    pub const fn bitmask(self) -> u8 {
        match self {
            // The NCS-compatible layout is identified by a cleared top bit in
            // the clock-seq high byte, matching `get_variant`, which classifies
            // any byte below 0x80 as NCS; the remaining seven bits are payload.
            Self::NCS => 0x7F,
            // RFC 4122 / 9562 UUIDs reserve the top two bits for the variant and
            // preserve the remaining six payload bits.
            Self::OSF => 0x3F,
            // Microsoft-compatible and future-reserved variants both encode their
            // variant in the top three bits, leaving five payload bits intact.
            Self::DCOM | Self::Reserved => 0x1F,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Variant;

    #[test]
    fn test_variant_bitmask_returns_correct_value_for_each_variant() {
        // The bitmask preserves data bits and clears all variant bits.
        assert_eq!(Variant::NCS.bitmask(), 0x7F); // bits 0-6 (variant is the top bit)
        assert_eq!(Variant::OSF.bitmask(), 0x3F); // bits 0-5
        assert_eq!(Variant::DCOM.bitmask(), 0x1F); // bits 0-4
        assert_eq!(Variant::Reserved.bitmask(), 0x1F); // bits 0-4
    }

    #[test]
    fn ncs_preserves_data_bits_for_all_inputs() {
        let bitmask = Variant::NCS.bitmask();
        let prefix = Variant::NCS.prefix();
        let data_mask = 0x7Fu8; // bits 0-6 (NCS reserves only the top bit)
        for original in 0u8..=255 {
            let result = (original & bitmask) | prefix;
            assert_eq!(
                original & data_mask,
                result & data_mask,
                "NCS: data bits not preserved for input 0x{original:02X}"
            );
            assert_eq!(
                result & 0x80,
                0x00,
                "NCS: variant bit incorrect for input 0x{original:02X}"
            );
        }
    }

    #[test]
    fn osf_preserves_data_bits_for_all_inputs() {
        let bitmask = Variant::OSF.bitmask();
        let prefix = Variant::OSF.prefix();
        let data_mask = 0x3Fu8; // bits 0-5
        for original in 0u8..=255 {
            let result = (original & bitmask) | prefix;
            assert_eq!(
                original & data_mask,
                result & data_mask,
                "OSF: data bits not preserved for input 0x{original:02X}"
            );
            assert_eq!(
                result & 0xC0,
                0x80,
                "OSF: variant bits incorrect for input 0x{original:02X}"
            );
        }
    }

    #[test]
    fn dcom_preserves_data_bits_for_all_inputs() {
        let bitmask = Variant::DCOM.bitmask();
        let prefix = Variant::DCOM.prefix();
        let data_mask = 0x1Fu8; // bits 0-4
        for original in 0u8..=255 {
            let result = (original & bitmask) | prefix;
            assert_eq!(
                original & data_mask,
                result & data_mask,
                "DCOM: data bits not preserved for input 0x{original:02X}"
            );
            assert_eq!(
                result & 0xE0,
                0xC0,
                "DCOM: variant bits incorrect for input 0x{original:02X}"
            );
        }
    }

    #[test]
    fn reserved_preserves_data_bits_for_all_inputs() {
        let bitmask = Variant::Reserved.bitmask();
        let prefix = Variant::Reserved.prefix();
        let data_mask = 0x1Fu8; // bits 0-4
        for original in 0u8..=255 {
            let result = (original & bitmask) | prefix;
            assert_eq!(
                original & data_mask,
                result & data_mask,
                "Reserved: data bits not preserved for input 0x{original:02X}"
            );
            assert_eq!(
                result & 0xE0,
                0xE0,
                "Reserved: variant bits incorrect for input 0x{original:02X}"
            );
        }
    }
}
