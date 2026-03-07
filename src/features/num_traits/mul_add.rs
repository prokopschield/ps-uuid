//! Fused multiply-add trait implementations for UUID.
//!
//! Computes `(self * a) + b` in a single operation with wrapping semantics.

use num_traits::{MulAdd, MulAddAssign};

use crate::UUID;

impl MulAdd for UUID {
    type Output = Self;

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        // Compute directly to avoid intermediate UUID conversions
        Self::from_u128(
            self.to_u128()
                .wrapping_mul(a.to_u128())
                .wrapping_add(b.to_u128()),
        )
    }
}

impl MulAddAssign for UUID {
    #[inline]
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        *self = self.mul_add(a, b);
    }
}

impl MulAddAssign<&Self, &Self> for UUID {
    #[inline]
    fn mul_add_assign(&mut self, a: &Self, b: &Self) {
        *self = self.mul_add(*a, *b);
    }
}
