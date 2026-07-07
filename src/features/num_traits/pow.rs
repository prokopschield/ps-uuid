//! Exponentiation trait implementations for UUID.
//!
//! Supports `Pow` with various exponent types:
//! - `u8`, `u16`, `u32` - Direct conversion to `u32` for `u128::wrapping_pow`
//! - `usize` - Uses `num_traits::pow` for arbitrary exponents
//!
//! All variants support owned and reference forms for both base and exponent.
//!
//! Every path wraps on overflow, matching the wrapping semantics of the `+`,
//! `-`, and `*` operators: the small-exponent path uses `u128::wrapping_pow`,
//! and the `usize` path composes the wrapping [`Mul`](core::ops::Mul) through
//! `num_traits::pow`. Both compute the true power reduced modulo 2¹²⁸.

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
                UUID::from_u128(self.to_u128().wrapping_pow(u32::from(rhs)))
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

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use num_traits::Pow;

    use crate::UUID;

    #[test]
    fn small_exponent_pow_wraps_on_overflow() {
        // 2^128 exceeds the 128-bit range and must wrap to zero rather than
        // panicking in debug builds.
        let base = UUID::from(2u128);
        assert_eq!(Pow::pow(base, 128u32), UUID::nil());
    }

    #[test]
    fn small_and_usize_exponent_paths_agree() {
        // The u32 path (u128::wrapping_pow) and the usize path (num_traits::pow
        // over the wrapping Mul) must produce identical results, including in
        // the overflow range.
        let base = UUID::from(0x1_0000_0001u128);

        for exp in [0usize, 1, 2, 7, 33, 200] {
            let e32 = u32::try_from(exp).expect("exponent fits in u32");

            let via_u32: UUID = Pow::pow(base, e32);
            let via_usize: UUID = Pow::pow(base, exp);

            assert_eq!(via_u32, via_usize, "paths disagree for exponent {exp}");
        }
    }
}
