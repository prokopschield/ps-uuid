//! Bitwise operator implementations for UUID.
//!
//! This module provides implementations for:
//!
//! | Operator | Trait | Description |
//! |----------|-------|-------------|
//! | `&` | [`BitAnd`] | Bitwise AND |
//! | `\|` | [`BitOr`] | Bitwise OR |
//! | `^` | [`BitXor`] | Bitwise XOR |
//! | `!` | [`Not`] | Bitwise NOT (complement) |
//!
//! # Supported Operand Combinations
//!
//! Binary operators (`&`, `|`, `^`) support:
//!
//! - `UUID op UUID` and ref variants
//! - `UUID op T` where `T: IntOperand` (all integer types)
//! - `T op UUID` where `T: IntOperand`
//! - Compound assignment: `UUID op= UUID`, `UUID op= T`
//!
//! Unary NOT (`!`) supports:
//!
//! - `!UUID`
//! - `!&UUID`
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // Bitwise AND - useful for masking
//! let uuid = UUID::from(0b1010_1010u128);
//! let mask = UUID::from(0b1111_0000u128);
//! assert_eq!(u128::from(uuid & mask), 0b1010_0000);
//!
//! // Bitwise OR - useful for setting bits
//! let uuid = UUID::from(0b1010_0000u128);
//! let bits = 0b0000_1010u32;
//! assert_eq!(u128::from(uuid | bits), 0b1010_1010);
//!
//! // Bitwise XOR - useful for toggling bits
//! let uuid = UUID::from(0b1010_1010u128);
//! assert_eq!(u128::from(uuid ^ 0b1111_1111u32), 0b0101_0101);
//!
//! // Bitwise NOT - inverts all bits
//! let uuid = UUID::nil();
//! assert_eq!(!uuid, UUID::max());
//! assert_eq!(!UUID::max(), UUID::nil());
//! ```

use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use crate::UUID;

use super::int_operand::IntOperand;
use super::{
    impl_binop_uuid_uuid, impl_int_op_uuid_commutative, impl_ref_lhs_uuid, impl_uuid_op_int,
};

// ============================================================================
// UUID op UUID Implementations
// ============================================================================

impl_binop_uuid_uuid!(BitAnd, bitand, BitAndAssign, bitand_assign, u128::bitand);
impl_binop_uuid_uuid!(BitOr, bitor, BitOrAssign, bitor_assign, u128::bitor);
impl_binop_uuid_uuid!(BitXor, bitxor, BitXorAssign, bitxor_assign, u128::bitxor);

impl_ref_lhs_uuid!(BitAnd, bitand);
impl_ref_lhs_uuid!(BitOr, bitor);
impl_ref_lhs_uuid!(BitXor, bitxor);

// ============================================================================
// UUID op T Implementations (where T: IntOperand)
// ============================================================================

impl_uuid_op_int!(BitAnd, bitand, BitAndAssign, bitand_assign, u128::bitand);
impl_uuid_op_int!(BitOr, bitor, BitOrAssign, bitor_assign, u128::bitor);
impl_uuid_op_int!(BitXor, bitxor, BitXorAssign, bitxor_assign, u128::bitxor);

// ============================================================================
// T op UUID Implementations (where T: IntOperand)
// ============================================================================

// All bitwise operations are commutative
impl_int_op_uuid_commutative!(
    BitAnd, bitand, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
impl_int_op_uuid_commutative!(
    BitOr, bitor, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
impl_int_op_uuid_commutative!(
    BitXor, bitxor, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

// ============================================================================
// Bitwise NOT
// ============================================================================

impl Not for UUID {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self::from_u128(!self.to_u128())
    }
}

impl Not for &UUID {
    type Output = UUID;

    #[inline]
    fn not(self) -> Self::Output {
        UUID::from_u128(!self.to_u128())
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
    // Bitwise AND tests
    // -------------------------------------------------------------------------

    #[test]
    fn bitand_uuid_uuid() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(u128::from(a & b), 0b1000_1000);
    }

    #[test]
    fn bitand_uuid_ref_uuid() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(u128::from(a & &b), 0b1000_1000);
    }

    #[test]
    fn bitand_ref_uuid_uuid() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(u128::from(&a & b), 0b1000_1000);
    }

    #[test]
    fn bitand_ref_uuid_ref_uuid() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(u128::from(&a & &b), 0b1000_1000);
    }

    #[test]
    fn bitand_uuid_int() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(u128::from(uuid & 0b1100_1100u8), 0b1000_1000);
        assert_eq!(u128::from(uuid & 0b1100_1100u16), 0b1000_1000);
        assert_eq!(u128::from(uuid & 0b1100_1100u32), 0b1000_1000);
        assert_eq!(u128::from(uuid & 0b1100_1100u64), 0b1000_1000);
        assert_eq!(u128::from(uuid & 0b1100_1100u128), 0b1000_1000);
    }

    #[test]
    fn bitand_int_uuid() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(u128::from(0b1100_1100u32 & uuid), 0b1000_1000);
    }

    #[test]
    fn bitand_int_ref_uuid() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(u128::from(0b1100_1100u32 & &uuid), 0b1000_1000);
    }

    #[test]
    fn bitand_ref_int_uuid() {
        let uuid = UUID::from(0b1010_1010u128);
        let n = 0b1100_1100u32;
        assert_eq!(u128::from(&n & uuid), 0b1000_1000);
    }

    #[test]
    fn bitand_ref_int_ref_uuid() {
        let uuid = UUID::from(0b1010_1010u128);
        let n = 0b1100_1100u32;
        assert_eq!(u128::from(&n & &uuid), 0b1000_1000);
    }

    #[test]
    fn bitand_assign() {
        let mut uuid = UUID::from(0b1010_1010u128);
        uuid &= 0b1100_1100u32;
        assert_eq!(u128::from(uuid), 0b1000_1000);
    }

    #[test]
    fn bitand_assign_uuid() {
        let mut uuid = UUID::from(0b1010_1010u128);
        uuid &= UUID::from(0b1100_1100u128);
        assert_eq!(u128::from(uuid), 0b1000_1000);
    }

    #[test]
    #[allow(clippy::erasing_op)]
    fn bitand_with_zero() {
        let uuid = UUID::max();
        assert_eq!(uuid & 0u32, UUID::nil());
        assert_eq!(uuid & UUID::nil(), UUID::nil());
    }

    #[test]
    fn bitand_with_max() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid & UUID::max(), uuid);
    }

    // -------------------------------------------------------------------------
    // Bitwise OR tests
    // -------------------------------------------------------------------------

    #[test]
    fn bitor_uuid_uuid() {
        let a = UUID::from(0b1010_0000u128);
        let b = UUID::from(0b0000_1010u128);
        assert_eq!(u128::from(a | b), 0b1010_1010);
    }

    #[test]
    fn bitor_uuid_int() {
        let uuid = UUID::from(0b1010_0000u128);
        assert_eq!(u128::from(uuid | 0b0000_1010u32), 0b1010_1010);
    }

    #[test]
    fn bitor_int_uuid() {
        let uuid = UUID::from(0b1010_0000u128);
        assert_eq!(u128::from(0b0000_1010u32 | uuid), 0b1010_1010);
    }

    #[test]
    fn bitor_assign() {
        let mut uuid = UUID::from(0b1010_0000u128);
        uuid |= 0b0000_1010u32;
        assert_eq!(u128::from(uuid), 0b1010_1010);
    }

    #[test]
    fn bitor_with_zero() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid | 0u32, uuid);
        assert_eq!(uuid | UUID::nil(), uuid);
    }

    #[test]
    fn bitor_with_max() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid | UUID::max(), UUID::max());
    }

    // -------------------------------------------------------------------------
    // Bitwise XOR tests
    // -------------------------------------------------------------------------

    #[test]
    fn bitxor_uuid_uuid() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(u128::from(a ^ b), 0b0110_0110);
    }

    #[test]
    fn bitxor_uuid_int() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(u128::from(uuid ^ 0b1111_1111u32), 0b0101_0101);
    }

    #[test]
    fn bitxor_int_uuid() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(u128::from(0b1111_1111u32 ^ uuid), 0b0101_0101);
    }

    #[test]
    fn bitxor_assign() {
        let mut uuid = UUID::from(0b1010_1010u128);
        uuid ^= 0b1111_1111u32;
        assert_eq!(u128::from(uuid), 0b0101_0101);
    }

    #[test]
    fn bitxor_with_zero() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid ^ 0u32, uuid);
        assert_eq!(uuid ^ UUID::nil(), uuid);
    }

    #[test]
    fn bitxor_with_self() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid ^ uuid, UUID::nil());
    }

    #[test]
    fn bitxor_with_max() {
        let uuid = UUID::from(0b1010_1010u128);
        // XOR with all 1s is equivalent to NOT
        assert_eq!(uuid ^ UUID::max(), !uuid);
    }

    // -------------------------------------------------------------------------
    // Bitwise NOT tests
    // -------------------------------------------------------------------------

    #[test]
    fn not_uuid() {
        let uuid = UUID::nil();
        assert_eq!(!uuid, UUID::max());
    }

    #[test]
    fn not_ref_uuid() {
        let uuid = UUID::nil();
        assert_eq!(!&uuid, UUID::max());
    }

    #[test]
    fn not_max() {
        assert_eq!(!UUID::max(), UUID::nil());
    }

    #[test]
    fn not_involution() {
        // NOT(NOT(x)) == x
        let uuid = UUID::from(0x0123_4567_89ab_cdef_u128);
        assert_eq!(!!uuid, uuid);
    }

    #[test]
    fn not_pattern() {
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(u128::from(!uuid) & 0xFF, 0b0101_0101);
    }

    // -------------------------------------------------------------------------
    // De Morgan's laws
    // -------------------------------------------------------------------------

    #[test]
    fn de_morgan_and() {
        // !(a & b) == !a | !b
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(!(a & b), !a | !b);
    }

    #[test]
    fn de_morgan_or() {
        // !(a | b) == !a & !b
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(!(a | b), !a & !b);
    }

    // -------------------------------------------------------------------------
    // Negative integer operand tests
    // -------------------------------------------------------------------------

    #[test]
    fn bitand_with_negative_int() {
        // -1i8 sign-extends to all 1s
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid & (-1i8), uuid); // AND with all 1s is identity
    }

    #[test]
    fn bitor_with_negative_int() {
        // -1i8 sign-extends to all 1s
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid | (-1i8), UUID::max()); // OR with all 1s is all 1s
    }

    #[test]
    fn bitxor_with_negative_int() {
        // -1i8 sign-extends to all 1s
        let uuid = UUID::from(0b1010_1010u128);
        assert_eq!(uuid ^ (-1i8), !uuid); // XOR with all 1s is NOT
    }

    // -------------------------------------------------------------------------
    // All reference variant tests
    // -------------------------------------------------------------------------

    #[test]
    fn all_reference_variants_bitand() {
        let a = UUID::from(0b1010u128);
        let b = UUID::from(0b1100u128);
        let n = 0b1100u32;
        let expected = UUID::from(0b1000u128);

        // UUID op UUID variants
        assert_eq!(a & b, expected);
        assert_eq!(a & &b, expected);
        assert_eq!(&a & b, expected);
        assert_eq!(&a & &b, expected);

        // UUID op T variants
        assert_eq!(a & n, expected);
        assert_eq!(&a & n, expected);
        assert_eq!(a & &n, expected);
        assert_eq!(&a & &n, expected);

        // T op UUID variants
        assert_eq!(n & a, expected);
        assert_eq!(n & &a, expected);
        assert_eq!(&n & a, expected);
        assert_eq!(&n & &a, expected);
    }
}
