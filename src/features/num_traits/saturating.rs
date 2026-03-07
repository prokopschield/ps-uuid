//! Saturating arithmetic trait implementations for UUID.
//!
//! These operations clamp the result to the valid range instead of
//! overflowing or wrapping.

use num_traits::{Saturating, SaturatingAdd, SaturatingMul, SaturatingSub};

use crate::UUID;

impl Saturating for UUID {
    #[inline]
    fn saturating_add(self, v: Self) -> Self {
        Self::from_u128(self.to_u128().saturating_add(v.to_u128()))
    }

    #[inline]
    fn saturating_sub(self, v: Self) -> Self {
        Self::from_u128(self.to_u128().saturating_sub(v.to_u128()))
    }
}

impl SaturatingAdd for UUID {
    #[inline]
    fn saturating_add(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().saturating_add(v.to_u128()))
    }
}

impl SaturatingSub for UUID {
    #[inline]
    fn saturating_sub(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().saturating_sub(v.to_u128()))
    }
}

impl SaturatingMul for UUID {
    #[inline]
    fn saturating_mul(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().saturating_mul(v.to_u128()))
    }
}
