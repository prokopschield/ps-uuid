//! Exponentiation trait implementations for UUID.
//!
//! Supports `Pow` with various exponent types:
//! - `u8`, `u16`, `u32` - Direct conversion to `u32` for `u128::pow`
//! - `usize` - Uses `num_traits::pow` for arbitrary exponents
//!
//! All variants support owned and reference forms for both base and exponent.

use num_traits::Pow;

use crate::UUID;

// ============================================================================
// Pow with small integer exponents (u8, u16, u32)
// ============================================================================

macro_rules! impl_pow_integer_rhs {
    ($rhs:ty) => {
        impl Pow<$rhs> for UUID {
            type Output = Self;

            #[inline]
            fn pow(self, rhs: $rhs) -> Self::Output {
                UUID::from_u128(self.to_u128().pow(u32::from(rhs)))
            }
        }

        impl<'a> Pow<&'a $rhs> for UUID {
            type Output = Self;

            #[inline]
            fn pow(self, rhs: &'a $rhs) -> Self::Output {
                <Self as Pow<$rhs>>::pow(self, *rhs)
            }
        }

        impl<'a> Pow<$rhs> for &'a UUID {
            type Output = UUID;

            #[inline]
            fn pow(self, rhs: $rhs) -> Self::Output {
                <UUID as Pow<$rhs>>::pow(*self, rhs)
            }
        }

        impl<'a, 'b> Pow<&'a $rhs> for &'b UUID {
            type Output = UUID;

            #[inline]
            fn pow(self, rhs: &'a $rhs) -> Self::Output {
                <UUID as Pow<$rhs>>::pow(*self, *rhs)
            }
        }
    };
}

impl_pow_integer_rhs!(u8);
impl_pow_integer_rhs!(u16);
impl_pow_integer_rhs!(u32);

// ============================================================================
// Pow with usize exponent (uses num_traits::pow for large exponents)
// ============================================================================

impl Pow<usize> for UUID {
    type Output = Self;

    #[inline]
    fn pow(self, rhs: usize) -> Self::Output {
        num_traits::pow(self, rhs)
    }
}

impl<'a> Pow<&'a usize> for UUID {
    type Output = Self;

    #[inline]
    fn pow(self, rhs: &'a usize) -> Self::Output {
        <Self as Pow<usize>>::pow(self, *rhs)
    }
}

impl Pow<usize> for &UUID {
    type Output = UUID;

    #[inline]
    fn pow(self, rhs: usize) -> Self::Output {
        <UUID as Pow<usize>>::pow(*self, rhs)
    }
}

impl<'a> Pow<&'a usize> for &UUID {
    type Output = UUID;

    #[inline]
    fn pow(self, rhs: &'a usize) -> Self::Output {
        <UUID as Pow<usize>>::pow(*self, *rhs)
    }
}
