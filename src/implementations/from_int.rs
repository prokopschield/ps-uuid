use std::num::TryFromIntError;

use crate::UUID;

// ---------------------------------------------------------------------------
// Into UUID: every integer widens to u128/i128, then big-endian encodes.
// ---------------------------------------------------------------------------

impl From<u128> for UUID {
    fn from(v: u128) -> Self {
        Self {
            bytes: v.to_be_bytes(),
        }
    }
}

impl From<i128> for UUID {
    fn from(v: i128) -> Self {
        Self::from(v.cast_unsigned())
    }
}

macro_rules! impl_from_small_unsigned {
    ($($t:ty),*) => { $(
        impl From<$t> for UUID {
            fn from(v: $t) -> Self {
                Self::from(u128::from(v))
            }
        }
    )* };
}

macro_rules! impl_from_small_signed {
    ($($t:ty),*) => { $(
        impl From<$t> for UUID {
            fn from(v: $t) -> Self {
                Self::from(i128::from(v))
            }
        }
    )* };
}

impl_from_small_unsigned!(u8, u16, u32, u64);
impl_from_small_signed!(i8, i16, i32, i64);

impl From<usize> for UUID {
    fn from(v: usize) -> Self {
        Self::from(v as u128)
    }
}

impl From<isize> for UUID {
    fn from(v: isize) -> Self {
        Self::from(v as i128)
    }
}

// ---------------------------------------------------------------------------
// From UUID: only 128-bit integers get infallible From.
// ---------------------------------------------------------------------------

impl From<UUID> for u128 {
    fn from(uuid: UUID) -> Self {
        Self::from_be_bytes(uuid.bytes)
    }
}

impl From<UUID> for i128 {
    fn from(uuid: UUID) -> Self {
        u128::from(uuid).cast_signed()
    }
}

// ---------------------------------------------------------------------------
// From UUID: smaller integers get TryFrom, delegating to TryFrom<u128>.
// ---------------------------------------------------------------------------

macro_rules! impl_try_from_uuid_unsigned {
    ($($t:ty),*) => { $(
        impl TryFrom<UUID> for $t {
            type Error = TryFromIntError;

            fn try_from(uuid: UUID) -> Result<Self, Self::Error> {
                <$t>::try_from(u128::from(uuid))
            }
        }
    )* };
}

macro_rules! impl_try_from_uuid_signed {
    ($($t:ty),*) => { $(
        impl TryFrom<UUID> for $t {
            type Error = TryFromIntError;

            fn try_from(uuid: UUID) -> Result<Self, Self::Error> {
                <$t>::try_from(i128::from(uuid))
            }
        }
    )* };
}

impl_try_from_uuid_unsigned!(u8, u16, u32, u64, usize);
impl_try_from_uuid_signed!(i8, i16, i32, i64, isize);

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // u128 <-> UUID
    // -----------------------------------------------------------------------

    #[test]
    fn u128_zero() {
        let uuid = UUID::from(0u128);
        assert_eq!(uuid, UUID::nil());
        assert_eq!(u128::from(uuid), 0);
    }

    #[test]
    fn u128_one() {
        let uuid = UUID::from(1u128);
        assert_eq!(u128::from(uuid), 1);
    }

    #[test]
    fn u128_max() {
        let uuid = UUID::from(u128::MAX);
        assert_eq!(uuid, UUID::max());
        assert_eq!(u128::from(uuid), u128::MAX);
    }

    #[test]
    fn u128_roundtrip() {
        let v: u128 = 0x0123_4567_89ab_cdef_0123_4567_89ab_cdef;
        let uuid = UUID::from(v);
        assert_eq!(u128::from(uuid), v);
    }

    // -----------------------------------------------------------------------
    // i128 <-> UUID
    // -----------------------------------------------------------------------

    #[test]
    fn i128_zero() {
        let uuid = UUID::from(0i128);
        assert_eq!(uuid, UUID::nil());
        assert_eq!(i128::from(uuid), 0);
    }

    #[test]
    fn i128_one() {
        let uuid = UUID::from(1i128);
        assert_eq!(i128::from(uuid), 1);
    }

    #[test]
    fn i128_minus_one() {
        let uuid = UUID::from(-1i128);
        assert_eq!(uuid, UUID::max());
        assert_eq!(i128::from(uuid), -1);
    }

    #[test]
    fn i128_min() {
        let uuid = UUID::from(i128::MIN);
        assert_eq!(i128::from(uuid), i128::MIN);
        // i128::MIN is 0x8000..00, so the high bit is set
        assert_eq!(uuid.as_bytes()[0], 0x80);
        assert!(uuid.as_bytes()[1..].iter().all(|&b| b == 0));
    }

    #[test]
    fn i128_max() {
        let uuid = UUID::from(i128::MAX);
        assert_eq!(i128::from(uuid), i128::MAX);
        assert_eq!(uuid.as_bytes()[0], 0x7F);
        assert!(uuid.as_bytes()[1..].iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn i128_roundtrip_negative() {
        let v: i128 = -0x0123_4567_89ab_cdef_0123_4567_89ab_cdef;
        let uuid = UUID::from(v);
        assert_eq!(i128::from(uuid), v);
    }

    // -----------------------------------------------------------------------
    // Big-endian byte layout
    // -----------------------------------------------------------------------

    #[test]
    fn big_endian_one() {
        let uuid = UUID::from(1u128);
        let bytes = uuid.as_bytes();
        assert!(bytes[..15].iter().all(|&b| b == 0));
        assert_eq!(bytes[15], 1);
    }

    #[test]
    fn big_endian_high_bit() {
        let uuid = UUID::from(1u128 << 127);
        let bytes = uuid.as_bytes();
        assert_eq!(bytes[0], 0x80);
        assert!(bytes[1..].iter().all(|&b| b == 0));
    }

    #[test]
    fn big_endian_known_pattern() {
        let v: u128 = 0x00112233_44556677_8899aabb_ccddeeff;
        let uuid = UUID::from(v);
        assert_eq!(
            *uuid.as_bytes(),
            [
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
                0xee, 0xff
            ]
        );
    }

    // -----------------------------------------------------------------------
    // From<small unsigned> for UUID
    // -----------------------------------------------------------------------

    #[test]
    fn from_u8_zero() {
        assert_eq!(u128::from(UUID::from(0u8)), 0);
    }

    #[test]
    fn from_u8_max() {
        assert_eq!(u128::from(UUID::from(u8::MAX)), u8::MAX as u128);
    }

    #[test]
    fn from_u16_zero() {
        assert_eq!(u128::from(UUID::from(0u16)), 0);
    }

    #[test]
    fn from_u16_max() {
        assert_eq!(u128::from(UUID::from(u16::MAX)), u16::MAX as u128);
    }

    #[test]
    fn from_u32_zero() {
        assert_eq!(u128::from(UUID::from(0u32)), 0);
    }

    #[test]
    fn from_u32_max() {
        assert_eq!(u128::from(UUID::from(u32::MAX)), u32::MAX as u128);
    }

    #[test]
    fn from_u64_zero() {
        assert_eq!(u128::from(UUID::from(0u64)), 0);
    }

    #[test]
    fn from_u64_max() {
        assert_eq!(u128::from(UUID::from(u64::MAX)), u64::MAX as u128);
    }

    #[test]
    fn from_usize_zero() {
        assert_eq!(u128::from(UUID::from(0usize)), 0);
    }

    #[test]
    fn from_usize_max() {
        assert_eq!(u128::from(UUID::from(usize::MAX)), usize::MAX as u128);
    }

    // -----------------------------------------------------------------------
    // From<small signed> for UUID — sign extension
    // -----------------------------------------------------------------------

    #[test]
    fn from_i8_zero() {
        assert_eq!(UUID::from(0i8), UUID::nil());
    }

    #[test]
    fn from_i8_one() {
        assert_eq!(u128::from(UUID::from(1i8)), 1);
    }

    #[test]
    fn from_i8_minus_one() {
        // -1i8 sign-extends through i128 to u128::MAX
        assert_eq!(UUID::from(-1i8), UUID::max());
    }

    #[test]
    fn from_i8_min() {
        assert_eq!(i128::from(UUID::from(i8::MIN)), i128::from(i8::MIN));
    }

    #[test]
    fn from_i8_max() {
        assert_eq!(u128::from(UUID::from(i8::MAX)), i8::MAX as u128);
    }

    #[test]
    fn from_i16_minus_one() {
        assert_eq!(UUID::from(-1i16), UUID::max());
    }

    #[test]
    fn from_i16_min() {
        assert_eq!(i128::from(UUID::from(i16::MIN)), i128::from(i16::MIN));
    }

    #[test]
    fn from_i16_max() {
        assert_eq!(u128::from(UUID::from(i16::MAX)), i16::MAX as u128);
    }

    #[test]
    fn from_i32_minus_one() {
        assert_eq!(UUID::from(-1i32), UUID::max());
    }

    #[test]
    fn from_i32_min() {
        assert_eq!(i128::from(UUID::from(i32::MIN)), i128::from(i32::MIN));
    }

    #[test]
    fn from_i32_max() {
        assert_eq!(u128::from(UUID::from(i32::MAX)), i32::MAX as u128);
    }

    #[test]
    fn from_i64_minus_one() {
        assert_eq!(UUID::from(-1i64), UUID::max());
    }

    #[test]
    fn from_i64_min() {
        assert_eq!(i128::from(UUID::from(i64::MIN)), i128::from(i64::MIN));
    }

    #[test]
    fn from_i64_max() {
        assert_eq!(u128::from(UUID::from(i64::MAX)), i64::MAX as u128);
    }

    #[test]
    fn from_isize_minus_one() {
        assert_eq!(UUID::from(-1isize), UUID::max());
    }

    #[test]
    fn from_isize_min() {
        assert_eq!(i128::from(UUID::from(isize::MIN)), isize::MIN as i128);
    }

    #[test]
    fn from_isize_max() {
        assert_eq!(u128::from(UUID::from(isize::MAX)), isize::MAX as u128);
    }

    // -----------------------------------------------------------------------
    // All negative signed values produce all-FF high bytes (sign extension)
    // -----------------------------------------------------------------------

    #[test]
    fn sign_extension_fills_high_bytes_i8() {
        let uuid = UUID::from(-2i8);
        // -2i8 as i128 = -2, as u128 = u128::MAX - 1 = 0xFFFF...FFFE
        assert!(uuid.as_bytes()[..15].iter().all(|&b| b == 0xFF));
        assert_eq!(uuid.as_bytes()[15], 0xFE);
    }

    #[test]
    fn sign_extension_fills_high_bytes_i16() {
        let uuid = UUID::from(-256i16);
        // -256i16 as i128 = -256, as u128 = 0xFFFF...FF00
        assert!(uuid.as_bytes()[..14].iter().all(|&b| b == 0xFF));
        assert_eq!(uuid.as_bytes()[14], 0xFF);
        assert_eq!(uuid.as_bytes()[15], 0x00);
    }

    #[test]
    fn sign_extension_fills_high_bytes_i32() {
        let uuid = UUID::from(i32::MIN);
        // i32::MIN = -2147483648 => 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_8000_0000
        assert!(uuid.as_bytes()[..12].iter().all(|&b| b == 0xFF));
        assert_eq!(uuid.as_bytes()[12], 0x80);
        assert!(uuid.as_bytes()[13..].iter().all(|&b| b == 0x00));
    }

    #[test]
    fn sign_extension_fills_high_bytes_i64() {
        let uuid = UUID::from(i64::MIN);
        // i64::MIN => 0xFFFF_FFFF_FFFF_FFFF_8000_0000_0000_0000
        assert!(uuid.as_bytes()[..8].iter().all(|&b| b == 0xFF));
        assert_eq!(uuid.as_bytes()[8], 0x80);
        assert!(uuid.as_bytes()[9..].iter().all(|&b| b == 0x00));
    }

    // -----------------------------------------------------------------------
    // Positive signed values zero-extend (no high-byte contamination)
    // -----------------------------------------------------------------------

    #[test]
    fn positive_signed_zero_extends_i8() {
        let uuid = UUID::from(i8::MAX);
        assert!(uuid.as_bytes()[..15].iter().all(|&b| b == 0));
        assert_eq!(uuid.as_bytes()[15], 0x7F);
    }

    #[test]
    fn positive_signed_zero_extends_i64() {
        let uuid = UUID::from(1i64);
        assert!(uuid.as_bytes()[..15].iter().all(|&b| b == 0));
        assert_eq!(uuid.as_bytes()[15], 1);
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for u8: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_u8_zero() {
        assert_eq!(u8::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_u8_max() {
        assert_eq!(u8::try_from(UUID::from(u8::MAX as u128)).unwrap(), u8::MAX);
    }

    #[test]
    fn try_from_uuid_u8_max_plus_one() {
        assert!(u8::try_from(UUID::from(u8::MAX as u128 + 1)).is_err());
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for u16: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_u16_zero() {
        assert_eq!(u16::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_u16_max() {
        assert_eq!(
            u16::try_from(UUID::from(u16::MAX as u128)).unwrap(),
            u16::MAX
        );
    }

    #[test]
    fn try_from_uuid_u16_max_plus_one() {
        assert!(u16::try_from(UUID::from(u16::MAX as u128 + 1)).is_err());
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for u32: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_u32_zero() {
        assert_eq!(u32::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_u32_max() {
        assert_eq!(
            u32::try_from(UUID::from(u32::MAX as u128)).unwrap(),
            u32::MAX
        );
    }

    #[test]
    fn try_from_uuid_u32_max_plus_one() {
        assert!(u32::try_from(UUID::from(u32::MAX as u128 + 1)).is_err());
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for u64: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_u64_zero() {
        assert_eq!(u64::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_u64_max() {
        assert_eq!(
            u64::try_from(UUID::from(u64::MAX as u128)).unwrap(),
            u64::MAX
        );
    }

    #[test]
    fn try_from_uuid_u64_max_plus_one() {
        assert!(u64::try_from(UUID::from(u64::MAX as u128 + 1)).is_err());
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for usize: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_usize_zero() {
        assert_eq!(usize::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_usize_max() {
        assert_eq!(
            usize::try_from(UUID::from(usize::MAX as u128)).unwrap(),
            usize::MAX
        );
    }

    #[test]
    fn try_from_uuid_usize_overflow() {
        // usize::MAX as u128 + 1 overflows usize on any platform
        assert!(usize::try_from(UUID::from(usize::MAX as u128 + 1)).is_err());
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for i8: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_i8_zero() {
        assert_eq!(i8::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_i8_max() {
        assert_eq!(i8::try_from(UUID::from(i8::MAX as u128)).unwrap(), i8::MAX);
    }

    #[test]
    fn try_from_uuid_i8_max_plus_one() {
        // 128 doesn't fit in i8
        assert!(i8::try_from(UUID::from(i8::MAX as u128 + 1)).is_err());
    }

    #[test]
    fn try_from_uuid_i8_from_negative() {
        // A UUID built from -1i8 has u128 value u128::MAX, which via i128
        // is -1, which fits in i8.
        let uuid = UUID::from(-1i8);
        assert_eq!(i8::try_from(uuid).unwrap(), -1);
    }

    #[test]
    fn try_from_uuid_i8_from_i8_min() {
        // UUID from i8::MIN round-trips back through i128 -> i8
        let uuid = UUID::from(i8::MIN);
        assert_eq!(i8::try_from(uuid).unwrap(), i8::MIN);
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for i16: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_i16_zero() {
        assert_eq!(i16::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_i16_max() {
        assert_eq!(
            i16::try_from(UUID::from(i16::MAX as u128)).unwrap(),
            i16::MAX
        );
    }

    #[test]
    fn try_from_uuid_i16_max_plus_one() {
        assert!(i16::try_from(UUID::from(i16::MAX as u128 + 1)).is_err());
    }

    #[test]
    fn try_from_uuid_i16_from_negative() {
        let uuid = UUID::from(-1i16);
        assert_eq!(i16::try_from(uuid).unwrap(), -1);
    }

    #[test]
    fn try_from_uuid_i16_from_i16_min() {
        let uuid = UUID::from(i16::MIN);
        assert_eq!(i16::try_from(uuid).unwrap(), i16::MIN);
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for i32: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_i32_zero() {
        assert_eq!(i32::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_i32_max() {
        assert_eq!(
            i32::try_from(UUID::from(i32::MAX as u128)).unwrap(),
            i32::MAX
        );
    }

    #[test]
    fn try_from_uuid_i32_max_plus_one() {
        assert!(i32::try_from(UUID::from(i32::MAX as u128 + 1)).is_err());
    }

    #[test]
    fn try_from_uuid_i32_from_negative() {
        let uuid = UUID::from(-1i32);
        assert_eq!(i32::try_from(uuid).unwrap(), -1);
    }

    #[test]
    fn try_from_uuid_i32_from_i32_min() {
        let uuid = UUID::from(i32::MIN);
        assert_eq!(i32::try_from(uuid).unwrap(), i32::MIN);
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for i64: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_i64_zero() {
        assert_eq!(i64::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_i64_max() {
        assert_eq!(
            i64::try_from(UUID::from(i64::MAX as u128)).unwrap(),
            i64::MAX
        );
    }

    #[test]
    fn try_from_uuid_i64_max_plus_one() {
        assert!(i64::try_from(UUID::from(i64::MAX as u128 + 1)).is_err());
    }

    #[test]
    fn try_from_uuid_i64_from_negative() {
        let uuid = UUID::from(-1i64);
        assert_eq!(i64::try_from(uuid).unwrap(), -1);
    }

    #[test]
    fn try_from_uuid_i64_from_i64_min() {
        let uuid = UUID::from(i64::MIN);
        assert_eq!(i64::try_from(uuid).unwrap(), i64::MIN);
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for isize: boundary
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_isize_zero() {
        assert_eq!(isize::try_from(UUID::nil()).unwrap(), 0);
    }

    #[test]
    fn try_from_uuid_isize_max() {
        assert_eq!(
            isize::try_from(UUID::from(isize::MAX as u128)).unwrap(),
            isize::MAX
        );
    }

    #[test]
    fn try_from_uuid_isize_overflow() {
        assert!(isize::try_from(UUID::from(isize::MAX as u128 + 1)).is_err());
    }

    #[test]
    fn try_from_uuid_isize_from_negative() {
        let uuid = UUID::from(-1isize);
        assert_eq!(isize::try_from(uuid).unwrap(), -1);
    }

    #[test]
    fn try_from_uuid_isize_from_isize_min() {
        let uuid = UUID::from(isize::MIN);
        assert_eq!(isize::try_from(uuid).unwrap(), isize::MIN);
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for small types rejects UUID::max()
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_max_uuid_fails_u8() {
        assert!(u8::try_from(UUID::max()).is_err());
    }

    #[test]
    fn try_from_max_uuid_fails_u16() {
        assert!(u16::try_from(UUID::max()).is_err());
    }

    #[test]
    fn try_from_max_uuid_fails_u32() {
        assert!(u32::try_from(UUID::max()).is_err());
    }

    #[test]
    fn try_from_max_uuid_fails_u64() {
        assert!(u64::try_from(UUID::max()).is_err());
    }

    #[test]
    fn try_from_max_uuid_fails_usize() {
        assert!(usize::try_from(UUID::max()).is_err());
    }

    // -----------------------------------------------------------------------
    // TryFrom<UUID> for small signed: UUID holding a large positive u128
    // that doesn't fit in the signed i128 interpretation either
    // -----------------------------------------------------------------------

    #[test]
    fn try_from_uuid_i8_large_positive() {
        // u128 value 1000 — i128 is 1000, doesn't fit in i8
        assert!(i8::try_from(UUID::from(1000u128)).is_err());
    }

    #[test]
    fn try_from_uuid_i16_large_positive() {
        assert!(i16::try_from(UUID::from(100_000u128)).is_err());
    }

    #[test]
    fn try_from_uuid_i32_large_positive() {
        assert!(i32::try_from(UUID::from(u32::MAX as u128)).is_err());
    }

    // -----------------------------------------------------------------------
    // Signed roundtrips: From<signed> then TryFrom back
    // -----------------------------------------------------------------------

    #[test]
    fn signed_roundtrip_i8() {
        for v in [i8::MIN, -42, -1, 0, 1, 42, i8::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(i8::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn signed_roundtrip_i16() {
        for v in [i16::MIN, -1000, -1, 0, 1, 1000, i16::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(i16::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn signed_roundtrip_i32() {
        for v in [i32::MIN, -100_000, -1, 0, 1, 100_000, i32::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(i32::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn signed_roundtrip_i64() {
        for v in [i64::MIN, -1_000_000_000, -1, 0, 1, 1_000_000_000, i64::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(i64::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    // -----------------------------------------------------------------------
    // Unsigned roundtrips: From<unsigned> then TryFrom back
    // -----------------------------------------------------------------------

    #[test]
    fn unsigned_roundtrip_u8() {
        for v in [0u8, 1, 127, 128, u8::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(u8::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn unsigned_roundtrip_u16() {
        for v in [0u16, 1, 255, 256, u16::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(u16::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn unsigned_roundtrip_u32() {
        for v in [0u32, 1, u16::MAX as u32, u16::MAX as u32 + 1, u32::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(u32::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn unsigned_roundtrip_u64() {
        for v in [0u64, 1, u32::MAX as u64, u32::MAX as u64 + 1, u64::MAX] {
            let uuid = UUID::from(v);
            assert_eq!(u64::try_from(uuid).unwrap(), v, "roundtrip failed for {v}");
        }
    }

    // -----------------------------------------------------------------------
    // Same integer value via different source types produces same UUID
    // -----------------------------------------------------------------------

    #[test]
    fn same_value_same_uuid_unsigned() {
        let v = 42u128;
        assert_eq!(UUID::from(42u8), UUID::from(v));
        assert_eq!(UUID::from(42u16), UUID::from(v));
        assert_eq!(UUID::from(42u32), UUID::from(v));
        assert_eq!(UUID::from(42u64), UUID::from(v));
        assert_eq!(UUID::from(42usize), UUID::from(v));
        assert_eq!(UUID::from(42u128), UUID::from(v));
    }

    #[test]
    fn same_value_same_uuid_signed_positive() {
        let v = 42u128;
        assert_eq!(UUID::from(42i8), UUID::from(v));
        assert_eq!(UUID::from(42i16), UUID::from(v));
        assert_eq!(UUID::from(42i32), UUID::from(v));
        assert_eq!(UUID::from(42i64), UUID::from(v));
        assert_eq!(UUID::from(42isize), UUID::from(v));
        assert_eq!(UUID::from(42i128), UUID::from(v));
    }

    #[test]
    fn minus_one_same_uuid_all_signed() {
        let expected = UUID::max();
        assert_eq!(UUID::from(-1i8), expected);
        assert_eq!(UUID::from(-1i16), expected);
        assert_eq!(UUID::from(-1i32), expected);
        assert_eq!(UUID::from(-1i64), expected);
        assert_eq!(UUID::from(-1isize), expected);
        assert_eq!(UUID::from(-1i128), expected);
    }

    // -----------------------------------------------------------------------
    // Ordering is preserved: larger integer → larger UUID
    // -----------------------------------------------------------------------

    #[test]
    fn ordering_preserved_unsigned() {
        assert!(UUID::from(0u128) < UUID::from(1u128));
        assert!(UUID::from(1u128) < UUID::from(u128::MAX));
    }

    #[test]
    fn ordering_preserved_for_positive_signed() {
        assert!(UUID::from(0i128) < UUID::from(1i128));
        assert!(UUID::from(1i128) < UUID::from(i128::MAX));
    }

    // -----------------------------------------------------------------------
    // Nil and max UUID → integer identity
    // -----------------------------------------------------------------------

    #[test]
    fn nil_uuid_to_all_unsigned() {
        let nil = UUID::nil();
        assert_eq!(u128::from(nil), 0);
        assert_eq!(u8::try_from(nil).unwrap(), 0);
        assert_eq!(u16::try_from(nil).unwrap(), 0);
        assert_eq!(u32::try_from(nil).unwrap(), 0);
        assert_eq!(u64::try_from(nil).unwrap(), 0);
        assert_eq!(usize::try_from(nil).unwrap(), 0);
    }

    #[test]
    fn nil_uuid_to_all_signed() {
        let nil = UUID::nil();
        assert_eq!(i128::from(nil), 0);
        assert_eq!(i8::try_from(nil).unwrap(), 0);
        assert_eq!(i16::try_from(nil).unwrap(), 0);
        assert_eq!(i32::try_from(nil).unwrap(), 0);
        assert_eq!(i64::try_from(nil).unwrap(), 0);
        assert_eq!(isize::try_from(nil).unwrap(), 0);
    }

    #[test]
    fn max_uuid_to_u128() {
        assert_eq!(u128::from(UUID::max()), u128::MAX);
    }

    #[test]
    fn max_uuid_to_i128() {
        assert_eq!(i128::from(UUID::max()), -1);
    }
}
