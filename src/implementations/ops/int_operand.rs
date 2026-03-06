//! Integer operand trait for mixed-type UUID operations.
//!
//! This module defines the [`IntOperand`] trait which enables UUID to participate
//! in operations with all Rust integer types (`u8`, `u16`, `u32`, `u64`, `u128`,
//! `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`).
//!
//! # Conversion Semantics
//!
//! - **Unsigned types**: Zero-extended to `u128`
//! - **Signed types**: Sign-extended to `i128`, then reinterpreted as `u128`
//!   (matching Rust's `as` cast semantics for two's complement representation)
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // Unsigned operands zero-extend
//! let uuid = UUID::from(0u128);
//! assert_eq!(u128::from(uuid + 255u8), 255);
//!
//! // Signed negative operands sign-extend (all high bits become 1)
//! let uuid = UUID::from(0u128);
//! let result = uuid + (-1i8);  // -1i8 sign-extends to u128::MAX
//! assert_eq!(result, UUID::max());
//! ```

/// Trait for types that can be used as operands in UUID arithmetic and bitwise operations.
///
/// This trait enables operations like `UUID + u32`, `UUID & i64`, etc.
/// All Rust integer primitives implement this trait, both owned and by reference.
///
/// # Implementation Notes
///
/// - Unsigned types are zero-extended to `u128`
/// - Signed types are sign-extended to `i128`, then reinterpreted as `u128`
///   (matching Rust's `as` cast semantics)
///
/// # Examples
///
/// ```
/// use ps_uuid::UUID;
///
/// // Works with all integer types
/// let uuid = UUID::from(100u128);
/// let _ = uuid + 1u8;
/// let _ = uuid + 1u16;
/// let _ = uuid + 1u32;
/// let _ = uuid + 1u64;
/// let _ = uuid + 1u128;
/// let _ = uuid + 1usize;
/// let _ = uuid + 1i8;
/// let _ = uuid + 1i16;
/// let _ = uuid + 1i32;
/// let _ = uuid + 1i64;
/// let _ = uuid + 1i128;
/// let _ = uuid + 1isize;
/// ```
pub trait IntOperand: Copy {
    /// Converts the integer to `u128` for use in UUID operations.
    ///
    /// # Conversion Rules
    ///
    /// - Unsigned integers are zero-extended
    /// - Signed integers are sign-extended to `i128`, then cast to `u128`
    fn to_u128(self) -> u128;
}

// ============================================================================
// Unsigned Integer Implementations
// ============================================================================

impl IntOperand for u8 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(self)
    }
}

impl IntOperand for &u8 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(*self)
    }
}

impl IntOperand for u16 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(self)
    }
}

impl IntOperand for &u16 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(*self)
    }
}

impl IntOperand for u32 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(self)
    }
}

impl IntOperand for &u32 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(*self)
    }
}

impl IntOperand for u64 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(self)
    }
}

impl IntOperand for &u64 {
    #[inline]
    fn to_u128(self) -> u128 {
        u128::from(*self)
    }
}

impl IntOperand for u128 {
    #[inline]
    fn to_u128(self) -> u128 {
        self
    }
}

impl IntOperand for &u128 {
    #[inline]
    fn to_u128(self) -> u128 {
        *self
    }
}

impl IntOperand for usize {
    #[inline]
    #[allow(clippy::cast_lossless)]
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl IntOperand for &usize {
    #[inline]
    #[allow(clippy::cast_lossless)]
    fn to_u128(self) -> u128 {
        *self as u128
    }
}

// ============================================================================
// Signed Integer Implementations
// ============================================================================

impl IntOperand for i8 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(self).cast_unsigned()
    }
}

impl IntOperand for &i8 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(*self).cast_unsigned()
    }
}

impl IntOperand for i16 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(self).cast_unsigned()
    }
}

impl IntOperand for &i16 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(*self).cast_unsigned()
    }
}

impl IntOperand for i32 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(self).cast_unsigned()
    }
}

impl IntOperand for &i32 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(*self).cast_unsigned()
    }
}

impl IntOperand for i64 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(self).cast_unsigned()
    }
}

impl IntOperand for &i64 {
    #[inline]
    fn to_u128(self) -> u128 {
        i128::from(*self).cast_unsigned()
    }
}

impl IntOperand for i128 {
    #[inline]
    fn to_u128(self) -> u128 {
        self.cast_unsigned()
    }
}

impl IntOperand for &i128 {
    #[inline]
    fn to_u128(self) -> u128 {
        (*self).cast_unsigned()
    }
}

impl IntOperand for isize {
    #[inline]
    #[allow(clippy::cast_lossless)]
    fn to_u128(self) -> u128 {
        (self as i128).cast_unsigned()
    }
}

impl IntOperand for &isize {
    #[inline]
    #[allow(clippy::cast_lossless)]
    fn to_u128(self) -> u128 {
        (*self as i128).cast_unsigned()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Unsigned zero-extension tests
    // -------------------------------------------------------------------------

    #[test]
    fn u8_zero_extends() {
        assert_eq!(0u8.to_u128(), 0);
        assert_eq!(1u8.to_u128(), 1);
        assert_eq!(u8::MAX.to_u128(), 255);
    }

    #[test]
    fn u16_zero_extends() {
        assert_eq!(0u16.to_u128(), 0);
        assert_eq!(u16::MAX.to_u128(), 65535);
    }

    #[test]
    fn u32_zero_extends() {
        assert_eq!(0u32.to_u128(), 0);
        assert_eq!(u32::MAX.to_u128(), u128::from(u32::MAX));
    }

    #[test]
    fn u64_zero_extends() {
        assert_eq!(0u64.to_u128(), 0);
        assert_eq!(u64::MAX.to_u128(), u128::from(u64::MAX));
    }

    #[test]
    fn u128_identity() {
        assert_eq!(0u128.to_u128(), 0);
        assert_eq!(u128::MAX.to_u128(), u128::MAX);
        assert_eq!(0x0123_4567_89ab_cdef_u128.to_u128(), 0x0123_4567_89ab_cdef);
    }

    #[test]
    fn usize_zero_extends() {
        assert_eq!(0usize.to_u128(), 0);
        assert_eq!(1usize.to_u128(), 1);
    }

    // -------------------------------------------------------------------------
    // Signed sign-extension tests
    // -------------------------------------------------------------------------

    #[test]
    fn i8_positive_zero_extends() {
        assert_eq!(0i8.to_u128(), 0);
        assert_eq!(1i8.to_u128(), 1);
        assert_eq!(i8::MAX.to_u128(), 127);
    }

    #[test]
    fn i8_negative_sign_extends() {
        // -1i8 as i128 = -1, as u128 = u128::MAX
        assert_eq!((-1i8).to_u128(), u128::MAX);
        // -2i8 as i128 = -2, as u128 = u128::MAX - 1
        assert_eq!((-2i8).to_u128(), u128::MAX - 1);
        // i8::MIN = -128
        assert_eq!(i8::MIN.to_u128(), (-128i128).to_u128());
    }

    #[test]
    fn i16_positive_zero_extends() {
        assert_eq!(0i16.to_u128(), 0);
        assert_eq!(i16::MAX.to_u128(), 32767);
    }

    #[test]
    fn i16_negative_sign_extends() {
        assert_eq!((-1i16).to_u128(), u128::MAX);
        assert_eq!(i16::MIN.to_u128(), i128::from(i16::MIN).to_u128());
    }

    #[test]
    fn i32_positive_zero_extends() {
        assert_eq!(0i32.to_u128(), 0);
        assert_eq!(i32::MAX.to_u128(), (i32::MAX as u128).to_u128());
    }

    #[test]
    fn i32_negative_sign_extends() {
        assert_eq!((-1i32).to_u128(), u128::MAX);
        assert_eq!(i32::MIN.to_u128(), i128::from(i32::MIN).to_u128());
    }

    #[test]
    fn i64_positive_zero_extends() {
        assert_eq!(0i64.to_u128(), 0);
        assert_eq!(i64::MAX.to_u128(), (i64::MAX as u128).to_u128());
    }

    #[test]
    fn i64_negative_sign_extends() {
        assert_eq!((-1i64).to_u128(), u128::MAX);
        assert_eq!(i64::MIN.to_u128(), i128::from(i64::MIN).to_u128());
    }

    #[test]
    fn i128_positive() {
        assert_eq!(0i128.to_u128(), 0);
        assert_eq!(1i128.to_u128(), 1);
        assert_eq!(i128::MAX.to_u128(), (i128::MAX as u128).to_u128());
    }

    #[test]
    fn i128_negative() {
        assert_eq!((-1i128).to_u128(), u128::MAX);
        assert_eq!(i128::MIN.to_u128(), i128::MIN.to_u128());
    }

    #[test]
    fn isize_positive() {
        assert_eq!(0isize.to_u128(), 0);
        assert_eq!(1isize.to_u128(), 1);
    }

    #[test]
    fn isize_negative() {
        assert_eq!((-1isize).to_u128(), u128::MAX);
    }

    // -------------------------------------------------------------------------
    // Reference variant tests
    // -------------------------------------------------------------------------

    #[test]
    fn reference_variants_match_owned() {
        let u = 42u32;
        assert_eq!(u.to_u128(), (&u).to_u128());

        let i = -42i32;
        assert_eq!(i.to_u128(), (&i).to_u128());

        let big = u128::MAX;
        assert_eq!(big.to_u128(), (&big).to_u128());
    }

    // -------------------------------------------------------------------------
    // Sign extension bit pattern verification
    // -------------------------------------------------------------------------

    #[test]
    fn sign_extension_fills_all_high_bits() {
        // -1 in any signed type should produce u128::MAX (all 1s)
        assert_eq!((-1i8).to_u128(), u128::MAX);
        assert_eq!((-1i16).to_u128(), u128::MAX);
        assert_eq!((-1i32).to_u128(), u128::MAX);
        assert_eq!((-1i64).to_u128(), u128::MAX);
        assert_eq!((-1i128).to_u128(), u128::MAX);
        assert_eq!((-1isize).to_u128(), u128::MAX);
    }

    #[test]
    fn sign_extension_preserves_two_complement_identity() {
        // For any signed value v, we should have:
        // v as i128 as u128 == v.to_u128()
        for v in [i8::MIN, -42i8, -1i8, 0i8, 1i8, 42i8, i8::MAX] {
            assert_eq!(v.to_u128(), i128::from(v).to_u128());
        }
    }
}
