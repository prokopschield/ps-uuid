//! Multiplicative inverse trait implementation for UUID.
//!
//! Computes the modular multiplicative inverse under 2^128. The inverse of `x`
//! is the value `y` such that `x * y ≡ 1 (mod 2^128)`.
//!
//! Returns `None` for even values, as they share a common factor with 2^128
//! and therefore have no multiplicative inverse.

use num_traits::Inv;

use crate::UUID;

impl Inv for UUID {
    type Output = Option<Self>;

    fn inv(self) -> Option<Self> {
        // Hensel lifting (Newton's method) for modular multiplicative inverse
        // Only exists for odd values (gcd(x, 2^128) = 1)
        let x = self.to_u128();
        if x & 1 == 0 {
            return None;
        }

        // Newton's method: y = y * (2 - x*y), doubles correct bits each iteration
        let mut y = x; // x*x ≡ 1 (mod 8) for odd x, giving 3 correct bits
        y = y.wrapping_mul(2u128.wrapping_sub(x.wrapping_mul(y)));
        y = y.wrapping_mul(2u128.wrapping_sub(x.wrapping_mul(y)));
        y = y.wrapping_mul(2u128.wrapping_sub(x.wrapping_mul(y)));
        y = y.wrapping_mul(2u128.wrapping_sub(x.wrapping_mul(y)));
        y = y.wrapping_mul(2u128.wrapping_sub(x.wrapping_mul(y)));
        y = y.wrapping_mul(2u128.wrapping_sub(x.wrapping_mul(y))); // 6 iterations → 192 bits

        Some(Self::from_u128(y))
    }
}
