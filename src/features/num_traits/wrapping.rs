//! Wrapping arithmetic trait implementations for UUID.
//!
//! These operations wrap around on overflow, matching the behavior of
//! standard integer arithmetic in release mode.

use num_traits::{WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub};

use crate::UUID;

impl WrappingAdd for UUID {
    #[inline]
    fn wrapping_add(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().wrapping_add(v.to_u128()))
    }
}

impl WrappingSub for UUID {
    #[inline]
    fn wrapping_sub(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().wrapping_sub(v.to_u128()))
    }
}

impl WrappingMul for UUID {
    #[inline]
    fn wrapping_mul(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().wrapping_mul(v.to_u128()))
    }
}

impl WrappingNeg for UUID {
    #[inline]
    fn wrapping_neg(&self) -> Self {
        Self::from_u128(self.to_u128().wrapping_neg())
    }
}

impl WrappingShl for UUID {
    #[inline]
    fn wrapping_shl(&self, rhs: u32) -> Self {
        Self::from_u128(self.to_u128().wrapping_shl(rhs))
    }
}

impl WrappingShr for UUID {
    #[inline]
    fn wrapping_shr(&self, rhs: u32) -> Self {
        Self::from_u128(self.to_u128().wrapping_shr(rhs))
    }
}
