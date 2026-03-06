//! Shift amount trait for shift operations on UUID.
//!
//! This module defines the [`ShiftAmount`] trait which enables UUID shift operations
//! (`<<` and `>>`) to accept any integer type as the shift amount.
//!
//! # Semantics
//!
//! Shift amounts are converted to `u32` using truncating semantics. This is correct
//! because `u128::wrapping_shl/shr` internally masks the shift amount to 0..127 anyway.
//!
//! # Supported Types
//!
//! - Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
//! - Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
//! - References to any of the above
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! let uuid = UUID::from(1u128);
//!
//! // All integer types work as shift amounts
//! let _ = uuid << 4u8;
//! let _ = uuid << 4u16;
//! let _ = uuid << 4u32;
//! let _ = uuid << 4i32;
//! let _ = uuid << 4usize;
//!
//! // Reference types also work
//! let shift = 4u32;
//! let _ = uuid << &shift;
//! ```

/// Trait for types that can be used as shift amounts in UUID shift operations.
///
/// This trait enables generic shift implementations that accept any integer type.
/// The conversion to `u32` uses truncating semantics, which is correct because
/// `u128::wrapping_shl/shr` internally masks the shift amount anyway.
///
/// # Implementation Notes
///
/// The conversion casts directly to `u32`, which:
/// - Truncates values larger than `u32::MAX`
/// - Preserves the low 32 bits of signed values
///
/// Both behaviors are acceptable because the shift amount is masked to 0..127
/// by the underlying `wrapping_shl/shr` operations.
#[allow(clippy::wrong_self_convention)]
pub trait ShiftAmount: Copy {
    /// Converts the shift amount to `u32` for use with `wrapping_shl/shr`.
    fn as_u32(self) -> u32;
}

// ============================================================================
// Unsigned Integer Implementations
// ============================================================================

impl ShiftAmount for u8 {
    #[inline]
    fn as_u32(self) -> u32 {
        u32::from(self)
    }
}

impl ShiftAmount for &u8 {
    #[inline]
    fn as_u32(self) -> u32 {
        u32::from(*self)
    }
}

impl ShiftAmount for u16 {
    #[inline]
    fn as_u32(self) -> u32 {
        u32::from(self)
    }
}

impl ShiftAmount for &u16 {
    #[inline]
    fn as_u32(self) -> u32 {
        u32::from(*self)
    }
}

impl ShiftAmount for u32 {
    #[inline]
    fn as_u32(self) -> u32 {
        self
    }
}

impl ShiftAmount for &u32 {
    #[inline]
    fn as_u32(self) -> u32 {
        *self
    }
}

impl ShiftAmount for u64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &u64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

impl ShiftAmount for u128 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &u128 {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

impl ShiftAmount for usize {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &usize {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

// ============================================================================
// Signed Integer Implementations
// ============================================================================

impl ShiftAmount for i8 {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &i8 {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

impl ShiftAmount for i16 {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &i16 {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

impl ShiftAmount for i32 {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &i32 {
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

impl ShiftAmount for i64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &i64 {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

impl ShiftAmount for i128 {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &i128 {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

impl ShiftAmount for isize {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        self as u32
    }
}

impl ShiftAmount for &isize {
    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn as_u32(self) -> u32 {
        *self as u32
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Basic conversion tests
    // -------------------------------------------------------------------------

    #[test]
    fn u8_converts_correctly() {
        assert_eq!(0u8.as_u32(), 0);
        assert_eq!(4u8.as_u32(), 4);
        assert_eq!(u8::MAX.as_u32(), 255);
    }

    #[test]
    fn u16_converts_correctly() {
        assert_eq!(0u16.as_u32(), 0);
        assert_eq!(4u16.as_u32(), 4);
        assert_eq!(u16::MAX.as_u32(), 65535);
    }

    #[test]
    fn u32_identity() {
        assert_eq!(0u32.as_u32(), 0);
        assert_eq!(4u32.as_u32(), 4);
        assert_eq!(u32::MAX.as_u32(), u32::MAX);
    }

    #[test]
    fn u64_truncates() {
        assert_eq!(0u64.as_u32(), 0);
        assert_eq!(4u64.as_u32(), 4);
        // Large values truncate to low 32 bits
        assert_eq!((u64::from(u32::MAX) + 1).as_u32(), 0);
    }

    #[test]
    fn u128_truncates() {
        assert_eq!(0u128.as_u32(), 0);
        assert_eq!(4u128.as_u32(), 4);
        assert_eq!((u128::from(u32::MAX) + 1).as_u32(), 0);
    }

    #[test]
    fn usize_converts() {
        assert_eq!(0usize.as_u32(), 0);
        assert_eq!(4usize.as_u32(), 4);
        assert_eq!(127usize.as_u32(), 127);
    }

    // -------------------------------------------------------------------------
    // Signed type tests
    // -------------------------------------------------------------------------

    #[test]
    fn i8_converts() {
        assert_eq!(0i8.as_u32(), 0);
        assert_eq!(4i8.as_u32(), 4);
        assert_eq!(i8::MAX.as_u32(), 127);
        // Negative values wrap (but this is fine - shift amount is masked anyway)
        assert_eq!((-1i8).as_u32(), u32::MAX);
    }

    #[test]
    fn i16_converts() {
        assert_eq!(0i16.as_u32(), 0);
        assert_eq!(4i16.as_u32(), 4);
    }

    #[test]
    fn i32_converts() {
        assert_eq!(0i32.as_u32(), 0);
        assert_eq!(4i32.as_u32(), 4);
        assert_eq!(i32::MAX.as_u32(), i32::MAX as u32);
    }

    #[test]
    fn i64_converts() {
        assert_eq!(0i64.as_u32(), 0);
        assert_eq!(4i64.as_u32(), 4);
    }

    #[test]
    fn i128_converts() {
        assert_eq!(0i128.as_u32(), 0);
        assert_eq!(4i128.as_u32(), 4);
    }

    #[test]
    fn isize_converts() {
        assert_eq!(0isize.as_u32(), 0);
        assert_eq!(4isize.as_u32(), 4);
    }

    // -------------------------------------------------------------------------
    // Reference variant tests
    // -------------------------------------------------------------------------

    #[test]
    fn reference_variants_match_owned() {
        let u = 42u32;
        assert_eq!(u.as_u32(), (&u).as_u32());

        let i = 42i32;
        assert_eq!(i.as_u32(), (&i).as_u32());

        let big = 42u64;
        assert_eq!(big.as_u32(), (&big).as_u32());
    }

    // -------------------------------------------------------------------------
    // Practical shift amount range tests
    // -------------------------------------------------------------------------

    #[test]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn common_shift_amounts() {
        // These are the most commonly used shift amounts
        for shift in 0u32..=127 {
            assert_eq!(shift.as_u32(), shift);
            assert_eq!((shift as u8).as_u32(), shift);
            assert_eq!((shift as i32).as_u32(), shift);
        }
    }

    #[test]
    fn shift_amounts_beyond_127_work() {
        // Shift amounts > 127 are masked by wrapping_shl/shr anyway
        // but the conversion should still work
        assert_eq!(128u32.as_u32(), 128);
        assert_eq!(256u32.as_u32(), 256);
    }
}
