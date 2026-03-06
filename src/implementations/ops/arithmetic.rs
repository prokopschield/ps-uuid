//! Arithmetic operator implementations for UUID.
//!
//! This module provides implementations for:
//!
//! | Operator | Trait | Semantics |
//! |----------|-------|-----------|
//! | `+` | [`Add`] | Wrapping addition |
//! | `-` | [`Sub`] | Wrapping subtraction |
//! | `*` | [`Mul`] | Wrapping multiplication |
//! | `/` | [`Div`] | Standard division (panics on zero) |
//! | `%` | [`Rem`] | Standard remainder (panics on zero) |
//!
//! # Wrapping Semantics
//!
//! Addition, subtraction, and multiplication use **wrapping semantics** to match
//! typical integer behavior and avoid panics on overflow. This means:
//!
//! - `UUID::max() + 1` wraps to `UUID::nil()`
//! - `UUID::nil() - 1` wraps to `UUID::max()`
//! - Multiplication can overflow and wrap
//!
//! # Division and Remainder
//!
//! Division and remainder use standard semantics and will **panic** if the
//! divisor is zero, matching the behavior of `u128`.
//!
//! # Supported Operand Combinations
//!
//! Each operator supports all of:
//!
//! - `UUID op UUID` and ref variants
//! - `UUID op T` where `T: IntOperand` (all integer types)
//! - `T op UUID` where `T: IntOperand`
//! - Compound assignment: `UUID op= UUID`, `UUID op= T`
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // Basic arithmetic
//! let a = UUID::from(100u128);
//! let b = UUID::from(30u128);
//!
//! assert_eq!(u128::from(a + b), 130);
//! assert_eq!(u128::from(a - b), 70);
//! assert_eq!(u128::from(a * 2u32), 200);
//! assert_eq!(u128::from(a / 10u32), 10);
//! assert_eq!(u128::from(a % 30u32), 10);
//!
//! // Wrapping behavior
//! let max = UUID::max();
//! assert_eq!(max + 1u32, UUID::nil());
//!
//! let zero = UUID::nil();
//! assert_eq!(zero - 1u32, UUID::max());
//!
//! // Compound assignment
//! let mut uuid = UUID::from(10u128);
//! uuid += 5u32;
//! assert_eq!(u128::from(uuid), 15);
//! ```

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use super::int_operand::IntOperand;
use super::{
    impl_binop_uuid_uuid, impl_int_op_uuid_commutative, impl_int_op_uuid_noncommutative,
    impl_ref_lhs_uuid, impl_uuid_op_int,
};
use crate::UUID;

// ============================================================================
// UUID op UUID Implementations
// ============================================================================

impl_binop_uuid_uuid!(Add, add, AddAssign, add_assign, u128::wrapping_add);
impl_binop_uuid_uuid!(Sub, sub, SubAssign, sub_assign, u128::wrapping_sub);
impl_binop_uuid_uuid!(Mul, mul, MulAssign, mul_assign, u128::wrapping_mul);
impl_binop_uuid_uuid!(Div, div, DivAssign, div_assign, u128::div);
impl_binop_uuid_uuid!(Rem, rem, RemAssign, rem_assign, u128::rem);

impl_ref_lhs_uuid!(Add, add);
impl_ref_lhs_uuid!(Sub, sub);
impl_ref_lhs_uuid!(Mul, mul);
impl_ref_lhs_uuid!(Div, div);
impl_ref_lhs_uuid!(Rem, rem);

// ============================================================================
// UUID op T Implementations (where T: IntOperand)
// ============================================================================

impl_uuid_op_int!(Add, add, AddAssign, add_assign, u128::wrapping_add);
impl_uuid_op_int!(Sub, sub, SubAssign, sub_assign, u128::wrapping_sub);
impl_uuid_op_int!(Mul, mul, MulAssign, mul_assign, u128::wrapping_mul);
impl_uuid_op_int!(Div, div, DivAssign, div_assign, u128::div);
impl_uuid_op_int!(Rem, rem, RemAssign, rem_assign, u128::rem);

// ============================================================================
// T op UUID Implementations (where T: IntOperand)
// ============================================================================

// Add and Mul are commutative: T + UUID == UUID + T
impl_int_op_uuid_commutative!(
    Add, add, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
impl_int_op_uuid_commutative!(
    Mul, mul, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

// Sub, Div, Rem are non-commutative: T - UUID != UUID - T
impl_int_op_uuid_noncommutative!(
    Sub,
    sub,
    u128::wrapping_sub,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);
impl_int_op_uuid_noncommutative!(
    Div,
    div,
    u128::div,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);
impl_int_op_uuid_noncommutative!(
    Rem,
    rem,
    u128::rem,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    #![allow(clippy::op_ref)]

    use super::*;

    // -------------------------------------------------------------------------
    // Addition tests
    // -------------------------------------------------------------------------

    #[test]
    fn add_uuid_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(50u128);
        assert_eq!(u128::from(a + b), 150);
    }

    #[test]
    fn add_uuid_ref_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(50u128);
        assert_eq!(u128::from(a + &b), 150);
    }

    #[test]
    fn add_ref_uuid_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(50u128);
        assert_eq!(u128::from(&a + b), 150);
    }

    #[test]
    fn add_ref_uuid_ref_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(50u128);
        assert_eq!(u128::from(&a + &b), 150);
    }

    #[test]
    fn add_uuid_int() {
        let uuid = UUID::from(100u128);
        assert_eq!(u128::from(uuid + 50u8), 150);
        assert_eq!(u128::from(uuid + 50u16), 150);
        assert_eq!(u128::from(uuid + 50u32), 150);
        assert_eq!(u128::from(uuid + 50u64), 150);
        assert_eq!(u128::from(uuid + 50u128), 150);
        assert_eq!(u128::from(uuid + 50usize), 150);
        assert_eq!(u128::from(uuid + 50i8), 150);
        assert_eq!(u128::from(uuid + 50i16), 150);
        assert_eq!(u128::from(uuid + 50i32), 150);
        assert_eq!(u128::from(uuid + 50i64), 150);
        assert_eq!(u128::from(uuid + 50i128), 150);
        assert_eq!(u128::from(uuid + 50isize), 150);
    }

    #[test]
    fn add_int_uuid() {
        let uuid = UUID::from(100u128);
        assert_eq!(u128::from(50u8 + uuid), 150);
        assert_eq!(u128::from(50u16 + uuid), 150);
        assert_eq!(u128::from(50u32 + uuid), 150);
        assert_eq!(u128::from(50u64 + uuid), 150);
        assert_eq!(u128::from(50u128 + uuid), 150);
        assert_eq!(u128::from(50usize + uuid), 150);
    }

    #[test]
    fn add_int_ref_uuid() {
        let uuid = UUID::from(100u128);
        assert_eq!(u128::from(50u32 + &uuid), 150);
    }

    #[test]
    fn add_ref_int_uuid() {
        let uuid = UUID::from(100u128);
        let n = 50u32;
        assert_eq!(u128::from(&n + uuid), 150);
    }

    #[test]
    fn add_ref_int_ref_uuid() {
        let uuid = UUID::from(100u128);
        let n = 50u32;
        assert_eq!(u128::from(&n + &uuid), 150);
    }

    #[test]
    fn add_wraps_on_overflow() {
        let max = UUID::max();
        assert_eq!(max + 1u32, UUID::nil());
        assert_eq!(max + 2u32, UUID::from(1u128));
    }

    #[test]
    fn add_assign_uuid() {
        let mut uuid = UUID::from(100u128);
        uuid += UUID::from(50u128);
        assert_eq!(u128::from(uuid), 150);
    }

    #[test]
    fn add_assign_ref_uuid() {
        let mut uuid = UUID::from(100u128);
        let other = UUID::from(50u128);
        uuid += &other;
        assert_eq!(u128::from(uuid), 150);
    }

    #[test]
    fn add_assign_int() {
        let mut uuid = UUID::from(100u128);
        uuid += 50u32;
        assert_eq!(u128::from(uuid), 150);
    }

    // -------------------------------------------------------------------------
    // Subtraction tests
    // -------------------------------------------------------------------------

    #[test]
    fn sub_uuid_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(30u128);
        assert_eq!(u128::from(a - b), 70);
    }

    #[test]
    fn sub_uuid_int() {
        let uuid = UUID::from(100u128);
        assert_eq!(u128::from(uuid - 30u32), 70);
    }

    #[test]
    fn sub_int_uuid() {
        let uuid = UUID::from(30u128);
        assert_eq!(u128::from(100u32 - uuid), 70);
    }

    #[test]
    fn sub_wraps_on_underflow() {
        let zero = UUID::nil();
        assert_eq!(zero - 1u32, UUID::max());
        assert_eq!(zero - 2u32, UUID::from(u128::MAX - 1));
    }

    #[test]
    fn sub_noncommutative() {
        let a = UUID::from(100u128);
        // UUID - int != int - UUID (unless they're equal)
        assert_eq!(u128::from(a - 30u32), 70);
        assert_eq!(u128::from(30u32 - a), u128::MAX - 69); // Wraps around
    }

    #[test]
    fn sub_assign() {
        let mut uuid = UUID::from(100u128);
        uuid -= 30u32;
        assert_eq!(u128::from(uuid), 70);
    }

    // -------------------------------------------------------------------------
    // Multiplication tests
    // -------------------------------------------------------------------------

    #[test]
    fn mul_uuid_uuid() {
        let a = UUID::from(10u128);
        let b = UUID::from(20u128);
        assert_eq!(u128::from(a * b), 200);
    }

    #[test]
    fn mul_uuid_int() {
        let uuid = UUID::from(10u128);
        assert_eq!(u128::from(uuid * 20u32), 200);
    }

    #[test]
    fn mul_int_uuid() {
        let uuid = UUID::from(10u128);
        assert_eq!(u128::from(20u32 * uuid), 200);
    }

    #[test]
    fn mul_wraps_on_overflow() {
        let large = UUID::from(u128::MAX / 2 + 1);
        let result = large * 2u32;
        // (u128::MAX / 2 + 1) * 2 wraps
        assert_eq!(u128::from(result), 0);
    }

    #[test]
    fn mul_assign() {
        let mut uuid = UUID::from(10u128);
        uuid *= 20u32;
        assert_eq!(u128::from(uuid), 200);
    }

    // -------------------------------------------------------------------------
    // Division tests
    // -------------------------------------------------------------------------

    #[test]
    fn div_uuid_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(10u128);
        assert_eq!(u128::from(a / b), 10);
    }

    #[test]
    fn div_uuid_int() {
        let uuid = UUID::from(100u128);
        assert_eq!(u128::from(uuid / 10u32), 10);
    }

    #[test]
    fn div_int_uuid() {
        let uuid = UUID::from(10u128);
        assert_eq!(u128::from(100u32 / uuid), 10);
    }

    #[test]
    fn div_truncates() {
        let uuid = UUID::from(10u128);
        assert_eq!(u128::from(uuid / 3u32), 3); // 10 / 3 = 3 (truncated)
    }

    #[test]
    fn div_assign() {
        let mut uuid = UUID::from(100u128);
        uuid /= 10u32;
        assert_eq!(u128::from(uuid), 10);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn div_by_zero_panics() {
        let uuid = UUID::from(100u128);
        let _ = uuid / 0u32;
    }

    // -------------------------------------------------------------------------
    // Remainder tests
    // -------------------------------------------------------------------------

    #[test]
    fn rem_uuid_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(30u128);
        assert_eq!(u128::from(a % b), 10);
    }

    #[test]
    fn rem_uuid_int() {
        let uuid = UUID::from(100u128);
        assert_eq!(u128::from(uuid % 30u32), 10);
    }

    #[test]
    fn rem_int_uuid() {
        let uuid = UUID::from(30u128);
        assert_eq!(u128::from(100u32 % uuid), 10);
    }

    #[test]
    fn rem_assign() {
        let mut uuid = UUID::from(100u128);
        uuid %= 30u32;
        assert_eq!(u128::from(uuid), 10);
    }

    #[test]
    #[should_panic(expected = "attempt to calculate the remainder with a divisor of zero")]
    fn rem_by_zero_panics() {
        let uuid = UUID::from(100u128);
        let _ = uuid % 0u32;
    }

    // -------------------------------------------------------------------------
    // Reference variant comprehensive tests
    // -------------------------------------------------------------------------

    #[test]
    fn all_reference_variants_add() {
        let a = UUID::from(100u128);
        let b = UUID::from(50u128);
        let n = 50u32;

        // UUID op UUID variants
        assert_eq!(a + b, UUID::from(150u128));
        assert_eq!(a + &b, UUID::from(150u128));
        assert_eq!(&a + b, UUID::from(150u128));
        assert_eq!(&a + &b, UUID::from(150u128));

        // UUID op T variants
        assert_eq!(a + n, UUID::from(150u128));
        assert_eq!(&a + n, UUID::from(150u128));
        assert_eq!(a + &n, UUID::from(150u128));
        assert_eq!(&a + &n, UUID::from(150u128));

        // T op UUID variants
        assert_eq!(n + a, UUID::from(150u128));
        assert_eq!(n + &a, UUID::from(150u128));
        assert_eq!(&n + a, UUID::from(150u128));
        assert_eq!(&n + &a, UUID::from(150u128));
    }

    // -------------------------------------------------------------------------
    // Negative integer operand tests
    // -------------------------------------------------------------------------

    #[test]
    fn add_negative_int_wraps() {
        let uuid = UUID::from(100u128);
        // Adding -1i32 is like adding u128::MAX (due to sign extension)
        // which wraps to uuid - 1
        let result = uuid + (-1i32);
        assert_eq!(result, UUID::from(99u128));
    }

    #[test]
    fn sub_negative_int() {
        let uuid = UUID::from(100u128);
        // Subtracting -1i32 (which is u128::MAX after sign extension)
        // is like adding 1
        let result = uuid - (-1i32);
        assert_eq!(result, UUID::from(101u128));
    }

    // -------------------------------------------------------------------------
    // Identity and boundary tests
    // -------------------------------------------------------------------------

    #[test]
    fn add_zero_is_identity() {
        let uuid = UUID::from(42u128);
        assert_eq!(uuid + 0u32, uuid);
        assert_eq!(uuid + UUID::nil(), uuid);
    }

    #[test]
    fn sub_zero_is_identity() {
        let uuid = UUID::from(42u128);
        assert_eq!(uuid - 0u32, uuid);
        assert_eq!(uuid - UUID::nil(), uuid);
    }

    #[test]
    fn mul_one_is_identity() {
        let uuid = UUID::from(42u128);
        assert_eq!(uuid * 1u32, uuid);
        assert_eq!(uuid * UUID::from(1u128), uuid);
    }

    #[test]
    #[allow(clippy::erasing_op)]
    fn mul_zero_is_zero() {
        let uuid = UUID::from(42u128);
        assert_eq!(uuid * 0u32, UUID::nil());
        assert_eq!(uuid * UUID::nil(), UUID::nil());
    }

    #[test]
    fn div_one_is_identity() {
        let uuid = UUID::from(42u128);
        assert_eq!(uuid / 1u32, uuid);
        assert_eq!(uuid / UUID::from(1u128), uuid);
    }

    #[test]
    fn div_self_is_one() {
        let uuid = UUID::from(42u128);
        assert_eq!(uuid / uuid, UUID::from(1u128));
    }

    #[test]
    fn rem_self_is_zero() {
        let uuid = UUID::from(42u128);
        assert_eq!(uuid % uuid, UUID::nil());
    }
}
