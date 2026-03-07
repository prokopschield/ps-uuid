//! Euclidean division trait implementations for UUID.
//!
//! Euclidean division differs from standard division in that the remainder
//! is always non-negative. For unsigned types like UUID, this is identical
//! to standard division.
//!
//! # Panics
//!
//! [`Euclid::div_euclid`] and [`Euclid::rem_euclid`] panic if the divisor is
//! zero. Use [`CheckedEuclid`] for a non-panicking alternative that returns
//! `None` on division by zero.

use num_traits::ops::euclid::CheckedEuclid;
use num_traits::Euclid;

use crate::UUID;

impl Euclid for UUID {
    #[inline]
    fn div_euclid(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().div_euclid(v.to_u128()))
    }

    #[inline]
    fn rem_euclid(&self, v: &Self) -> Self {
        Self::from_u128(self.to_u128().rem_euclid(v.to_u128()))
    }
}

impl CheckedEuclid for UUID {
    #[inline]
    fn checked_div_euclid(&self, v: &Self) -> Option<Self> {
        self.to_u128()
            .checked_div_euclid(v.to_u128())
            .map(Self::from_u128)
    }

    #[inline]
    fn checked_rem_euclid(&self, v: &Self) -> Option<Self> {
        self.to_u128()
            .checked_rem_euclid(v.to_u128())
            .map(Self::from_u128)
    }
}
