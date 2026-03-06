//! Unary negation operator implementation for UUID.
//!
//! This module provides the implementation for:
//!
//! | Operator | Trait | Semantics |
//! |----------|-------|-----------|
//! | `-` (unary) | [`Neg`] | Two's complement negation |
//!
//! # Two's Complement Negation
//!
//! Negation uses **two's complement** semantics, which is the standard for
//! representing signed integers in binary. For a value `x`, `-x` is computed as
//! `!x + 1` (bitwise NOT plus one).
//!
//! This means:
//!
//! - `-UUID::nil()` = `UUID::nil()` (0 is its own negation)
//! - `-UUID::from(1u128)` = `UUID::max()` (equivalent to -1 in two's complement)
//! - `-UUID::max()` = `UUID::from(1u128)` (negating -1 gives 1)
//!
//! # Mathematical Properties
//!
//! - `x + (-x) = 0` (additive inverse)
//! - `-(-x) = x` (involution)
//! - `-0 = 0` (zero is its own negation)
//!
//! # Supported Operand Combinations
//!
//! - `-UUID`
//! - `-&UUID`
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // Negating 1 gives the two's complement representation of -1
//! let one = UUID::from(1u128);
//! assert_eq!(-one, UUID::max());
//!
//! // Negating max (which represents -1 in two's complement) gives 1
//! assert_eq!(-UUID::max(), UUID::from(1u128));
//!
//! // Zero is its own negation
//! assert_eq!(-UUID::nil(), UUID::nil());
//!
//! // Additive inverse property
//! let uuid = UUID::from(42u128);
//! assert_eq!(uuid + (-uuid), UUID::nil());
//!
//! // Double negation is identity
//! let uuid = UUID::from(12345u128);
//! assert_eq!(-(-uuid), uuid);
//! ```

use core::ops::Neg;

use crate::UUID;

// ============================================================================
// Negation Implementations
// ============================================================================

impl Neg for UUID {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::from_u128(self.to_u128().wrapping_neg())
    }
}

impl Neg for &UUID {
    type Output = UUID;

    #[inline]
    fn neg(self) -> Self::Output {
        UUID::from_u128(self.to_u128().wrapping_neg())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Basic negation tests
    // -------------------------------------------------------------------------

    #[test]
    fn neg_zero() {
        assert_eq!(-UUID::nil(), UUID::nil());
    }

    #[test]
    fn neg_one() {
        let one = UUID::from(1u128);
        // -1 in two's complement is all 1s
        assert_eq!(-one, UUID::max());
    }

    #[test]
    fn neg_max() {
        // max is all 1s, which represents -1 in two's complement
        // -(-1) = 1
        assert_eq!(-UUID::max(), UUID::from(1u128));
    }

    #[test]
    fn neg_two() {
        let two = UUID::from(2u128);
        // -2 in two's complement is u128::MAX - 1
        assert_eq!(u128::from(-two), u128::MAX - 1);
    }

    #[test]
    fn neg_ref_uuid() {
        let uuid = UUID::from(1u128);
        assert_eq!(-&uuid, UUID::max());
    }

    // -------------------------------------------------------------------------
    // Mathematical property tests
    // -------------------------------------------------------------------------

    #[test]
    fn additive_inverse() {
        // x + (-x) = 0
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(
                uuid + (-uuid),
                UUID::nil(),
                "additive inverse failed for {v}"
            );
        }
    }

    #[test]
    fn double_negation_is_identity() {
        // -(-x) = x
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(-(-uuid), uuid, "double negation failed for {v}");
        }
    }

    #[test]
    fn negation_is_equivalent_to_not_plus_one() {
        // -x = !x + 1
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            let negated = -uuid;
            let not_plus_one = !uuid + 1u32;
            assert_eq!(negated, not_plus_one, "neg != !x+1 for {v}");
        }
    }

    #[test]
    fn subtraction_via_negation() {
        // a - b = a + (-b)
        let a = UUID::from(100u128);
        let b = UUID::from(30u128);
        assert_eq!(a - b, a + (-b));
    }

    // -------------------------------------------------------------------------
    // Two's complement representation tests
    // -------------------------------------------------------------------------

    #[test]
    fn twos_complement_minus_one() {
        // -1 in two's complement is represented as all 1s
        let minus_one = -UUID::from(1u128);
        assert_eq!(minus_one, UUID::max());
        assert!(minus_one.as_bytes().iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn twos_complement_minus_two() {
        // -2 in two's complement
        let minus_two = -UUID::from(2u128);
        assert_eq!(u128::from(minus_two), u128::MAX - 1);
        // All bytes are 0xFF except the last byte is 0xFE
        let bytes = minus_two.as_bytes();
        assert!(bytes[..15].iter().all(|&b| b == 0xFF));
        assert_eq!(bytes[15], 0xFE);
    }

    #[test]
    fn twos_complement_high_bit() {
        // The value 1 << 127 (high bit set) represents the minimum signed value
        let high_bit = UUID::from(1u128 << 127);
        // Negating it wraps back to itself (overflow)
        assert_eq!(-high_bit, high_bit);
    }

    // -------------------------------------------------------------------------
    // Edge case tests
    // -------------------------------------------------------------------------

    #[test]
    fn neg_various_patterns() {
        // Test various bit patterns
        let patterns = [
            0b1010_1010u128,
            0b1111_0000u128,
            0x0123_4567_89ab_cdef_u128,
            (1u128 << 64) - 1, // Lower 64 bits set
            1u128 << 64,       // Only bit 64 set
        ];
        for v in patterns {
            let uuid = UUID::from(v);
            // Verify additive inverse property
            assert_eq!(
                uuid + (-uuid),
                UUID::nil(),
                "additive inverse failed for {v:#x}"
            );
            // Verify double negation
            assert_eq!(-(-uuid), uuid, "double negation failed for {v:#x}");
        }
    }

    #[test]
    fn neg_consecutive_values() {
        // Negations of consecutive values should differ by 1
        for v in 0u128..100 {
            let neg_v = -UUID::from(v);
            let neg_v_plus_1 = -UUID::from(v + 1);
            // -v and -(v+1) should differ by 1
            // Note: -v - 1 = -(v+1)
            assert_eq!(neg_v - 1u32, neg_v_plus_1);
        }
    }
}
