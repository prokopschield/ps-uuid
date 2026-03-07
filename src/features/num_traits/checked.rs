//! Checked arithmetic trait implementations for UUID.
//!
//! These operations return `None` on overflow/underflow instead of panicking
//! or wrapping.

use num_traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub,
};

use crate::UUID;

impl CheckedAdd for UUID {
    #[inline]
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.to_u128().checked_add(v.to_u128()).map(Self::from_u128)
    }
}

impl CheckedSub for UUID {
    #[inline]
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        self.to_u128().checked_sub(v.to_u128()).map(Self::from_u128)
    }
}

impl CheckedMul for UUID {
    #[inline]
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        self.to_u128().checked_mul(v.to_u128()).map(Self::from_u128)
    }
}

impl CheckedDiv for UUID {
    #[inline]
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.to_u128().checked_div(v.to_u128()).map(Self::from_u128)
    }
}

impl CheckedRem for UUID {
    #[inline]
    fn checked_rem(&self, v: &Self) -> Option<Self> {
        self.to_u128().checked_rem(v.to_u128()).map(Self::from_u128)
    }
}

impl CheckedNeg for UUID {
    #[inline]
    fn checked_neg(&self) -> Option<Self> {
        self.to_u128().checked_neg().map(Self::from_u128)
    }
}

impl CheckedShl for UUID {
    #[inline]
    fn checked_shl(&self, rhs: u32) -> Option<Self> {
        self.to_u128().checked_shl(rhs).map(Self::from_u128)
    }
}

impl CheckedShr for UUID {
    #[inline]
    fn checked_shr(&self, rhs: u32) -> Option<Self> {
        self.to_u128().checked_shr(rhs).map(Self::from_u128)
    }
}
