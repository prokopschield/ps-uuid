//! Core numeric trait implementations (`Num`, `Unsigned`) for UUID.

use core::num::ParseIntError;

use num_traits::{Num, Unsigned};

use crate::UUID;

impl Num for UUID {
    type FromStrRadixErr = ParseIntError;

    #[inline]
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        u128::from_str_radix(src, radix).map(Self::from_u128)
    }
}

/// Marker trait indicating UUID is an unsigned type.
impl Unsigned for UUID {}
