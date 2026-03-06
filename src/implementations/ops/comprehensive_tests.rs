//! Comprehensive integration tests for UUID operator implementations.
//!
//! This module provides extensive test coverage for all operator traits,
//! focusing on areas not covered by the inline module tests:
//!
//! - Integer type coverage for Sub/Div/Rem/BitOr/BitXor (all 12 types)
//! - Mathematical property verification
//! - Cross-operation consistency
//! - Compound assignment completeness
//! - Edge cases and large values
//! - Reference variant completeness

#[cfg(test)]
mod integer_type_coverage {
    //! Tests all operations with all 12 integer types.

    use crate::UUID;

    // =========================================================================
    // Sub - All Integer Types
    // =========================================================================

    macro_rules! test_sub_uuid_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(100u128);
                    let n: $ty = 30 as $ty;
                    assert_eq!(u128::from(uuid - n), 70);
                }
            )*
        };
    }

    test_sub_uuid_int! {
        sub_uuid_u8: u8,
        sub_uuid_u16: u16,
        sub_uuid_u32: u32,
        sub_uuid_u64: u64,
        sub_uuid_u128: u128,
        sub_uuid_usize: usize,
        sub_uuid_i8: i8,
        sub_uuid_i16: i16,
        sub_uuid_i32: i32,
        sub_uuid_i64: i64,
        sub_uuid_i128: i128,
        sub_uuid_isize: isize,
    }

    macro_rules! test_sub_int_uuid {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(30u128);
                    let n: $ty = 100 as $ty;
                    assert_eq!(u128::from(n - uuid), 70);
                }
            )*
        };
    }

    test_sub_int_uuid! {
        sub_u8_uuid: u8,
        sub_u16_uuid: u16,
        sub_u32_uuid: u32,
        sub_u64_uuid: u64,
        sub_u128_uuid: u128,
        sub_usize_uuid: usize,
        sub_i8_uuid: i8,
        sub_i16_uuid: i16,
        sub_i32_uuid: i32,
        sub_i64_uuid: i64,
        sub_i128_uuid: i128,
        sub_isize_uuid: isize,
    }

    // =========================================================================
    // Div - All Integer Types
    // =========================================================================

    macro_rules! test_div_uuid_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(100u128);
                    let n: $ty = 10 as $ty;
                    assert_eq!(u128::from(uuid / n), 10);
                }
            )*
        };
    }

    test_div_uuid_int! {
        div_uuid_u8: u8,
        div_uuid_u16: u16,
        div_uuid_u32: u32,
        div_uuid_u64: u64,
        div_uuid_u128: u128,
        div_uuid_usize: usize,
        div_uuid_i8: i8,
        div_uuid_i16: i16,
        div_uuid_i32: i32,
        div_uuid_i64: i64,
        div_uuid_i128: i128,
        div_uuid_isize: isize,
    }

    macro_rules! test_div_int_uuid {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(10u128);
                    let n: $ty = 100 as $ty;
                    assert_eq!(u128::from(n / uuid), 10);
                }
            )*
        };
    }

    test_div_int_uuid! {
        div_u8_uuid: u8,
        div_u16_uuid: u16,
        div_u32_uuid: u32,
        div_u64_uuid: u64,
        div_u128_uuid: u128,
        div_usize_uuid: usize,
        div_i8_uuid: i8,
        div_i16_uuid: i16,
        div_i32_uuid: i32,
        div_i64_uuid: i64,
        div_i128_uuid: i128,
        div_isize_uuid: isize,
    }

    // =========================================================================
    // Rem - All Integer Types
    // =========================================================================

    macro_rules! test_rem_uuid_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(100u128);
                    let n: $ty = 30 as $ty;
                    assert_eq!(u128::from(uuid % n), 10);
                }
            )*
        };
    }

    test_rem_uuid_int! {
        rem_uuid_u8: u8,
        rem_uuid_u16: u16,
        rem_uuid_u32: u32,
        rem_uuid_u64: u64,
        rem_uuid_u128: u128,
        rem_uuid_usize: usize,
        rem_uuid_i8: i8,
        rem_uuid_i16: i16,
        rem_uuid_i32: i32,
        rem_uuid_i64: i64,
        rem_uuid_i128: i128,
        rem_uuid_isize: isize,
    }

    macro_rules! test_rem_int_uuid {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(30u128);
                    let n: $ty = 100 as $ty;
                    assert_eq!(u128::from(n % uuid), 10);
                }
            )*
        };
    }

    test_rem_int_uuid! {
        rem_u8_uuid: u8,
        rem_u16_uuid: u16,
        rem_u32_uuid: u32,
        rem_u64_uuid: u64,
        rem_u128_uuid: u128,
        rem_usize_uuid: usize,
        rem_i8_uuid: i8,
        rem_i16_uuid: i16,
        rem_i32_uuid: i32,
        rem_i64_uuid: i64,
        rem_i128_uuid: i128,
        rem_isize_uuid: isize,
    }

    // =========================================================================
    // BitOr - All Integer Types
    // =========================================================================

    macro_rules! test_bitor_uuid_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(0b1010_0000u128);
                    let n: $ty = 0b0000_1010 as $ty;
                    assert_eq!(u128::from(uuid | n), 0b1010_1010);
                }
            )*
        };
    }

    test_bitor_uuid_int! {
        bitor_uuid_u8: u8,
        bitor_uuid_u16: u16,
        bitor_uuid_u32: u32,
        bitor_uuid_u64: u64,
        bitor_uuid_u128: u128,
        bitor_uuid_usize: usize,
    }

    macro_rules! test_bitor_int_uuid {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(0b1010_0000u128);
                    let n: $ty = 0b0000_1010 as $ty;
                    assert_eq!(u128::from(n | uuid), 0b1010_1010);
                }
            )*
        };
    }

    test_bitor_int_uuid! {
        bitor_u8_uuid: u8,
        bitor_u16_uuid: u16,
        bitor_u32_uuid: u32,
        bitor_u64_uuid: u64,
        bitor_u128_uuid: u128,
        bitor_usize_uuid: usize,
    }

    // =========================================================================
    // BitXor - All Integer Types
    // =========================================================================

    macro_rules! test_bitxor_uuid_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(0b1010_1010u128);
                    let n: $ty = 0b1111_0000 as $ty;
                    assert_eq!(u128::from(uuid ^ n), 0b0101_1010);
                }
            )*
        };
    }

    test_bitxor_uuid_int! {
        bitxor_uuid_u8: u8,
        bitxor_uuid_u16: u16,
        bitxor_uuid_u32: u32,
        bitxor_uuid_u64: u64,
        bitxor_uuid_u128: u128,
        bitxor_uuid_usize: usize,
    }

    macro_rules! test_bitxor_int_uuid {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(0b1010_1010u128);
                    let n: $ty = 0b1111_0000 as $ty;
                    assert_eq!(u128::from(n ^ uuid), 0b0101_1010);
                }
            )*
        };
    }

    test_bitxor_int_uuid! {
        bitxor_u8_uuid: u8,
        bitxor_u16_uuid: u16,
        bitxor_u32_uuid: u32,
        bitxor_u64_uuid: u64,
        bitxor_u128_uuid: u128,
        bitxor_usize_uuid: usize,
    }

    // =========================================================================
    // Shift - All Integer Types
    // =========================================================================

    macro_rules! test_shl_all_types {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(1u128);
                    let n: $ty = 4 as $ty;
                    assert_eq!(u128::from(uuid << n), 16);
                }
            )*
        };
    }

    test_shl_all_types! {
        shl_uuid_u8: u8,
        shl_uuid_u16: u16,
        shl_uuid_u32: u32,
        shl_uuid_u64: u64,
        shl_uuid_u128: u128,
        shl_uuid_usize: usize,
        shl_uuid_i8: i8,
        shl_uuid_i16: i16,
        shl_uuid_i32: i32,
        shl_uuid_i64: i64,
        shl_uuid_i128: i128,
        shl_uuid_isize: isize,
    }

    macro_rules! test_shr_all_types {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let uuid = UUID::from(16u128);
                    let n: $ty = 4 as $ty;
                    assert_eq!(u128::from(uuid >> n), 1);
                }
            )*
        };
    }

    test_shr_all_types! {
        shr_uuid_u8: u8,
        shr_uuid_u16: u16,
        shr_uuid_u32: u32,
        shr_uuid_u64: u64,
        shr_uuid_u128: u128,
        shr_uuid_usize: usize,
        shr_uuid_i8: i8,
        shr_uuid_i16: i16,
        shr_uuid_i32: i32,
        shr_uuid_i64: i64,
        shr_uuid_i128: i128,
        shr_uuid_isize: isize,
    }
}

#[cfg(test)]
mod mathematical_properties {
    //! Tests for algebraic properties of operations.

    use crate::UUID;

    // =========================================================================
    // Addition Properties
    // =========================================================================

    #[test]
    fn add_commutativity() {
        let a = UUID::from(123u128);
        let b = UUID::from(456u128);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn add_commutativity_various_values() {
        let pairs = [
            (0u128, 0u128),
            (1, 2),
            (100, 200),
            (u128::MAX / 2, u128::MAX / 3),
            (u128::MAX, 1),
        ];
        for (a_val, b_val) in pairs {
            let a = UUID::from(a_val);
            let b = UUID::from(b_val);
            assert_eq!(a + b, b + a, "commutativity failed for {a_val} + {b_val}");
        }
    }

    #[test]
    fn add_associativity() {
        let a = UUID::from(10u128);
        let b = UUID::from(20u128);
        let c = UUID::from(30u128);
        assert_eq!((a + b) + c, a + (b + c));
    }

    #[test]
    fn add_associativity_various_values() {
        let triples = [
            (1u128, 2, 3),
            (100, 200, 300),
            (u128::MAX / 4, u128::MAX / 4, u128::MAX / 4),
        ];
        for (a_val, b_val, c_val) in triples {
            let a = UUID::from(a_val);
            let b = UUID::from(b_val);
            let c = UUID::from(c_val);
            assert_eq!(
                (a + b) + c,
                a + (b + c),
                "associativity failed for ({a_val} + {b_val}) + {c_val}"
            );
        }
    }

    #[test]
    fn add_identity() {
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid + 0u32, uuid, "add identity failed for {v}");
            assert_eq!(uuid + UUID::nil(), uuid, "add identity failed for {v}");
        }
    }

    #[test]
    fn add_inverse() {
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid + (-uuid), UUID::nil(), "add inverse failed for {v}");
        }
    }

    // =========================================================================
    // Multiplication Properties
    // =========================================================================

    #[test]
    fn mul_commutativity() {
        let a = UUID::from(7u128);
        let b = UUID::from(11u128);
        assert_eq!(a * b, b * a);
    }

    #[test]
    fn mul_commutativity_various_values() {
        let pairs = [(0u128, 1u128), (2, 3), (17, 19), (1000, 2000)];
        for (a_val, b_val) in pairs {
            let a = UUID::from(a_val);
            let b = UUID::from(b_val);
            assert_eq!(a * b, b * a, "commutativity failed for {a_val} * {b_val}");
        }
    }

    #[test]
    fn mul_associativity() {
        let a = UUID::from(2u128);
        let b = UUID::from(3u128);
        let c = UUID::from(5u128);
        assert_eq!((a * b) * c, a * (b * c));
    }

    #[test]
    fn mul_identity() {
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid * 1u32, uuid, "mul identity failed for {v}");
            assert_eq!(
                uuid * UUID::from(1u128),
                uuid,
                "mul identity failed for {v}"
            );
        }
    }

    #[test]
    #[allow(clippy::erasing_op)]
    fn mul_zero() {
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid * 0u32, UUID::nil(), "mul zero failed for {v}");
            assert_eq!(uuid * UUID::nil(), UUID::nil(), "mul zero failed for {v}");
        }
    }

    #[test]
    fn mul_distributivity_over_add() {
        // a * (b + c) == a*b + a*c
        let a = UUID::from(5u128);
        let b = UUID::from(7u128);
        let c = UUID::from(11u128);
        assert_eq!(a * (b + c), (a * b) + (a * c));
    }

    #[test]
    fn mul_distributivity_various_values() {
        let triples = [(2u128, 3, 4), (10, 20, 30), (100, 200, 300)];
        for (a_val, b_val, c_val) in triples {
            let a = UUID::from(a_val);
            let b = UUID::from(b_val);
            let c = UUID::from(c_val);
            assert_eq!(
                a * (b + c),
                (a * b) + (a * c),
                "distributivity failed for {a_val} * ({b_val} + {c_val})"
            );
        }
    }

    // =========================================================================
    // Bitwise Properties
    // =========================================================================

    #[test]
    fn bitand_commutativity() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(a & b, b & a);
    }

    #[test]
    fn bitand_associativity() {
        let a = UUID::from(0b1111_0000u128);
        let b = UUID::from(0b1010_1010u128);
        let c = UUID::from(0b1100_1100u128);
        assert_eq!((a & b) & c, a & (b & c));
    }

    #[test]
    fn bitand_idempotence() {
        let values = [0u128, 1, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid & uuid, uuid, "bitand idempotence failed for {v}");
        }
    }

    #[test]
    fn bitor_commutativity() {
        let a = UUID::from(0b1010_0000u128);
        let b = UUID::from(0b0000_1010u128);
        assert_eq!(a | b, b | a);
    }

    #[test]
    fn bitor_associativity() {
        let a = UUID::from(0b1000_0000u128);
        let b = UUID::from(0b0100_0000u128);
        let c = UUID::from(0b0010_0000u128);
        assert_eq!((a | b) | c, a | (b | c));
    }

    #[test]
    fn bitor_idempotence() {
        let values = [0u128, 1, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid | uuid, uuid, "bitor idempotence failed for {v}");
        }
    }

    #[test]
    fn bitxor_commutativity() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(a ^ b, b ^ a);
    }

    #[test]
    fn bitxor_associativity() {
        let a = UUID::from(0b1111_0000u128);
        let b = UUID::from(0b1010_1010u128);
        let c = UUID::from(0b1100_1100u128);
        assert_eq!((a ^ b) ^ c, a ^ (b ^ c));
    }

    #[test]
    fn bitxor_self_inverse() {
        let values = [0u128, 1, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid ^ uuid, UUID::nil(), "xor self-inverse failed for {v}");
        }
    }

    #[test]
    fn absorption_law_and_or() {
        // a & (a | b) == a
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(a & (a | b), a);
    }

    #[test]
    fn absorption_law_or_and() {
        // a | (a & b) == a
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1100_1100u128);
        assert_eq!(a | (a & b), a);
    }

    // =========================================================================
    // Shift Properties
    // =========================================================================

    #[test]
    fn shift_identity() {
        let values = [0u128, 1, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid << 0u32, uuid, "shl identity failed for {v}");
            assert_eq!(uuid >> 0u32, uuid, "shr identity failed for {v}");
        }
    }

    #[test]
    fn shift_mul_equivalence() {
        // a << n == a * 2^n (when no overflow)
        let uuid = UUID::from(7u128);
        assert_eq!(uuid << 1u32, uuid * 2u32);
        assert_eq!(uuid << 2u32, uuid * 4u32);
        assert_eq!(uuid << 3u32, uuid * 8u32);
        assert_eq!(uuid << 4u32, uuid * 16u32);
    }

    #[test]
    fn shift_div_equivalence() {
        // a >> n == a / 2^n (integer division)
        let uuid = UUID::from(1024u128);
        assert_eq!(uuid >> 1u32, uuid / 2u32);
        assert_eq!(uuid >> 2u32, uuid / 4u32);
        assert_eq!(uuid >> 3u32, uuid / 8u32);
        assert_eq!(uuid >> 4u32, uuid / 16u32);
    }

    #[test]
    fn consecutive_shifts_combine() {
        // (a << n) << m == a << (n + m) (when n + m < 128)
        let uuid = UUID::from(1u128);
        assert_eq!((uuid << 3u32) << 4u32, uuid << 7u32);
        assert_eq!((uuid >> 3u32) >> 4u32, uuid >> 7u32);
    }
}

#[cfg(test)]
mod cross_operation_consistency {
    //! Tests that verify relationships between different operations.

    use crate::UUID;

    #[test]
    fn add_neg_equals_sub() {
        // a + (-b) == a - b
        let pairs = [(100u128, 30u128), (1000, 1), (u128::MAX, 1), (50, 50)];
        for (a_val, b_val) in pairs {
            let a = UUID::from(a_val);
            let b = UUID::from(b_val);
            assert_eq!(a + (-b), a - b, "a + (-b) != a - b for {a_val}, {b_val}");
        }
    }

    #[test]
    fn sub_neg_equals_add() {
        // a - (-b) == a + b
        let pairs = [(100u128, 30u128), (0, 1), (50, 50)];
        for (a_val, b_val) in pairs {
            let a = UUID::from(a_val);
            let b = UUID::from(b_val);
            assert_eq!(a - (-b), a + b, "a - (-b) != a + b for {a_val}, {b_val}");
        }
    }

    #[test]
    fn not_and_equals_zero() {
        // a & !a == 0
        let values = [0u128, 1, 42, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid & !uuid, UUID::nil(), "a & !a != 0 for {v}");
        }
    }

    #[test]
    fn not_or_equals_max() {
        // a | !a == MAX
        let values = [0u128, 1, 42, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid | !uuid, UUID::max(), "a | !a != MAX for {v}");
        }
    }

    #[test]
    fn xor_max_equals_not() {
        // a ^ MAX == !a
        let values = [0u128, 1, 42, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid ^ UUID::max(), !uuid, "a ^ MAX != !a for {v}");
        }
    }

    #[test]
    fn neg_equals_not_plus_one() {
        // -a == !a + 1
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(-uuid, !uuid + 1u32, "-a != !a + 1 for {v}");
        }
    }

    #[test]
    fn div_mul_rem_identity() {
        // (a / b) * b + (a % b) == a
        let pairs = [(100u128, 7u128), (1000, 13), (12345, 67), (u128::MAX, 3)];
        for (a_val, b_val) in pairs {
            let a = UUID::from(a_val);
            let b = UUID::from(b_val);
            let quotient = a / b;
            let remainder = a % b;
            assert_eq!(
                quotient * b + remainder,
                a,
                "div/mul/rem identity failed for {a_val} / {b_val}"
            );
        }
    }

    #[test]
    fn shl_shr_roundtrip_no_loss() {
        // (a << n) >> n == a when no bits are shifted out
        let uuid = UUID::from(0b1010_1010u128);
        for n in 0u32..=64 {
            assert_eq!(
                (uuid << n) >> n,
                uuid,
                "shl/shr roundtrip failed for shift {n}"
            );
        }
    }

    #[test]
    fn shr_shl_loses_low_bits() {
        // (a >> n) << n zeros out low n bits
        let uuid = UUID::from(0b1111_1111u128);
        assert_eq!((uuid >> 4u32) << 4u32, UUID::from(0b1111_0000u128));
    }

    #[test]
    fn double_negation() {
        // -(-a) == a
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(-(-uuid), uuid, "double negation failed for {v}");
        }
    }

    #[test]
    fn double_not() {
        // !!a == a
        let values = [0u128, 1, 42, 0b1010_1010, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(!!uuid, uuid, "double not failed for {v}");
        }
    }
}

#[cfg(test)]
mod compound_assignments {
    //! Tests for compound assignment operators.

    use crate::UUID;

    // =========================================================================
    // Sub Assign
    // =========================================================================

    #[test]
    fn sub_assign_uuid() {
        let mut uuid = UUID::from(100u128);
        uuid -= UUID::from(30u128);
        assert_eq!(u128::from(uuid), 70);
    }

    #[test]
    fn sub_assign_ref_uuid() {
        let mut uuid = UUID::from(100u128);
        let other = UUID::from(30u128);
        uuid -= &other;
        assert_eq!(u128::from(uuid), 70);
    }

    macro_rules! test_sub_assign_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut uuid = UUID::from(100u128);
                    let n: $ty = 30 as $ty;
                    uuid -= n;
                    assert_eq!(u128::from(uuid), 70);
                }
            )*
        };
    }

    test_sub_assign_int! {
        sub_assign_u8: u8,
        sub_assign_u16: u16,
        sub_assign_u32: u32,
        sub_assign_u64: u64,
        sub_assign_u128: u128,
        sub_assign_usize: usize,
        sub_assign_i8: i8,
        sub_assign_i16: i16,
        sub_assign_i32: i32,
        sub_assign_i64: i64,
        sub_assign_i128: i128,
        sub_assign_isize: isize,
    }

    // =========================================================================
    // Div Assign
    // =========================================================================

    #[test]
    fn div_assign_uuid() {
        let mut uuid = UUID::from(100u128);
        uuid /= UUID::from(10u128);
        assert_eq!(u128::from(uuid), 10);
    }

    #[test]
    fn div_assign_ref_uuid() {
        let mut uuid = UUID::from(100u128);
        let other = UUID::from(10u128);
        uuid /= &other;
        assert_eq!(u128::from(uuid), 10);
    }

    macro_rules! test_div_assign_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut uuid = UUID::from(100u128);
                    let n: $ty = 10 as $ty;
                    uuid /= n;
                    assert_eq!(u128::from(uuid), 10);
                }
            )*
        };
    }

    test_div_assign_int! {
        div_assign_u8: u8,
        div_assign_u16: u16,
        div_assign_u32: u32,
        div_assign_u64: u64,
        div_assign_u128: u128,
        div_assign_usize: usize,
        div_assign_i8: i8,
        div_assign_i16: i16,
        div_assign_i32: i32,
        div_assign_i64: i64,
        div_assign_i128: i128,
        div_assign_isize: isize,
    }

    // =========================================================================
    // Rem Assign
    // =========================================================================

    #[test]
    fn rem_assign_uuid() {
        let mut uuid = UUID::from(100u128);
        uuid %= UUID::from(30u128);
        assert_eq!(u128::from(uuid), 10);
    }

    #[test]
    fn rem_assign_ref_uuid() {
        let mut uuid = UUID::from(100u128);
        let other = UUID::from(30u128);
        uuid %= &other;
        assert_eq!(u128::from(uuid), 10);
    }

    macro_rules! test_rem_assign_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut uuid = UUID::from(100u128);
                    let n: $ty = 30 as $ty;
                    uuid %= n;
                    assert_eq!(u128::from(uuid), 10);
                }
            )*
        };
    }

    test_rem_assign_int! {
        rem_assign_u8: u8,
        rem_assign_u16: u16,
        rem_assign_u32: u32,
        rem_assign_u64: u64,
        rem_assign_u128: u128,
        rem_assign_usize: usize,
        rem_assign_i8: i8,
        rem_assign_i16: i16,
        rem_assign_i32: i32,
        rem_assign_i64: i64,
        rem_assign_i128: i128,
        rem_assign_isize: isize,
    }

    // =========================================================================
    // BitOr Assign
    // =========================================================================

    #[test]
    fn bitor_assign_uuid() {
        let mut uuid = UUID::from(0b1010_0000u128);
        uuid |= UUID::from(0b0000_1010u128);
        assert_eq!(u128::from(uuid), 0b1010_1010);
    }

    #[test]
    fn bitor_assign_ref_uuid() {
        let mut uuid = UUID::from(0b1010_0000u128);
        let other = UUID::from(0b0000_1010u128);
        uuid |= &other;
        assert_eq!(u128::from(uuid), 0b1010_1010);
    }

    macro_rules! test_bitor_assign_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut uuid = UUID::from(0b1010_0000u128);
                    let n: $ty = 0b0000_1010 as $ty;
                    uuid |= n;
                    assert_eq!(u128::from(uuid), 0b1010_1010);
                }
            )*
        };
    }

    test_bitor_assign_int! {
        bitor_assign_u8: u8,
        bitor_assign_u16: u16,
        bitor_assign_u32: u32,
        bitor_assign_u64: u64,
        bitor_assign_u128: u128,
        bitor_assign_usize: usize,
        bitor_assign_i8: i8,
        bitor_assign_i16: i16,
        bitor_assign_i32: i32,
        bitor_assign_i64: i64,
        bitor_assign_i128: i128,
        bitor_assign_isize: isize,
    }

    // =========================================================================
    // BitXor Assign
    // =========================================================================

    #[test]
    fn bitxor_assign_uuid() {
        let mut uuid = UUID::from(0b1010_1010u128);
        uuid ^= UUID::from(0b1111_0000u128);
        assert_eq!(u128::from(uuid), 0b0101_1010);
    }

    #[test]
    fn bitxor_assign_ref_uuid() {
        let mut uuid = UUID::from(0b1010_1010u128);
        let other = UUID::from(0b1111_0000u128);
        uuid ^= &other;
        assert_eq!(u128::from(uuid), 0b0101_1010);
    }

    macro_rules! test_bitxor_assign_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut uuid = UUID::from(0b0101_0101u128);
                    let n: $ty = 0b0011_0011 as $ty;
                    uuid ^= n;
                    assert_eq!(u128::from(uuid), 0b0110_0110);
                }
            )*
        };
    }

    test_bitxor_assign_int! {
        bitxor_assign_u8: u8,
        bitxor_assign_u16: u16,
        bitxor_assign_u32: u32,
        bitxor_assign_u64: u64,
        bitxor_assign_u128: u128,
        bitxor_assign_usize: usize,
        bitxor_assign_i8: i8,
        bitxor_assign_i16: i16,
        bitxor_assign_i32: i32,
        bitxor_assign_i64: i64,
        bitxor_assign_i128: i128,
        bitxor_assign_isize: isize,
    }

    // =========================================================================
    // Shift Assign
    // =========================================================================

    macro_rules! test_shl_assign_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut uuid = UUID::from(1u128);
                    let n: $ty = 4 as $ty;
                    uuid <<= n;
                    assert_eq!(u128::from(uuid), 16);
                }
            )*
        };
    }

    test_shl_assign_int! {
        shl_assign_u8: u8,
        shl_assign_u16: u16,
        shl_assign_u32: u32,
        shl_assign_u64: u64,
        shl_assign_u128: u128,
        shl_assign_usize: usize,
        shl_assign_i8: i8,
        shl_assign_i16: i16,
        shl_assign_i32: i32,
        shl_assign_i64: i64,
        shl_assign_i128: i128,
        shl_assign_isize: isize,
    }

    macro_rules! test_shr_assign_int {
        ($($name:ident: $ty:ty),* $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let mut uuid = UUID::from(16u128);
                    let n: $ty = 4 as $ty;
                    uuid >>= n;
                    assert_eq!(u128::from(uuid), 1);
                }
            )*
        };
    }

    test_shr_assign_int! {
        shr_assign_u8: u8,
        shr_assign_u16: u16,
        shr_assign_u32: u32,
        shr_assign_u64: u64,
        shr_assign_u128: u128,
        shr_assign_usize: usize,
        shr_assign_i8: i8,
        shr_assign_i16: i16,
        shr_assign_i32: i32,
        shr_assign_i64: i64,
        shr_assign_i128: i128,
        shr_assign_isize: isize,
    }
}

#[cfg(test)]
mod edge_cases {
    //! Tests for edge cases, boundary values, and large numbers.

    use crate::UUID;

    // =========================================================================
    // Near-Max Value Operations
    // =========================================================================

    #[test]
    fn operations_near_max() {
        let max = UUID::max();
        let near_max = UUID::from(u128::MAX - 10);

        // Addition wraps
        assert_eq!(max + 1u32, UUID::nil());
        assert_eq!(near_max + 20u32, UUID::from(9u128));

        // Subtraction works
        assert_eq!(max - 1u32, UUID::from(u128::MAX - 1));
        assert_eq!(near_max - 1u32, UUID::from(u128::MAX - 11));
    }

    #[test]
    fn overflow_patterns() {
        let max = UUID::max();

        // Various additions that overflow
        assert_eq!(max + 1u32, UUID::nil());
        assert_eq!(max + 2u32, UUID::from(1u128));
        assert_eq!(max + max, UUID::from(u128::MAX - 1));

        // Multiplication overflow
        let half = UUID::from(u128::MAX / 2 + 1);
        assert_eq!(half * 2u32, UUID::nil());
    }

    #[test]
    fn underflow_patterns() {
        let zero = UUID::nil();

        // Various subtractions that underflow
        assert_eq!(zero - 1u32, UUID::max());
        assert_eq!(zero - 2u32, UUID::from(u128::MAX - 1));

        let one = UUID::from(1u128);
        assert_eq!(one - 2u32, UUID::max());
    }

    // =========================================================================
    // Division Edge Cases
    // =========================================================================

    #[test]
    fn div_by_larger_uuid() {
        // small / large == 0
        let small = UUID::from(10u128);
        let large = UUID::from(100u128);
        assert_eq!(small / large, UUID::nil());
    }

    #[test]
    fn div_by_one() {
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid / 1u32, uuid, "div by one failed for {v}");
        }
    }

    #[test]
    fn div_max_by_various() {
        let max = UUID::max();
        assert_eq!(max / 2u32, UUID::from(u128::MAX / 2));
        assert_eq!(max / 3u32, UUID::from(u128::MAX / 3));
        assert_eq!(max / max, UUID::from(1u128));
    }

    #[test]
    fn div_equals_self() {
        let values = [1u128, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid / uuid, UUID::from(1u128), "div self failed for {v}");
        }
    }

    // =========================================================================
    // Remainder Edge Cases
    // =========================================================================

    #[test]
    #[allow(clippy::modulo_one)]
    fn rem_by_one() {
        // a % 1 == 0
        let values = [0u128, 1, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid % 1u32, UUID::nil(), "rem by one failed for {v}");
        }
    }

    #[test]
    fn rem_smaller_by_larger() {
        // small % large == small
        let small = UUID::from(10u128);
        let large = UUID::from(100u128);
        assert_eq!(small % large, small);
    }

    #[test]
    fn rem_equals_self() {
        // a % a == 0
        let values = [1u128, 42, 12345, u128::MAX / 2, u128::MAX];
        for v in values {
            let uuid = UUID::from(v);
            assert_eq!(uuid % uuid, UUID::nil(), "rem self failed for {v}");
        }
    }

    // =========================================================================
    // Shift Edge Cases
    // =========================================================================

    #[test]
    fn shift_by_127() {
        let one = UUID::from(1u128);
        let high_bit = one << 127u32;
        assert_eq!(u128::from(high_bit), 1u128 << 127);
        assert_eq!(high_bit >> 127u32, one);
    }

    #[test]
    fn shift_wraps_at_128() {
        let uuid = UUID::from(0b1010_1010u128);
        // Shift by 128 wraps to shift by 0
        assert_eq!(uuid << 128u32, uuid);
        assert_eq!(uuid >> 128u32, uuid);
    }

    #[test]
    fn shift_by_large_values() {
        let uuid = UUID::from(0b1010_1010u128);
        // 256 % 128 == 0, so this is like shifting by 0
        assert_eq!(uuid << 256u32, uuid);
        // 129 % 128 == 1
        assert_eq!(uuid << 129u32, uuid << 1u32);
    }

    #[test]
    fn shl_shifts_out_bits() {
        let max = UUID::max();
        // Shifting left loses high bits
        assert_eq!(max << 1u32, UUID::from(u128::MAX << 1));
        assert_eq!(max << 64u32, UUID::from(u128::MAX << 64));
        assert_eq!(max << 127u32, UUID::from(1u128 << 127));
    }

    #[test]
    fn shr_shifts_in_zeros() {
        let max = UUID::max();
        // Shifting right brings in zeros (logical shift)
        assert_eq!(max >> 1u32, UUID::from(u128::MAX >> 1));
        assert_eq!(max >> 64u32, UUID::from(u128::MAX >> 64));
        assert_eq!(max >> 127u32, UUID::from(1u128));
    }

    // =========================================================================
    // Signed Operand Edge Cases
    // =========================================================================

    #[test]
    fn sign_extension_all_types() {
        let uuid = UUID::from(100u128);

        // -1 sign-extends to all 1s (u128::MAX)
        // So uuid + (-1) should wrap around to uuid - 1
        assert_eq!(uuid + (-1i8), UUID::from(99u128));
        assert_eq!(uuid + (-1i16), UUID::from(99u128));
        assert_eq!(uuid + (-1i32), UUID::from(99u128));
        assert_eq!(uuid + (-1i64), UUID::from(99u128));
        assert_eq!(uuid + (-1i128), UUID::from(99u128));
        assert_eq!(uuid + (-1isize), UUID::from(99u128));
    }

    #[test]
    fn negative_operands_sub() {
        let uuid = UUID::from(100u128);
        // Subtracting -1 (all 1s) wraps around to add 1
        assert_eq!(uuid - (-1i32), UUID::from(101u128));
    }

    #[test]
    fn negative_operands_bitwise() {
        let uuid = UUID::from(0b1010_1010u128);

        // AND with -1 (all 1s) is identity
        assert_eq!(uuid & (-1i8), uuid);

        // OR with -1 (all 1s) is all 1s
        assert_eq!(uuid | (-1i8), UUID::max());

        // XOR with -1 (all 1s) is NOT
        assert_eq!(uuid ^ (-1i8), !uuid);
    }

    // =========================================================================
    // Large Value Operations
    // =========================================================================

    #[test]
    fn large_value_add() {
        let a = UUID::from(u128::MAX / 2);
        let b = UUID::from(u128::MAX / 3);
        // Just verify it doesn't panic and produces a result
        let _ = a + b;
    }

    #[test]
    fn large_value_mul() {
        let a = UUID::from(u128::from(u64::MAX));
        let b = UUID::from(2u128);
        assert_eq!(a * b, UUID::from(u128::from(u64::MAX) * 2));
    }

    #[test]
    fn large_value_div() {
        let max = UUID::max();
        // Division of max by small values
        for divisor in [2u32, 3, 7, 11, 13, 17, 19, 23] {
            let expected = u128::MAX / u128::from(divisor);
            assert_eq!(
                max / divisor,
                UUID::from(expected),
                "div by {divisor} failed"
            );
        }
    }
}

#[cfg(test)]
mod reference_variants {
    //! Tests ensuring all reference combinations work for all ops.

    #![allow(clippy::op_ref)]

    use crate::UUID;

    // =========================================================================
    // Sub Reference Variants
    // =========================================================================

    #[test]
    fn sub_all_ref_variants_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(30u128);
        let expected = UUID::from(70u128);

        assert_eq!(a - b, expected);
        assert_eq!(a - &b, expected);
        assert_eq!(&a - b, expected);
        assert_eq!(&a - &b, expected);
    }

    #[test]
    fn sub_all_ref_variants_int() {
        let a = UUID::from(100u128);
        let n = 30u32;
        let expected = UUID::from(70u128);

        assert_eq!(a - n, expected);
        assert_eq!(&a - n, expected);
        assert_eq!(a - &n, expected);
        assert_eq!(&a - &n, expected);
    }

    #[test]
    fn sub_int_uuid_all_ref_variants() {
        let uuid = UUID::from(30u128);
        let n = 100u32;
        let expected = UUID::from(70u128);

        assert_eq!(n - uuid, expected);
        assert_eq!(n - &uuid, expected);
        assert_eq!(&n - uuid, expected);
        assert_eq!(&n - &uuid, expected);
    }

    // =========================================================================
    // Div Reference Variants
    // =========================================================================

    #[test]
    fn div_all_ref_variants_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(10u128);
        let expected = UUID::from(10u128);

        assert_eq!(a / b, expected);
        assert_eq!(a / &b, expected);
        assert_eq!(&a / b, expected);
        assert_eq!(&a / &b, expected);
    }

    #[test]
    fn div_all_ref_variants_int() {
        let a = UUID::from(100u128);
        let n = 10u32;
        let expected = UUID::from(10u128);

        assert_eq!(a / n, expected);
        assert_eq!(&a / n, expected);
        assert_eq!(a / &n, expected);
        assert_eq!(&a / &n, expected);
    }

    #[test]
    fn div_int_uuid_all_ref_variants() {
        let uuid = UUID::from(10u128);
        let n = 100u32;
        let expected = UUID::from(10u128);

        assert_eq!(n / uuid, expected);
        assert_eq!(n / &uuid, expected);
        assert_eq!(&n / uuid, expected);
        assert_eq!(&n / &uuid, expected);
    }

    // =========================================================================
    // Rem Reference Variants
    // =========================================================================

    #[test]
    fn rem_all_ref_variants_uuid() {
        let a = UUID::from(100u128);
        let b = UUID::from(30u128);
        let expected = UUID::from(10u128);

        assert_eq!(a % b, expected);
        assert_eq!(a % &b, expected);
        assert_eq!(&a % b, expected);
        assert_eq!(&a % &b, expected);
    }

    #[test]
    fn rem_all_ref_variants_int() {
        let a = UUID::from(100u128);
        let n = 30u32;
        let expected = UUID::from(10u128);

        assert_eq!(a % n, expected);
        assert_eq!(&a % n, expected);
        assert_eq!(a % &n, expected);
        assert_eq!(&a % &n, expected);
    }

    #[test]
    fn rem_int_uuid_all_ref_variants() {
        let uuid = UUID::from(30u128);
        let n = 100u32;
        let expected = UUID::from(10u128);

        assert_eq!(n % uuid, expected);
        assert_eq!(n % &uuid, expected);
        assert_eq!(&n % uuid, expected);
        assert_eq!(&n % &uuid, expected);
    }

    // =========================================================================
    // BitOr Reference Variants
    // =========================================================================

    #[test]
    fn bitor_all_ref_variants_uuid() {
        let a = UUID::from(0b1010_0000u128);
        let b = UUID::from(0b0000_1010u128);
        let expected = UUID::from(0b1010_1010u128);

        assert_eq!(a | b, expected);
        assert_eq!(a | &b, expected);
        assert_eq!(&a | b, expected);
        assert_eq!(&a | &b, expected);
    }

    #[test]
    fn bitor_all_ref_variants_int() {
        let a = UUID::from(0b1010_0000u128);
        let n = 0b0000_1010u32;
        let expected = UUID::from(0b1010_1010u128);

        assert_eq!(a | n, expected);
        assert_eq!(&a | n, expected);
        assert_eq!(a | &n, expected);
        assert_eq!(&a | &n, expected);
    }

    #[test]
    fn bitor_int_uuid_all_ref_variants() {
        let uuid = UUID::from(0b1010_0000u128);
        let n = 0b0000_1010u32;
        let expected = UUID::from(0b1010_1010u128);

        assert_eq!(n | uuid, expected);
        assert_eq!(n | &uuid, expected);
        assert_eq!(&n | uuid, expected);
        assert_eq!(&n | &uuid, expected);
    }

    // =========================================================================
    // BitXor Reference Variants
    // =========================================================================

    #[test]
    fn bitxor_all_ref_variants_uuid() {
        let a = UUID::from(0b1010_1010u128);
        let b = UUID::from(0b1111_0000u128);
        let expected = UUID::from(0b0101_1010u128);

        assert_eq!(a ^ b, expected);
        assert_eq!(a ^ &b, expected);
        assert_eq!(&a ^ b, expected);
        assert_eq!(&a ^ &b, expected);
    }

    #[test]
    fn bitxor_all_ref_variants_int() {
        let a = UUID::from(0b1010_1010u128);
        let n = 0b1111_0000u32;
        let expected = UUID::from(0b0101_1010u128);

        assert_eq!(a ^ n, expected);
        assert_eq!(&a ^ n, expected);
        assert_eq!(a ^ &n, expected);
        assert_eq!(&a ^ &n, expected);
    }

    #[test]
    fn bitxor_int_uuid_all_ref_variants() {
        let uuid = UUID::from(0b1010_1010u128);
        let n = 0b1111_0000u32;
        let expected = UUID::from(0b0101_1010u128);

        assert_eq!(n ^ uuid, expected);
        assert_eq!(n ^ &uuid, expected);
        assert_eq!(&n ^ uuid, expected);
        assert_eq!(&n ^ &uuid, expected);
    }

    // =========================================================================
    // Shift Reference Variants
    // =========================================================================

    #[test]
    fn shl_all_ref_variants() {
        let uuid = UUID::from(1u128);
        let n = 4u32;
        let expected = UUID::from(16u128);

        assert_eq!(uuid << n, expected);
        assert_eq!(&uuid << n, expected);
        assert_eq!(uuid << &n, expected);
        assert_eq!(&uuid << &n, expected);
    }

    #[test]
    fn shr_all_ref_variants() {
        let uuid = UUID::from(16u128);
        let n = 4u32;
        let expected = UUID::from(1u128);

        assert_eq!(uuid >> n, expected);
        assert_eq!(&uuid >> n, expected);
        assert_eq!(uuid >> &n, expected);
        assert_eq!(&uuid >> &n, expected);
    }

    // =========================================================================
    // Neg Reference Variants
    // =========================================================================

    #[test]
    fn neg_all_ref_variants() {
        let uuid = UUID::from(1u128);
        let expected = UUID::max();

        assert_eq!(-uuid, expected);
        assert_eq!(-&uuid, expected);
    }

    // =========================================================================
    // Not Reference Variants
    // =========================================================================

    #[test]
    fn not_all_ref_variants() {
        let uuid = UUID::nil();
        let expected = UUID::max();

        assert_eq!(!uuid, expected);
        assert_eq!(!&uuid, expected);
    }
}

#[cfg(test)]
mod index_operations {
    //! Additional index operation tests.

    use crate::UUID;

    #[test]
    fn index_preserves_big_endian() {
        // Value with distinct bytes
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);

        // Verify big-endian layout
        assert_eq!(uuid[0], 0x01);
        assert_eq!(uuid[1], 0x02);
        assert_eq!(uuid[14], 0x0f);
        assert_eq!(uuid[15], 0x10);
    }

    #[test]
    fn index_mut_affects_value() {
        let mut uuid = UUID::nil();
        uuid[0] = 0x12;
        uuid[15] = 0x34;

        // Check via indexing
        assert_eq!(uuid[0], 0x12);
        assert_eq!(uuid[15], 0x34);

        // Check via u128 conversion (big-endian)
        let val = u128::from(uuid);
        assert_eq!(val >> 120, 0x12);
        assert_eq!(val & 0xFF, 0x34);
    }

    #[test]
    fn range_index_slices() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);

        // Various range types
        assert_eq!(&uuid[0..4], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[12..], &[0x0d, 0x0e, 0x0f, 0x10]);
        assert_eq!(&uuid[..4], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[0..=3], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[..=3], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(uuid[..].len(), 16);
    }
}
