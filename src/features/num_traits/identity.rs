//! Identity trait implementations (`Zero`, `One`) for UUID.

use num_traits::{ConstOne, ConstZero, One, Zero};

use crate::{UUID, UUID_BYTES};

/// The multiplicative identity (1) as a UUID.
const ONE: UUID = UUID::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

impl Zero for UUID {
    #[inline]
    fn zero() -> Self {
        Self::nil()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self.as_bytes() == [0; UUID_BYTES]
    }
}

impl ConstZero for UUID {
    const ZERO: Self = Self::nil();
}

impl One for UUID {
    #[inline]
    fn one() -> Self {
        ONE
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.to_u128() == 1
    }
}

impl ConstOne for UUID {
    const ONE: Self = ONE;
}
