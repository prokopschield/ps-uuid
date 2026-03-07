//! Overflowing arithmetic trait implementations for UUID.
//!
//! These operations return both the wrapped result and a boolean indicating
//! whether overflow occurred.

use num_traits::ops::overflowing::{OverflowingAdd, OverflowingMul, OverflowingSub};

use crate::UUID;

impl OverflowingAdd for UUID {
    #[inline]
    fn overflowing_add(&self, v: &Self) -> (Self, bool) {
        let (value, overflowed) = self.to_u128().overflowing_add(v.to_u128());
        (Self::from_u128(value), overflowed)
    }
}

impl OverflowingSub for UUID {
    #[inline]
    fn overflowing_sub(&self, v: &Self) -> (Self, bool) {
        let (value, overflowed) = self.to_u128().overflowing_sub(v.to_u128());
        (Self::from_u128(value), overflowed)
    }
}

impl OverflowingMul for UUID {
    #[inline]
    fn overflowing_mul(&self, v: &Self) -> (Self, bool) {
        let (value, overflowed) = self.to_u128().overflowing_mul(v.to_u128());
        (Self::from_u128(value), overflowed)
    }
}
