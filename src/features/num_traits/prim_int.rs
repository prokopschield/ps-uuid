//! Primitive integer trait implementation for UUID.
//!
//! `PrimInt` provides bit manipulation operations typically found on
//! primitive integer types.

use num_traits::PrimInt;

use crate::UUID;

impl PrimInt for UUID {
    #[inline]
    fn count_ones(self) -> u32 {
        self.to_u128().count_ones()
    }

    #[inline]
    fn count_zeros(self) -> u32 {
        self.to_u128().count_zeros()
    }

    #[inline]
    fn leading_ones(self) -> u32 {
        self.to_u128().leading_ones()
    }

    #[inline]
    fn leading_zeros(self) -> u32 {
        self.to_u128().leading_zeros()
    }

    #[inline]
    fn trailing_ones(self) -> u32 {
        self.to_u128().trailing_ones()
    }

    #[inline]
    fn trailing_zeros(self) -> u32 {
        self.to_u128().trailing_zeros()
    }

    #[inline]
    fn rotate_left(self, n: u32) -> Self {
        Self::from_u128(self.to_u128().rotate_left(n))
    }

    #[inline]
    fn rotate_right(self, n: u32) -> Self {
        Self::from_u128(self.to_u128().rotate_right(n))
    }

    /// Shifts the bits to the left by `n`, masking the shift amount modulo
    /// 128. Left shifts are identical for signed and unsigned operands, so
    /// this is equivalent to [`unsigned_shl`](PrimInt::unsigned_shl).
    #[inline]
    fn signed_shl(self, n: u32) -> Self {
        Self::from_u128(self.to_u128().wrapping_shl(n))
    }

    /// Shifts the bits to the right by `n`, sign-extending from bit 127
    /// (arithmetic shift) and masking the shift amount modulo 128, consistent
    /// with the `>>` operator's wrapping semantics.
    #[inline]
    fn signed_shr(self, n: u32) -> Self {
        Self::from_u128(self.to_u128().cast_signed().wrapping_shr(n).cast_unsigned())
    }

    /// Shifts the bits to the left by `n`, masking the shift amount modulo
    /// 128, consistent with the `<<` operator and
    /// [`WrappingShl`](num_traits::WrappingShl).
    #[inline]
    fn unsigned_shl(self, n: u32) -> Self {
        Self::from_u128(self.to_u128().wrapping_shl(n))
    }

    /// Shifts the bits to the right by `n`, filling zeros in the most
    /// significant bits and masking the shift amount modulo 128, consistent
    /// with the `>>` operator and [`WrappingShr`](num_traits::WrappingShr).
    #[inline]
    fn unsigned_shr(self, n: u32) -> Self {
        Self::from_u128(self.to_u128().wrapping_shr(n))
    }

    #[inline]
    fn swap_bytes(self) -> Self {
        Self::from_u128(self.to_u128().swap_bytes())
    }

    #[inline]
    fn reverse_bits(self) -> Self {
        Self::from_u128(self.to_u128().reverse_bits())
    }

    #[inline]
    fn from_be(x: Self) -> Self {
        Self::from_u128(u128::from_be(x.to_u128()))
    }

    #[inline]
    fn from_le(x: Self) -> Self {
        Self::from_u128(u128::from_le(x.to_u128()))
    }

    #[inline]
    fn to_be(self) -> Self {
        Self::from_u128(self.to_u128().to_be())
    }

    #[inline]
    fn to_le(self) -> Self {
        Self::from_u128(self.to_u128().to_le())
    }

    /// Raises `self` to the power of `exp`, wrapping modulo 2¹²⁸ on
    /// overflow, consistent with the [`Pow`](num_traits::Pow)
    /// implementations and the `+`, `-`, and `*` operators.
    #[inline]
    fn pow(self, exp: u32) -> Self {
        Self::from_u128(self.to_u128().wrapping_pow(exp))
    }
}

#[cfg(test)]
mod tests {
    use num_traits::{Pow, PrimInt};

    use crate::UUID;

    #[test]
    fn pow_wraps_on_overflow() {
        // 2^128 exceeds the 128-bit range and must wrap to zero rather than
        // panicking in overflow-checked builds.
        let base = UUID::from(2u128);

        assert_eq!(PrimInt::pow(base, 128), UUID::nil());
    }

    #[test]
    fn shifts_agree_with_the_operators() {
        // The high bit is clear, so signed_shr also agrees with the logical
        // `>>` operator.
        let uuid = UUID::from(0x5EAD_BEEF_CAFE_BABE_1234_5678_9ABC_DEF0u128);

        for n in [0u32, 1, 64, 127, 128, 200] {
            assert_eq!(uuid.unsigned_shl(n), uuid << n, "unsigned_shl({n})");
            assert_eq!(uuid.unsigned_shr(n), uuid >> n, "unsigned_shr({n})");
            assert_eq!(uuid.signed_shl(n), uuid << n, "signed_shl({n})");
            assert_eq!(uuid.signed_shr(n), uuid >> n, "signed_shr({n})");
        }
    }

    #[test]
    fn signed_shl_agrees_with_unsigned_shl_on_high_bit_values() {
        let uuid = UUID::from(u128::MAX - 0xFF);

        for n in [0u32, 1, 127, 128, 200] {
            assert_eq!(uuid.signed_shl(n), uuid.unsigned_shl(n), "amount {n}");
        }
    }

    #[test]
    fn signed_shr_sign_extends() {
        let negative = UUID::from(1u128 << 127);

        assert_eq!(
            negative.signed_shr(127),
            UUID::max(),
            "An arithmetic shift must fill with the sign bit."
        );
        assert_eq!(
            negative.signed_shr(128),
            negative,
            "The shift amount is masked modulo 128."
        );
    }

    #[test]
    fn pow_agrees_with_the_pow_trait() {
        // PrimInt::pow and Pow::pow must produce identical results,
        // including in the overflow range.
        let base = UUID::from(0x1_0000_0001u128);

        for exp in [0u32, 1, 2, 7, 33, 128, 200] {
            let via_prim_int = PrimInt::pow(base, exp);
            let via_pow: UUID = Pow::pow(base, exp);

            assert_eq!(via_prim_int, via_pow, "paths disagree for exponent {exp}");
        }
    }
}
