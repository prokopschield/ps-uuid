//! Comprehensive tests for `num_traits` implementations.
//!
//! These tests verify that UUID correctly implements all `num_traits` traits
//! and that the implementations behave consistently with the underlying u128.

use num_traits::bounds::{LowerBounded, UpperBounded};
use num_traits::ops::bytes::NumBytes;
use num_traits::ops::euclid::CheckedEuclid;
use num_traits::ops::overflowing::{OverflowingAdd, OverflowingMul, OverflowingSub};
use num_traits::{
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedShl,
    CheckedShr, CheckedSub, ConstOne, ConstZero, Euclid, FromBytes, FromPrimitive, Inv, MulAdd,
    MulAddAssign, Num, NumAssign, NumAssignRef, NumRef, One, Pow, PrimInt, RefNum, Saturating,
    SaturatingAdd, SaturatingMul, SaturatingSub, ToBytes, ToPrimitive, Unsigned, WrappingAdd,
    WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, Zero,
};

use crate::UUID;

// ============================================================================
// Compile-time trait bound verification
// ============================================================================

/// Verifies at compile time that UUID implements all expected `num_traits`.
fn assert_num_traits_impls<T>()
where
    T: PrimInt
        + Unsigned
        + Num
        + num_traits::NumCast
        + AsPrimitive<u8>
        + AsPrimitive<u16>
        + AsPrimitive<u32>
        + AsPrimitive<u64>
        + AsPrimitive<u128>
        + AsPrimitive<usize>
        + AsPrimitive<i8>
        + AsPrimitive<i16>
        + AsPrimitive<i32>
        + AsPrimitive<i64>
        + AsPrimitive<i128>
        + AsPrimitive<isize>
        + AsPrimitive<f32>
        + AsPrimitive<f64>
        + AsPrimitive<T>
        + NumRef
        + NumAssign
        + NumAssignRef
        + LowerBounded
        + UpperBounded
        + ToPrimitive
        + FromPrimitive
        + CheckedAdd
        + CheckedSub
        + CheckedMul
        + CheckedDiv
        + CheckedRem
        + CheckedNeg
        + CheckedShl
        + CheckedShr
        + Saturating
        + SaturatingAdd
        + SaturatingSub
        + SaturatingMul
        + WrappingAdd
        + WrappingSub
        + WrappingMul
        + WrappingNeg
        + WrappingShl
        + WrappingShr
        + OverflowingAdd
        + OverflowingSub
        + OverflowingMul
        + Euclid
        + CheckedEuclid
        + ToBytes<Bytes = [u8; 16]>
        + FromBytes<Bytes = [u8; 16]>
        + MulAdd<Output = T>
        + MulAddAssign
        + MulAddAssign<T, T>
        + for<'a> MulAddAssign<&'a T, &'a T>
        + Pow<u8, Output = T>
        + Pow<u16, Output = T>
        + Pow<u32, Output = T>
        + Pow<usize, Output = T>
        + for<'a> Pow<&'a u8, Output = T>
        + for<'a> Pow<&'a u16, Output = T>
        + for<'a> Pow<&'a u32, Output = T>
        + for<'a> Pow<&'a usize, Output = T>
        + Inv<Output = Option<T>>
        + Copy,
    for<'a> &'a T: RefNum<T>
        + Pow<u8, Output = T>
        + Pow<u16, Output = T>
        + Pow<u32, Output = T>
        + Pow<usize, Output = T>
        + Pow<&'a u8, Output = T>
        + Pow<&'a u16, Output = T>
        + Pow<&'a u32, Output = T>
        + Pow<&'a usize, Output = T>,
{
}

fn assert_bytes_impl<T>()
where
    T: ToBytes<Bytes = [u8; 16]> + FromBytes<Bytes = [u8; 16]>,
    <T as ToBytes>::Bytes: NumBytes,
{
}

#[test]
fn implements_all_supported_num_traits() {
    assert_num_traits_impls::<UUID>();
    assert_bytes_impl::<UUID>();
}

// ============================================================================
// Identity and bounds tests
// ============================================================================

#[test]
fn identities_and_bounds_match_uuid_helpers() {
    let zero = UUID::zero();
    let one = UUID::one();

    assert_eq!(zero, UUID::nil());
    assert_eq!(one, UUID::from(1u8));
    assert!(zero.is_zero());
    assert!(!one.is_zero());
    assert!(one.is_one());
    assert!(!zero.is_one());
    assert_eq!(<UUID as ConstZero>::ZERO, UUID::nil());
    assert_eq!(<UUID as ConstOne>::ONE, UUID::from(1u8));
    assert_eq!(<UUID as Bounded>::min_value(), UUID::nil());
    assert_eq!(<UUID as Bounded>::max_value(), UUID::max());
    assert_eq!(<UUID as LowerBounded>::min_value(), UUID::nil());
    assert_eq!(<UUID as UpperBounded>::max_value(), UUID::max());
}

// ============================================================================
// Numeric conversion tests
// ============================================================================

#[test]
fn num_to_primitive_from_primitive_and_num_cast_work() {
    let parsed = match <UUID as Num>::from_str_radix("ff", 16) {
        Ok(parsed) => parsed,
        Err(error) => panic!("valid hexadecimal u128 should parse: {error}"),
    };
    assert_eq!(parsed, UUID::from(255u16));
    assert!(<UUID as Num>::from_str_radix("zz", 10).is_err());

    let parsed = match <UUID as Num>::from_str_radix("ffffffffffffffffffffffffffffffff", 16) {
        Ok(parsed) => parsed,
        Err(error) => panic!("valid hexadecimal u128 should parse: {error}"),
    };

    assert_eq!(parsed, UUID::max());
    assert_eq!(<UUID as ToPrimitive>::to_u128(&parsed), Some(u128::MAX));
    assert_eq!(UUID::from(42u8).to_i64(), Some(42i64));
    assert_eq!(UUID::max().to_i64(), None);
    assert_eq!(
        <UUID as FromPrimitive>::from_u128(u128::MAX),
        Some(UUID::max())
    );
    assert_eq!(
        <UUID as FromPrimitive>::from_u64(255u64),
        Some(UUID::from(255u16))
    );
    assert_eq!(
        <UUID as FromPrimitive>::from_f32(42.0f32),
        Some(UUID::from(42u8))
    );
    assert_eq!(
        <UUID as num_traits::NumCast>::from(255u16),
        Some(UUID::from(255u16))
    );
    assert_eq!(<UUID as num_traits::NumCast>::from(-1i8), None);
    assert_eq!(<UUID as FromPrimitive>::from_i64(-1), None);
}

#[test]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::float_cmp
)]
fn as_primitive_matches_u128_as_cast_semantics() {
    let value = UUID::from_u128(0x1234_5678_90ab_cdef_0011_2233_4455_6677u128);

    assert_eq!(<UUID as AsPrimitive<u8>>::as_(value), value.to_u128() as u8);
    assert_eq!(<UUID as AsPrimitive<u128>>::as_(value), value.to_u128());
    assert_eq!(
        <UUID as AsPrimitive<i128>>::as_(value),
        value.to_u128().cast_signed()
    );
    assert_eq!(
        <UUID as AsPrimitive<f64>>::as_(value),
        value.to_u128() as f64
    );
    assert_eq!(<UUID as AsPrimitive<UUID>>::as_(value), value);
}

// ============================================================================
// Checked operations tests
// ============================================================================

#[test]
fn checked_trait_methods_cover_every_checked_trait() {
    let one = UUID::one();
    let two = UUID::from(2u8);
    let three = UUID::from(3u8);
    let max = UUID::max();

    assert_eq!(
        <UUID as CheckedAdd>::checked_add(&two, &three),
        Some(UUID::from(5u8))
    );
    assert_eq!(<UUID as CheckedSub>::checked_sub(&three, &two), Some(one));
    assert_eq!(
        <UUID as CheckedMul>::checked_mul(&three, &two),
        Some(UUID::from(6u8))
    );
    assert_eq!(<UUID as CheckedDiv>::checked_div(&three, &two), Some(one));
    assert_eq!(<UUID as CheckedRem>::checked_rem(&three, &two), Some(one));
    assert_eq!(
        <UUID as CheckedNeg>::checked_neg(&UUID::zero()),
        Some(UUID::zero())
    );
    assert_eq!(<UUID as CheckedShl>::checked_shl(&one, 1), Some(two));
    assert_eq!(<UUID as CheckedShr>::checked_shr(&two, 1), Some(one));

    assert_eq!(<UUID as CheckedAdd>::checked_add(&max, &one), None);
    assert_eq!(<UUID as CheckedSub>::checked_sub(&UUID::zero(), &one), None);
    assert_eq!(<UUID as CheckedMul>::checked_mul(&max, &two), None);
    assert_eq!(
        <UUID as CheckedDiv>::checked_div(&three, &UUID::zero()),
        None
    );
    assert_eq!(
        <UUID as CheckedRem>::checked_rem(&three, &UUID::zero()),
        None
    );
    assert_eq!(<UUID as CheckedNeg>::checked_neg(&one), None);
    assert_eq!(<UUID as CheckedShl>::checked_shl(&one, 128), None);
    assert_eq!(<UUID as CheckedShr>::checked_shr(&one, 128), None);
}

// ============================================================================
// Saturating operations tests
// ============================================================================

#[test]
fn saturating_trait_methods_cover_every_saturating_trait() {
    let one = UUID::one();
    let two = UUID::from(2u8);
    let max = UUID::max();

    assert_eq!(<UUID as Saturating>::saturating_add(max, one), max);
    assert_eq!(
        <UUID as Saturating>::saturating_sub(UUID::zero(), one),
        UUID::zero()
    );
    assert_eq!(<UUID as SaturatingAdd>::saturating_add(&max, &one), max);
    assert_eq!(
        <UUID as SaturatingSub>::saturating_sub(&UUID::zero(), &one),
        UUID::zero()
    );
    assert_eq!(<UUID as SaturatingMul>::saturating_mul(&max, &two), max);
}

// ============================================================================
// Wrapping operations tests
// ============================================================================

#[test]
fn wrapping_trait_methods_cover_every_wrapping_trait() {
    let one = UUID::one();
    let two = UUID::from(2u8);
    let max = UUID::max();

    assert_eq!(<UUID as WrappingAdd>::wrapping_add(&max, &one), UUID::nil());
    assert_eq!(
        <UUID as WrappingSub>::wrapping_sub(&UUID::zero(), &one),
        max
    );
    assert_eq!(
        <UUID as WrappingMul>::wrapping_mul(&max, &two),
        UUID::from_u128(u128::MAX.wrapping_mul(2))
    );
    assert_eq!(<UUID as WrappingNeg>::wrapping_neg(&one), max);
    assert_eq!(<UUID as WrappingShl>::wrapping_shl(&one, 128), one);
    assert_eq!(<UUID as WrappingShr>::wrapping_shr(&one, 128), one);
}

// ============================================================================
// Overflowing operations tests
// ============================================================================

#[test]
fn overflowing_trait_methods_cover_every_overflowing_trait() {
    let one = UUID::one();
    let two = UUID::from(2u8);
    let max = UUID::max();

    assert_eq!(
        <UUID as OverflowingAdd>::overflowing_add(&max, &one),
        (UUID::nil(), true)
    );
    assert_eq!(
        <UUID as OverflowingSub>::overflowing_sub(&UUID::zero(), &one),
        (max, true)
    );
    assert_eq!(
        <UUID as OverflowingMul>::overflowing_mul(&max, &two),
        (UUID::from_u128(u128::MAX.wrapping_mul(2)), true)
    );
}

// ============================================================================
// Euclidean operations tests
// ============================================================================

#[test]
fn euclid_trait_methods_cover_every_euclid_trait() {
    let ten = UUID::from(10u8);
    let three = UUID::from(3u8);
    let zero = UUID::zero();

    assert_eq!(<UUID as Euclid>::div_euclid(&ten, &three), UUID::from(3u8));
    assert_eq!(<UUID as Euclid>::rem_euclid(&ten, &three), UUID::from(1u8));
    assert_eq!(
        <UUID as CheckedEuclid>::checked_div_euclid(&ten, &three),
        Some(UUID::from(3u8))
    );
    assert_eq!(
        <UUID as CheckedEuclid>::checked_rem_euclid(&ten, &three),
        Some(UUID::from(1u8))
    );
    assert_eq!(
        <UUID as CheckedEuclid>::checked_div_euclid(&ten, &zero),
        None
    );
    assert_eq!(
        <UUID as CheckedEuclid>::checked_rem_euclid(&ten, &zero),
        None
    );
}

// ============================================================================
// PrimInt tests
// ============================================================================

#[test]
fn primint_methods_match_underlying_u128_behavior() {
    let x = UUID::from_u128(0xfedc_ba98_7654_3210_0123_4567_89ab_cdefu128);
    let one = UUID::one();

    assert_eq!(x.count_ones(), x.to_u128().count_ones());
    assert_eq!(x.count_zeros(), x.to_u128().count_zeros());
    assert_eq!(x.leading_ones(), x.to_u128().leading_ones());
    assert_eq!(x.leading_zeros(), x.to_u128().leading_zeros());
    assert_eq!(x.trailing_ones(), x.to_u128().trailing_ones());
    assert_eq!(x.trailing_zeros(), x.to_u128().trailing_zeros());
    assert_eq!(
        <UUID as PrimInt>::rotate_left(x, 17),
        UUID::from_u128(x.to_u128().rotate_left(17))
    );
    assert_eq!(
        <UUID as PrimInt>::rotate_right(x, 17),
        UUID::from_u128(x.to_u128().rotate_right(17))
    );
    assert_eq!(
        <UUID as PrimInt>::signed_shl(x, 4),
        UUID::from((i128::from(x) << 4).cast_unsigned())
    );
    assert_eq!(
        <UUID as PrimInt>::signed_shr(x, 4),
        UUID::from((i128::from(x) >> 4).cast_unsigned())
    );
    assert_eq!(
        <UUID as PrimInt>::unsigned_shl(x, 4),
        UUID::from_u128(x.to_u128() << 4)
    );
    assert_eq!(
        <UUID as PrimInt>::unsigned_shr(x, 4),
        UUID::from_u128(x.to_u128() >> 4)
    );
    assert_eq!(
        <UUID as PrimInt>::swap_bytes(x),
        UUID::from_u128(x.to_u128().swap_bytes())
    );
    assert_eq!(
        <UUID as PrimInt>::reverse_bits(x),
        UUID::from_u128(x.to_u128().reverse_bits())
    );
    assert_eq!(
        <UUID as PrimInt>::from_be(x),
        UUID::from_u128(u128::from_be(x.to_u128()))
    );
    assert_eq!(
        <UUID as PrimInt>::from_le(x),
        UUID::from_u128(u128::from_le(x.to_u128()))
    );
    assert_eq!(
        <UUID as PrimInt>::to_be(x),
        UUID::from_u128(x.to_u128().to_be())
    );
    assert_eq!(
        <UUID as PrimInt>::to_le(x),
        UUID::from_u128(x.to_u128().to_le())
    );
    assert_eq!(<UUID as PrimInt>::pow(one + one, 8), UUID::from(256u16));
}

// ============================================================================
// Pow tests
// ============================================================================

#[test]
fn pow_trait_supports_every_rhs_variant() {
    let two = UUID::from(2u8);
    let exp_u8 = 8u8;
    let exp_u16 = 8u16;
    let exp_u32 = 8u32;
    let exp_usize = 8usize;
    let expected = UUID::from(256u16);

    assert_eq!(<UUID as Pow<u8>>::pow(two, exp_u8), expected);
    assert_eq!(<UUID as Pow<&u8>>::pow(two, &exp_u8), expected);
    assert_eq!(<&UUID as Pow<u8>>::pow(&two, exp_u8), expected);
    assert_eq!(<&UUID as Pow<&u8>>::pow(&two, &exp_u8), expected);

    assert_eq!(<UUID as Pow<u16>>::pow(two, exp_u16), expected);
    assert_eq!(<UUID as Pow<&u16>>::pow(two, &exp_u16), expected);
    assert_eq!(<&UUID as Pow<u16>>::pow(&two, exp_u16), expected);
    assert_eq!(<&UUID as Pow<&u16>>::pow(&two, &exp_u16), expected);

    assert_eq!(<UUID as Pow<u32>>::pow(two, exp_u32), expected);
    assert_eq!(<UUID as Pow<&u32>>::pow(two, &exp_u32), expected);
    assert_eq!(<&UUID as Pow<u32>>::pow(&two, exp_u32), expected);
    assert_eq!(<&UUID as Pow<&u32>>::pow(&two, &exp_u32), expected);

    assert_eq!(<UUID as Pow<usize>>::pow(two, exp_usize), expected);
    assert_eq!(<UUID as Pow<&usize>>::pow(two, &exp_usize), expected);
    assert_eq!(<&UUID as Pow<usize>>::pow(&two, exp_usize), expected);
    assert_eq!(<&UUID as Pow<&usize>>::pow(&two, &exp_usize), expected);
}

// ============================================================================
// MulAdd tests
// ============================================================================

#[test]
fn mul_add_and_mul_add_assign_use_uuid_numeric_semantics() {
    let two = UUID::from(2u8);
    let three = UUID::from(3u8);
    let four = UUID::from(4u8);
    assert_eq!(
        <UUID as MulAdd>::mul_add(two, three, four),
        UUID::from(10u8)
    );

    let mut value = two;
    <UUID as MulAddAssign>::mul_add_assign(&mut value, three, four);
    assert_eq!(value, UUID::from(10u8));

    let mut value = two;
    <UUID as MulAddAssign<&UUID, &UUID>>::mul_add_assign(&mut value, &three, &four);
    assert_eq!(value, UUID::from(10u8));
}

// ============================================================================
// Byte conversion tests
// ============================================================================

#[test]
fn to_bytes_and_from_bytes_cover_big_little_and_native_endian() {
    let value = UUID::from_u128(0x0011_2233_4455_6677_8899_aabb_ccdd_eeffu128);

    let be = <UUID as ToBytes>::to_be_bytes(&value);
    let le = <UUID as ToBytes>::to_le_bytes(&value);
    let ne = <UUID as ToBytes>::to_ne_bytes(&value);

    assert_eq!(be, *value.as_bytes());
    assert_eq!(<UUID as FromBytes>::from_be_bytes(&be), value);
    assert_eq!(<UUID as FromBytes>::from_le_bytes(&le), value);
    assert_eq!(<UUID as FromBytes>::from_ne_bytes(&ne), value);
}

// ============================================================================
// Standard operator tests (verifies core::ops integration)
// ============================================================================

#[test]
fn num_ops_num_ref_ref_num_and_num_assign_ref_work() {
    let lhs = UUID::from(20u8);
    let rhs = UUID::from(3u8);

    // Owned + Owned
    assert_eq!(lhs + rhs, UUID::from(23u8));
    assert_eq!(lhs - rhs, UUID::from(17u8));
    assert_eq!(lhs * rhs, UUID::from(60u8));
    assert_eq!(lhs / rhs, UUID::from(6u8));
    assert_eq!(lhs % rhs, UUID::from(2u8));

    // Ref + Ref
    assert_eq!(lhs + rhs, UUID::from(23u8));
    assert_eq!(lhs - rhs, UUID::from(17u8));
    assert_eq!(lhs * rhs, UUID::from(60u8));
    assert_eq!(lhs / rhs, UUID::from(6u8));
    assert_eq!(lhs % rhs, UUID::from(2u8));

    // Compound assignment with ref
    let mut value = UUID::from(10u8);
    value += &rhs;
    assert_eq!(value, UUID::from(13u8));
    value -= &rhs;
    assert_eq!(value, UUID::from(10u8));
    value *= &rhs;
    assert_eq!(value, UUID::from(30u8));
    value /= &rhs;
    assert_eq!(value, UUID::from(10u8));
    value %= &rhs;
    assert_eq!(value, UUID::from(1u8));

    // Bitwise operations
    let bits_lhs = UUID::from(0b1010u8);
    let bits_rhs = UUID::from(0b1100u8);
    assert_eq!(bits_lhs & bits_rhs, UUID::from(0b1000u8));
    assert_eq!(bits_lhs | bits_rhs, UUID::from(0b1110u8));
    assert_eq!(bits_lhs ^ bits_rhs, UUID::from(0b0110u8));
    assert_eq!(!UUID::zero(), UUID::max());
    assert_eq!(!&UUID::zero(), UUID::max());

    // Shift operations
    assert_eq!(bits_lhs << 2usize, UUID::from(0b10_1000u8));
    assert_eq!(bits_lhs << 2u32, UUID::from(0b10_1000u8));
    assert_eq!(bits_lhs >> 1usize, UUID::from(0b0101u8));
    assert_eq!(bits_lhs >> 1u32, UUID::from(0b0101u8));
}

// ============================================================================
// Bitwise assign operators tests
// ============================================================================

#[test]
fn bitwise_assign_operators_work() {
    let mut value = UUID::from(0b1010u8);

    value &= UUID::from(0b1100u8);
    assert_eq!(value, UUID::from(0b1000u8));

    value |= UUID::from(0b0011u8);
    assert_eq!(value, UUID::from(0b1011u8));

    value ^= UUID::from(0b1111u8);
    assert_eq!(value, UUID::from(0b0100u8));

    // With references
    let mask = UUID::from(0b1111u8);
    value &= &mask;
    assert_eq!(value, UUID::from(0b0100u8));
}

// ============================================================================
// Shift assign operators tests
// ============================================================================

#[test]
fn shift_assign_operators_work() {
    let mut value = UUID::from(0b0001u8);

    value <<= 4u32;
    assert_eq!(value, UUID::from(0b1_0000u8));

    value >>= 2usize;
    assert_eq!(value, UUID::from(0b0100u8));

    // Various shift amount types
    value <<= 1u8;
    assert_eq!(value, UUID::from(0b1000u8));

    value >>= 1i32;
    assert_eq!(value, UUID::from(0b0100u8));
}

// ============================================================================
// Mixed-type operations tests (UUID op int, int op UUID)
// ============================================================================

#[test]
fn uuid_op_int_arithmetic_works() {
    let uuid = UUID::from(100u8);

    // UUID + int
    assert_eq!(uuid + 50u8, UUID::from(150u8));
    assert_eq!(uuid + 50u32, UUID::from(150u8));
    assert_eq!(uuid + 50u128, UUID::from(150u8));
    assert_eq!(uuid + 50i32, UUID::from(150u8));

    // UUID - int
    assert_eq!(uuid - 30u8, UUID::from(70u8));
    assert_eq!(uuid - 30i64, UUID::from(70u8));

    // UUID * int
    assert_eq!(uuid * 2u8, UUID::from(200u8));
    assert_eq!(uuid * 3u64, UUID::from(300u16));

    // UUID / int
    assert_eq!(uuid / 10u8, UUID::from(10u8));
    assert_eq!(uuid / 25u32, UUID::from(4u8));

    // UUID % int
    assert_eq!(uuid % 30u8, UUID::from(10u8));
    assert_eq!(uuid % 7i32, UUID::from(2u8));
}

#[test]
fn int_op_uuid_arithmetic_works() {
    let uuid = UUID::from(10u8);

    // int + UUID -> UUID
    assert_eq!(5u8 + uuid, UUID::from(15u8));
    assert_eq!(100u32 + uuid, UUID::from(110u8));

    // int - UUID -> UUID
    assert_eq!(100u8 - uuid, UUID::from(90u8));
    assert_eq!(50u64 - uuid, UUID::from(40u8));

    // int * UUID -> UUID
    assert_eq!(5u8 * uuid, UUID::from(50u8));
    assert_eq!(3u32 * uuid, UUID::from(30u8));

    // int / UUID -> UUID
    assert_eq!(100u8 / uuid, UUID::from(10u8));

    // int % UUID -> UUID
    assert_eq!(25u8 % uuid, UUID::from(5u8));
}

#[test]
fn uuid_op_int_bitwise_works() {
    let uuid = UUID::from(0b1111_0000u8);

    // UUID & int
    assert_eq!(uuid & 0b1010_1010u8, UUID::from(0b1010_0000u8));
    assert_eq!(uuid & 0b0000_1111u32, UUID::from(0u8));

    // UUID | int
    assert_eq!(uuid | 0b0000_1111u8, UUID::from(0b1111_1111u8));

    // UUID ^ int
    assert_eq!(uuid ^ 0b1111_1111u8, UUID::from(0b0000_1111u8));
}

#[test]
fn int_op_uuid_bitwise_works() {
    let uuid = UUID::from(0b1111_0000u8);

    // int & UUID -> UUID
    assert_eq!(0b1010_1010u8 & uuid, UUID::from(0b1010_0000u8));

    // int | UUID -> UUID
    assert_eq!(0b0000_1111u8 | uuid, UUID::from(0b1111_1111u8));

    // int ^ UUID -> UUID
    assert_eq!(0b1111_1111u8 ^ uuid, UUID::from(0b0000_1111u8));
}

#[test]
fn uuid_op_int_compound_assignment_works() {
    let mut uuid = UUID::from(100u8);

    uuid += 10u32;
    assert_eq!(uuid, UUID::from(110u8));

    uuid -= 20u64;
    assert_eq!(uuid, UUID::from(90u8));

    uuid *= 2u8;
    assert_eq!(uuid, UUID::from(180u8));

    uuid /= 3u16;
    assert_eq!(uuid, UUID::from(60u8));

    uuid %= 7i32;
    assert_eq!(uuid, UUID::from(4u8));

    // Bitwise compound assignment
    uuid = UUID::from(0b1111_0000u8);
    uuid &= 0b1010_1010u8;
    assert_eq!(uuid, UUID::from(0b1010_0000u8));

    uuid |= 0b0000_1111u8;
    assert_eq!(uuid, UUID::from(0b1010_1111u8));

    uuid ^= 0b1111_1111u8;
    assert_eq!(uuid, UUID::from(0b0101_0000u8));
}

#[test]
fn uuid_op_ref_int_works() {
    let uuid = UUID::from(100u8);
    let val = 50u32;

    // UUID op &int
    assert_eq!(uuid + val, UUID::from(150u8));
    assert_eq!(uuid - val, UUID::from(50u8));
    assert_eq!(uuid * 2u8, UUID::from(200u8));

    // &UUID op int
    assert_eq!(&uuid + 10u8, UUID::from(110u8));
    assert_eq!(&uuid - 10u8, UUID::from(90u8));
}

// ============================================================================
// Inv tests
// ============================================================================

#[test]
#[allow(clippy::expect_used)]
fn inv_returns_multiplicative_inverse_for_odd_values() {
    let x = UUID::from(3u8);
    let y = x.inv().expect("odd value has inverse");
    assert_eq!(x * y, UUID::one());
}

#[test]
fn inv_returns_none_for_even_values() {
    assert_eq!(UUID::from(0u8).inv(), None);
    assert_eq!(UUID::from(2u8).inv(), None);
    assert_eq!(UUID::from(4u8).inv(), None);
}
