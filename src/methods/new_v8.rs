use crate::UUID;

impl UUID {
    /// Build an RFC-4122 **Version 8** (custom) UUID from any value convertible to `u128`.
    ///
    /// The caller supplies the complete 128-bit payload (in host order).
    /// This function:
    /// - Converts the value to a `u128`
    /// - Serializes it to big-endian (network order)
    /// - Overwrites the version nibble (bits 48‥=51) with `0b1000`
    /// - Overwrites the variant bits (bits 64‥=65) with `0b10`
    /// - Leaves all other bits untouched
    ///
    /// # Example
    /// ```
    /// use ps_uuid::UUID;
    ///
    /// let payload: u128 = 0xDEAD_BEEF_DEAD_BEEF_DEAD_BEEF_DEAD_BEEF;
    /// let uuid = UUID::new_v8(payload);
    ///
    /// assert_eq!(uuid.get_version(), Some(8));
    /// ```
    #[must_use]
    pub fn new_v8<V: Into<u128>>(value: V) -> Self {
        Self::from_parts_v8(value.into().to_be_bytes())
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Variant;

    const fn ver(b: &[u8; 16]) -> u8 {
        b[6] >> 4
    }
    const fn var(b: &[u8; 16]) -> u8 {
        b[8] >> 6
    }

    #[test]
    fn version_and_variant_bits_are_fixed() {
        for payload in &[0u128, u128::MAX] {
            let uuid = UUID::new_v8(*payload);
            let b = uuid.as_bytes();
            assert_eq!(ver(b), 0b1000, "version must be 8");
            assert_eq!(var(b), 0b10, "variant must be RFC-4122");
        }
    }

    #[test]
    fn non_reserved_bits_are_preserved() {
        let mut bytes = [0u8; 16];
        for (i, item) in bytes.iter_mut().enumerate() {
            *item = u8::try_from(i).unwrap();
        }
        let payload = u128::from_be_bytes(bytes);

        let uuid = UUID::new_v8(payload);
        let out = uuid.as_bytes();

        for i in 0..16 {
            match i {
                6 => {
                    assert_eq!(out[6] & 0x0F, bytes[6] & 0x0F);
                    assert_eq!(out[6] >> 4, 0x8);
                }
                8 => {
                    assert_eq!(out[8] & 0x3F, bytes[8] & 0x3F);
                    assert_eq!(out[8] >> 6, 0b10);
                }
                _ => assert_eq!(out[i], bytes[i], "byte {i} changed"),
            }
        }
    }

    #[test]
    fn version_and_variant_helpers_report_correctly() {
        let uuid = UUID::new_v8(0u128);
        assert_eq!(uuid.get_version(), Some(8));
        assert_eq!(uuid.get_variant(), Variant::OSF);
    }
}
