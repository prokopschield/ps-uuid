//! Bounded trait implementation for UUID.

use num_traits::Bounded;

use crate::UUID;

impl Bounded for UUID {
    #[inline]
    fn min_value() -> Self {
        Self::nil()
    }

    #[inline]
    fn max_value() -> Self {
        Self::max()
    }
}
