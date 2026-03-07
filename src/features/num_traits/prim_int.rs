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

    #[inline]
    fn signed_shl(self, n: u32) -> Self {
        Self::from_u128((self.to_u128().cast_signed() << n).cast_unsigned())
    }

    #[inline]
    fn signed_shr(self, n: u32) -> Self {
        Self::from_u128((self.to_u128().cast_signed() >> n).cast_unsigned())
    }

    #[inline]
    fn unsigned_shl(self, n: u32) -> Self {
        Self::from_u128(self.to_u128() << n)
    }

    #[inline]
    fn unsigned_shr(self, n: u32) -> Self {
        Self::from_u128(self.to_u128() >> n)
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

    #[inline]
    fn pow(self, exp: u32) -> Self {
        Self::from_u128(self.to_u128().pow(exp))
    }
}
