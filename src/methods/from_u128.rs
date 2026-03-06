use crate::UUID;

impl UUID {
    /// Creates a UUID from a `u128` integer in big-endian byte order.
    ///
    /// This is a `const fn` equivalent of `UUID::from(value)`.
    #[must_use]
    pub const fn from_u128(value: u128) -> Self {
        Self {
            bytes: value.to_be_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::UUID;

    #[test]
    fn zero_equals_nil() {
        assert_eq!(UUID::from_u128(0), UUID::nil());
    }

    #[test]
    fn max_equals_max() {
        assert_eq!(UUID::from_u128(u128::MAX), UUID::max());
    }

    #[test]
    fn roundtrip_with_to_u128() {
        let value: u128 = 0xfedc_ba98_7654_3210_fedc_ba98_7654_3210;
        let uuid = UUID::from_u128(value);
        assert_eq!(uuid.to_u128(), value);
    }

    #[test]
    fn const_context() {
        const UUID_CONST: UUID = UUID::from_u128(42);
        assert_eq!(UUID_CONST.to_u128(), 42);
    }
}
