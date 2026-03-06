//! Operator trait implementations for UUID.
//!
//! This module provides comprehensive implementations of standard Rust operator
//! traits (`core::ops`) for the UUID type, enabling arithmetic, bitwise, shift,
//! and indexing operations directly on UUID values.
//!
//! # Design Principles
//!
//! - **Consistency with `u128`**: All operations behave identically to their
//!   `u128` counterparts, as UUID is internally a 128-bit big-endian value.
//! - **Wrapping semantics**: Arithmetic operations use wrapping behavior to
//!   match typical integer semantics and avoid panics.
//! - **Complete trait coverage**: All operator traits include implementations
//!   for owned values, references, and compound assignment variants.
//! - **Mixed-type operations**: Operators work with UUID and any integer type.
//! - **Zero-cost abstractions**: All functions are marked `#[inline]` to ensure
//!   optimal codegen.
//!
//! # Supported Operations
//!
//! ## Arithmetic Operations
//!
//! | Operator | Trait | Semantics |
//! |----------|-------|-----------|
//! | `+` | [`Add`](core::ops::Add) | Wrapping addition |
//! | `-` | [`Sub`](core::ops::Sub) | Wrapping subtraction |
//! | `*` | [`Mul`](core::ops::Mul) | Wrapping multiplication |
//! | `/` | [`Div`](core::ops::Div) | Standard division (panics on zero) |
//! | `%` | [`Rem`](core::ops::Rem) | Standard remainder (panics on zero) |
//! | `-` (unary) | [`Neg`](core::ops::Neg) | Two's complement negation |
//!
//! ## Bitwise Operations
//!
//! | Operator | Trait | Description |
//! |----------|-------|-------------|
//! | `&` | [`BitAnd`](core::ops::BitAnd) | Bitwise AND |
//! | `\|` | [`BitOr`](core::ops::BitOr) | Bitwise OR |
//! | `^` | [`BitXor`](core::ops::BitXor) | Bitwise XOR |
//! | `!` | [`Not`](core::ops::Not) | Bitwise NOT |
//!
//! ## Shift Operations
//!
//! | Operator | Trait | Semantics |
//! |----------|-------|-----------|
//! | `<<` | [`Shl`](core::ops::Shl) | Wrapping left shift |
//! | `>>` | [`Shr`](core::ops::Shr) | Wrapping right shift |
//!
//! ## Indexing Operations
//!
//! | Trait | Description |
//! |-------|-------------|
//! | [`Index`](core::ops::Index) | Immutable byte access |
//! | [`IndexMut`](core::ops::IndexMut) | Mutable byte access |
//!
//! # Operand Combinations
//!
//! For binary operators, all of the following combinations are supported:
//!
//! - `UUID op UUID`
//! - `UUID op &UUID`
//! - `&UUID op UUID`
//! - `&UUID op &UUID`
//! - `UUID op T` where `T: IntOperand` (all integer types)
//! - `&UUID op T`
//! - `T op UUID`
//! - `T op &UUID`
//! - `&T op UUID`
//! - `&T op &UUID`
//!
//! Compound assignment operators (`+=`, `-=`, etc.) support:
//!
//! - `UUID op= UUID`
//! - `UUID op= &UUID`
//! - `UUID op= T`
//! - `UUID op= &T`
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // Arithmetic
//! let a = UUID::from(100u128);
//! let b = UUID::from(50u128);
//! assert_eq!(u128::from(a + b), 150);
//! assert_eq!(u128::from(a - b), 50);
//! assert_eq!(u128::from(a * 2u32), 200);
//!
//! // Bitwise
//! let x = UUID::from(0b1010u128);
//! let y = UUID::from(0b1100u128);
//! assert_eq!(u128::from(x & y), 0b1000);
//! assert_eq!(u128::from(x | y), 0b1110);
//! assert_eq!(u128::from(x ^ y), 0b0110);
//!
//! // Shift
//! let z = UUID::from(1u128);
//! assert_eq!(u128::from(z << 4u32), 16);
//! assert_eq!(u128::from(UUID::from(16u128) >> 2u32), 4);
//!
//! // Negation
//! let one = UUID::from(1u128);
//! assert_eq!(-one, UUID::max()); // Two's complement: -1 == all bits set
//!
//! // Indexing
//! let uuid = UUID::from(0xFF00_0000_0000_0000_0000_0000_0000_00FFu128);
//! assert_eq!(uuid[0], 0xFF);  // First byte (big-endian)
//! assert_eq!(uuid[15], 0xFF); // Last byte
//! ```

mod arithmetic;
mod bitwise;
#[cfg(test)]
mod comprehensive_tests;
mod index;
mod int_operand;
mod neg;
mod shift;
mod shift_amount;

#[allow(unused_imports)]
pub use int_operand::IntOperand;
#[allow(unused_imports)]
pub use shift_amount::ShiftAmount;

// ============================================================================
// Internal Macros for Generating Operator Implementations
// ============================================================================

/// Generates a binary operator with its compound assignment variant for UUID op UUID.
///
/// This macro creates:
/// - `impl Trait for UUID` (UUID op UUID)
/// - `impl Trait<&Self> for UUID` (UUID op &UUID)
/// - `impl TraitAssign for UUID` (UUID op= UUID)
/// - `impl TraitAssign<&Self> for UUID` (UUID op= &UUID)
macro_rules! impl_binop_uuid_uuid {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident, $op:expr) => {
        // UUID op UUID
        impl $trait for UUID {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                Self::from_u128(($op)(self.to_u128(), rhs.to_u128()))
            }
        }

        // UUID op &UUID
        impl $trait<&Self> for UUID {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: &Self) -> Self::Output {
                <Self as $trait>::$method(self, *rhs)
            }
        }

        // UUID op= UUID
        impl $assign_trait for UUID {
            #[inline]
            fn $assign_method(&mut self, rhs: Self) {
                *self = <Self as $trait>::$method(*self, rhs);
            }
        }

        // UUID op= &UUID
        impl $assign_trait<&Self> for UUID {
            #[inline]
            fn $assign_method(&mut self, rhs: &Self) {
                *self = <Self as $trait>::$method(*self, *rhs);
            }
        }
    };
}

/// Generates reference-on-LHS variants for a binary operator.
///
/// This macro creates:
/// - `impl Trait<UUID> for &UUID` (&UUID op UUID)
/// - `impl Trait<&UUID> for &UUID` (&UUID op &UUID)
macro_rules! impl_ref_lhs_uuid {
    ($trait:ident, $method:ident) => {
        // &UUID op UUID
        impl $trait<UUID> for &UUID {
            type Output = UUID;

            #[inline]
            fn $method(self, rhs: UUID) -> Self::Output {
                <UUID as $trait>::$method(*self, rhs)
            }
        }

        // &UUID op &UUID
        impl<'a, 'b> $trait<&'a UUID> for &'b UUID {
            type Output = UUID;

            #[inline]
            fn $method(self, rhs: &'a UUID) -> Self::Output {
                <UUID as $trait>::$method(*self, *rhs)
            }
        }
    };
}

/// Generates UUID op T implementations where T: `IntOperand`.
///
/// This macro creates:
/// - `impl Trait<T> for UUID` (UUID op T)
/// - `impl Trait<T> for &UUID` (&UUID op T)
/// - `impl TraitAssign<T> for UUID` (UUID op= T)
macro_rules! impl_uuid_op_int {
    ($trait:ident, $method:ident, $assign_trait:ident, $assign_method:ident, $op:expr) => {
        // UUID op T
        impl<T: IntOperand> $trait<T> for UUID {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                Self::from_u128(($op)(self.to_u128(), rhs.to_u128()))
            }
        }

        // &UUID op T
        impl<T: IntOperand> $trait<T> for &UUID {
            type Output = UUID;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                <UUID as $trait<T>>::$method(*self, rhs)
            }
        }

        // UUID op= T
        impl<T: IntOperand> $assign_trait<T> for UUID {
            #[inline]
            fn $assign_method(&mut self, rhs: T) {
                *self = <UUID as $trait<T>>::$method(*self, rhs);
            }
        }
    };
}

/// Generates T op UUID implementations for a commutative operation.
///
/// For commutative operations (add, mul, bitand, bitor, bitxor), we can
/// simply delegate to UUID op T.
macro_rules! impl_int_op_uuid_commutative {
    ($trait:ident, $method:ident, $($t:ty),* $(,)?) => {
        $(
            // T op UUID
            impl $trait<UUID> for $t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: UUID) -> Self::Output {
                    <UUID as $trait<$t>>::$method(rhs, self)
                }
            }

            // T op &UUID
            impl $trait<&UUID> for $t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: &UUID) -> Self::Output {
                    <UUID as $trait<$t>>::$method(*rhs, self)
                }
            }

            // &T op UUID
            impl $trait<UUID> for &$t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: UUID) -> Self::Output {
                    <UUID as $trait<$t>>::$method(rhs, *self)
                }
            }

            // &T op &UUID
            impl $trait<&UUID> for &$t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: &UUID) -> Self::Output {
                    <UUID as $trait<$t>>::$method(*rhs, *self)
                }
            }
        )*
    };
}

/// Generates T op UUID implementations for a non-commutative operation.
///
/// For non-commutative operations (sub, div, rem), we must compute
/// T op UUID directly, not delegate to UUID op T.
macro_rules! impl_int_op_uuid_noncommutative {
    ($trait:ident, $method:ident, $op:expr, $($t:ty),* $(,)?) => {
        $(
            // T op UUID
            impl $trait<UUID> for $t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: UUID) -> Self::Output {
                    UUID::from_u128(($op)(<$t as IntOperand>::to_u128(self), rhs.to_u128()))
                }
            }

            // T op &UUID
            impl $trait<&UUID> for $t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: &UUID) -> Self::Output {
                    <$t as $trait<UUID>>::$method(self, *rhs)
                }
            }

            // &T op UUID
            impl $trait<UUID> for &$t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: UUID) -> Self::Output {
                    <$t as $trait<UUID>>::$method(*self, rhs)
                }
            }

            // &T op &UUID
            impl $trait<&UUID> for &$t {
                type Output = UUID;

                #[inline]
                fn $method(self, rhs: &UUID) -> Self::Output {
                    <$t as $trait<UUID>>::$method(*self, *rhs)
                }
            }
        )*
    };
}

pub(crate) use impl_binop_uuid_uuid;
pub(crate) use impl_int_op_uuid_commutative;
pub(crate) use impl_int_op_uuid_noncommutative;
pub(crate) use impl_ref_lhs_uuid;
pub(crate) use impl_uuid_op_int;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::UUID;

    #[test]
    fn to_u128_and_from_u128_roundtrip() {
        let values = [0u128, 1, 42, u128::MAX, 0x0123_4567_89ab_cdef];
        for &v in &values {
            let uuid = UUID::from_u128(v);
            assert_eq!(uuid.to_u128(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn to_u128_preserves_byte_order() {
        let uuid = UUID::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]);
        assert_eq!(uuid.to_u128(), 0x0011_2233_4455_6677_8899_aabb_ccdd_eeff);
    }

    #[test]
    fn from_u128_preserves_byte_order() {
        let uuid = UUID::from_u128(0x0011_2233_4455_6677_8899_aabb_ccdd_eeff);
        assert_eq!(
            *uuid.as_bytes(),
            [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
                0xee, 0xff
            ]
        );
    }
}
