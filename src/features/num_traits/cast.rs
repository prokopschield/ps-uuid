//! Type conversion trait implementations for UUID.
//!
//! Provides implementations for:
//! - [`ToPrimitive`] - Fallible conversion to primitive types
//! - [`FromPrimitive`] - Fallible conversion from primitive types
//! - [`NumCast`] - Generic numeric casting
//! - [`AsPrimitive`] - Unchecked casting with `as` semantics

use num_traits::{AsPrimitive, FromPrimitive, ToPrimitive};

use crate::UUID;

// ============================================================================
// ToPrimitive - Convert UUID to primitive types
// ============================================================================

impl ToPrimitive for UUID {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.to_u128().to_i64()
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.to_u128().to_i128()
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.to_u128().to_u64()
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        Some(self.to_u128())
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.to_u128().to_f32()
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        self.to_u128().to_f64()
    }
}

// ============================================================================
// FromPrimitive - Convert primitive types to UUID
// ============================================================================

impl FromPrimitive for UUID {
    #[inline]
    fn from_i64(n: i64) -> Option<Self> {
        <u128 as FromPrimitive>::from_i64(n).map(Self::from_u128)
    }

    #[inline]
    fn from_i128(n: i128) -> Option<Self> {
        <u128 as FromPrimitive>::from_i128(n).map(Self::from_u128)
    }

    #[inline]
    fn from_u64(n: u64) -> Option<Self> {
        Some(Self::from_u128(u128::from(n)))
    }

    #[inline]
    fn from_u128(n: u128) -> Option<Self> {
        Some(Self::from_u128(n))
    }

    #[inline]
    fn from_f32(n: f32) -> Option<Self> {
        <u128 as FromPrimitive>::from_f32(n).map(Self::from_u128)
    }

    #[inline]
    fn from_f64(n: f64) -> Option<Self> {
        <u128 as FromPrimitive>::from_f64(n).map(Self::from_u128)
    }
}

// ============================================================================
// NumCast - Generic numeric casting
// ============================================================================

impl num_traits::NumCast for UUID {
    #[inline]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        n.to_u128().map(Self::from_u128)
    }
}

// ============================================================================
// AsPrimitive - Unchecked casting with `as` semantics
// ============================================================================

macro_rules! impl_as_primitive_for_uuid {
    ($($target:ty),* $(,)?) => {
        $(
            impl AsPrimitive<$target> for UUID {
                #[inline]
                #[allow(
                    clippy::cast_possible_truncation,
                    clippy::cast_possible_wrap,
                    clippy::cast_precision_loss
                )]
                fn as_(self) -> $target {
                    self.to_u128() as $target
                }
            }
        )*
    };
}

impl_as_primitive_for_uuid!(u8, u16, u32, u64, u128, usize);
impl_as_primitive_for_uuid!(i8, i16, i32, i64, i128, isize);
impl_as_primitive_for_uuid!(f32, f64);

impl AsPrimitive<Self> for UUID {
    #[inline]
    fn as_(self) -> Self {
        self
    }
}
