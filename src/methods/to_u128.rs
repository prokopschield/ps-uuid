use crate::UUID;

impl UUID {
    /// Returns the UUID as a `u128` integer in big-endian byte order.
    ///
    /// This is a `const fn` equivalent of `u128::from(uuid)`.
    #[must_use]
    pub const fn to_u128(&self) -> u128 {
        u128::from_be_bytes(self.bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::UUID;

    #[test]
    fn nil_returns_zero() {
        assert_eq!(UUID::nil().to_u128(), 0);
    }

    #[test]
    fn max_returns_u128_max() {
        assert_eq!(UUID::max().to_u128(), u128::MAX);
    }

    #[test]
    fn roundtrip_with_from_u128() {
        let value: u128 = 0x0123_4567_89ab_cdef_0123_4567_89ab_cdef;
        let uuid = UUID::from_u128(value);
        assert_eq!(uuid.to_u128(), value);
    }

    #[test]
    fn const_context() {
        const VALUE: u128 = UUID::nil().to_u128();
        assert_eq!(VALUE, 0);
    }
}
