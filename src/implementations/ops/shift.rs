//! Shift operator implementations for UUID.
//!
//! This module provides implementations for:
//!
//! | Operator | Trait | Semantics |
//! |----------|-------|-----------|
//! | `<<` | [`Shl`] | Wrapping left shift |
//! | `>>` | [`Shr`] | Wrapping right shift |
//!
//! # Wrapping Semantics
//!
//! Shift operations use **wrapping semantics**: the shift amount is masked to
//! the valid range (0..128), matching the behavior of `u128::wrapping_shl/shr`.
//! This means:
//!
//! - `uuid << 128` is equivalent to `uuid << 0` (shift amount is masked)
//! - `uuid << 256` is equivalent to `uuid << 0`
//! - Negative shift amounts are converted to their unsigned equivalent
//!
//! # Shift Amount Types
//!
//! Shift operations accept any integer type as the shift amount:
//!
//! - Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
//! - Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
//! - References to any of the above
//!
//! # Supported Operand Combinations
//!
//! For each shift amount type `T`:
//!
//! - `UUID << T` and `UUID >> T`
//! - `&UUID << T` and `&UUID >> T`
//! - `UUID <<= T` and `UUID >>= T`
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // Left shift - multiply by powers of 2
//! let uuid = UUID::from(1u128);
//! assert_eq!(u128::from(uuid << 4u32), 16);  // 1 * 2^4 = 16
//! assert_eq!(u128::from(uuid << 8u32), 256); // 1 * 2^8 = 256
//!
//! // Right shift - divide by powers of 2
//! let uuid = UUID::from(256u128);
//! assert_eq!(u128::from(uuid >> 4u32), 16);  // 256 / 2^4 = 16
//! assert_eq!(u128::from(uuid >> 8u32), 1);   // 256 / 2^8 = 1
//!
//! // Different shift amount types
//! let uuid = UUID::from(1u128);
//! let _ = uuid << 4u8;
//! let _ = uuid << 4u16;
//! let _ = uuid << 4u32;
//! let _ = uuid << 4i32;
//! let _ = uuid << 4usize;
//!
//! // Compound assignment
//! let mut uuid = UUID::from(1u128);
//! uuid <<= 4u32;
//! assert_eq!(u128::from(uuid), 16);
//! ```

use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::UUID;

use super::shift_amount::ShiftAmount;

// ============================================================================
// Left Shift
// ============================================================================

impl<T: ShiftAmount> Shl<T> for UUID {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: T) -> Self::Output {
        Self::from_u128(self.to_u128().wrapping_shl(rhs.as_u32()))
    }
}

impl<T: ShiftAmount> Shl<T> for &UUID {
    type Output = UUID;

    #[inline]
    fn shl(self, rhs: T) -> Self::Output {
        UUID::from_u128(self.to_u128().wrapping_shl(rhs.as_u32()))
    }
}

impl<T: ShiftAmount> ShlAssign<T> for UUID {
    #[inline]
    fn shl_assign(&mut self, rhs: T) {
        *self = *self << rhs;
    }
}

// ============================================================================
// Right Shift
// ============================================================================

impl<T: ShiftAmount> Shr<T> for UUID {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: T) -> Self::Output {
        Self::from_u128(self.to_u128().wrapping_shr(rhs.as_u32()))
    }
}

impl<T: ShiftAmount> Shr<T> for &UUID {
    type Output = UUID;

    #[inline]
    fn shr(self, rhs: T) -> Self::Output {
        UUID::from_u128(self.to_u128().wrapping_shr(rhs.as_u32()))
    }
}

impl<T: ShiftAmount> ShrAssign<T> for UUID {
    #[inline]
    fn shr_assign(&mut self, rhs: T) {
        *self = *self >> rhs;
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    #![allow(clippy::op_ref)]

    use super::*;

    // -------------------------------------------------------------------------
    // Left shift basic tests
    // -------------------------------------------------------------------------

    #[test]
    fn shl_by_zero() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid << 0u32, uuid);
    }

    #[test]
    fn shl_by_one() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 1u32), 2);
    }

    #[test]
    fn shl_by_four() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4u32), 16);
    }

    #[test]
    fn shl_by_eight() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 8u32), 256);
    }

    #[test]
    fn shl_pattern() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(u128::from(uuid << 4u32), 0b1010_1010_0000);
    }

    // -------------------------------------------------------------------------
    // Right shift basic tests
    // -------------------------------------------------------------------------

    #[test]
    fn shr_by_zero() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid >> 0u32, uuid);
    }

    #[test]
    fn shr_by_one() {
        let uuid = UUID::from(2u128);
        assert_eq!(u128::from(uuid >> 1u32), 1);
    }

    #[test]
    fn shr_by_four() {
        let uuid = UUID::from(256u128);
        assert_eq!(u128::from(uuid >> 4u32), 16);
    }

    #[test]
    fn shr_by_eight() {
        let uuid = UUID::from(256u128);
        assert_eq!(u128::from(uuid >> 8u32), 1);
    }

    #[test]
    fn shr_pattern() {
        let uuid = UUID::from(0b1010_1010_0000u128);
        assert_eq!(u128::from(uuid >> 4u32), 0b1010_1010);
    }

    // -------------------------------------------------------------------------
    // Shift amount type tests
    // -------------------------------------------------------------------------

    #[test]
    fn shl_with_u8() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4u8), 16);
    }

    #[test]
    fn shl_with_u16() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4u16), 16);
    }

    #[test]
    fn shl_with_u32() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4u32), 16);
    }

    #[test]
    fn shl_with_u64() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4u64), 16);
    }

    #[test]
    fn shl_with_u128() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4u128), 16);
    }

    #[test]
    fn shl_with_usize() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4usize), 16);
    }

    #[test]
    fn shl_with_i8() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4i8), 16);
    }

    #[test]
    fn shl_with_i16() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4i16), 16);
    }

    #[test]
    fn shl_with_i32() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4i32), 16);
    }

    #[test]
    fn shl_with_i64() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4i64), 16);
    }

    #[test]
    fn shl_with_i128() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4i128), 16);
    }

    #[test]
    fn shl_with_isize() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid << 4isize), 16);
    }

    // -------------------------------------------------------------------------
    // Reference variant tests
    // -------------------------------------------------------------------------

    #[test]
    fn shl_ref_uuid() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(&uuid << 4u32), 16);
    }

    #[test]
    fn shr_ref_uuid() {
        let uuid = UUID::from(16u128);
        assert_eq!(u128::from(&uuid >> 4u32), 1);
    }

    #[test]
    fn shl_with_ref_amount() {
        let uuid = UUID::from(1u128);
        let shift = 4u32;
        assert_eq!(u128::from(uuid << &shift), 16);
    }

    #[test]
    fn shl_ref_uuid_ref_amount() {
        let uuid = UUID::from(1u128);
        let shift = 4u32;
        assert_eq!(u128::from(&uuid << &shift), 16);
    }

    // -------------------------------------------------------------------------
    // Compound assignment tests
    // -------------------------------------------------------------------------

    #[test]
    fn shl_assign() {
        let mut uuid = UUID::from(1u128);
        uuid <<= 4u32;
        assert_eq!(u128::from(uuid), 16);
    }

    #[test]
    fn shl_assign_ref_amount() {
        let mut uuid = UUID::from(1u128);
        let shift = 4u32;
        uuid <<= &shift;
        assert_eq!(u128::from(uuid), 16);
    }

    #[test]
    fn shr_assign() {
        let mut uuid = UUID::from(16u128);
        uuid >>= 4u32;
        assert_eq!(u128::from(uuid), 1);
    }

    #[test]
    fn shr_assign_ref_amount() {
        let mut uuid = UUID::from(16u128);
        let shift = 4u32;
        uuid >>= &shift;
        assert_eq!(u128::from(uuid), 1);
    }

    // -------------------------------------------------------------------------
    // Wrapping semantics tests
    // -------------------------------------------------------------------------

    #[test]
    fn shl_by_128_wraps() {
        let uuid = UUID::from(0b1010_1010u128);
        // Shift by 128 is masked to 0, so result is unchanged
        assert_eq!(uuid << 128u32, uuid);
    }

    #[test]
    fn shl_by_127() {
        let uuid = UUID::from(1u128);
        // 1 << 127 = 0x8000...0000 (high bit set)
        assert_eq!(u128::from(uuid << 127u32), 1u128 << 127);
    }

    #[test]
    fn shr_by_128_wraps() {
        let uuid = UUID::from(0b1010_1010u128);
        // Shift by 128 is masked to 0, so result is unchanged
        assert_eq!(uuid >> 128u32, uuid);
    }

    #[test]
    fn shr_by_127() {
        let uuid = UUID::from(1u128 << 127);
        assert_eq!(u128::from(uuid >> 127u32), 1);
    }

    // -------------------------------------------------------------------------
    // Shift out / shift in tests
    // -------------------------------------------------------------------------

    #[test]
    fn shl_shifts_out_high_bits() {
        let uuid = UUID::max();
        // Shifting left loses high bits
        assert_eq!(u128::from(uuid << 1u32), u128::MAX << 1);
    }

    #[test]
    fn shr_shifts_in_zeros() {
        let uuid = UUID::max();
        // Shifting right brings in zeros
        assert_eq!(u128::from(uuid >> 1u32), u128::MAX >> 1);
    }

    #[test]
    fn shl_shr_roundtrip_when_no_loss() {
        let uuid = UUID::from(0b1010_1010u128);
        // Shift left then right by same amount (no bits lost)
        assert_eq!((uuid << 4u32) >> 4u32, uuid);
    }

    #[test]
    fn shr_shl_not_roundtrip_when_bits_lost() {
        let uuid = UUID::from(0b1010_1010u128);
        // Shift right loses low bits, shift left doesn't restore them
        assert_eq!((uuid >> 4u32) << 4u32, UUID::from(0b1010_0000u128));
    }

    // -------------------------------------------------------------------------
    // Boundary tests
    // -------------------------------------------------------------------------

    #[test]
    fn shl_nil() {
        // Shifting zero produces zero
        assert_eq!(UUID::nil() << 64u32, UUID::nil());
    }

    #[test]
    fn shr_nil() {
        // Shifting zero produces zero
        assert_eq!(UUID::nil() >> 64u32, UUID::nil());
    }

    #[test]
    fn shl_max_by_any_nonzero() {
        // Shifting max left by any amount produces a different value
        // (unless masked to 0)
        let uuid = UUID::max();
        assert_ne!(uuid << 1u32, uuid);
    }

    #[test]
    fn shr_max_by_any_nonzero() {
        // Shifting max right by any amount produces a different value
        // (unless masked to 0)
        let uuid = UUID::max();
        assert_ne!(uuid >> 1u32, uuid);
    }

    // -------------------------------------------------------------------------
    // Negative shift amount tests (wrapping behavior)
    // -------------------------------------------------------------------------

    #[test]
    fn shl_negative_amount() {
        // Negative shift amounts wrap due to the as_u32 conversion
        // -1i32 as u32 = u32::MAX, which gets masked by wrapping_shl
        let uuid = UUID::from(1u128);
        // The exact result depends on how wrapping_shl masks large values
        let result = uuid << (-1i32);
        // Verify it doesn't panic and produces some result
        let _ = u128::from(result);
    }
}
