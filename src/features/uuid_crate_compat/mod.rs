//! Compatibility layer for the `uuid` crate.
//!
//! This module provides API compatibility with the `uuid` crate, including:
//! - A `Uuid` type alias matching the uuid crate's naming convention
//! - Additional methods that mirror the uuid crate's API
//! - Conversion traits between `UUID` and `uuid::Uuid`

use crate::UUID;

/// Type alias for compatibility with the `uuid` crate's naming convention.
pub type Uuid = UUID;

impl UUID {
    /// Creates a UUID from a 128-bit value in little-endian order.
    #[must_use]
    pub const fn from_u128_le(value: u128) -> Self {
        Self {
            bytes: value.swap_bytes().to_be_bytes(),
        }
    }

    /// Returns the UUID as a 128-bit value in little-endian order.
    #[must_use]
    pub const fn to_u128_le(&self) -> u128 {
        self.to_u128().swap_bytes()
    }

    /// Creates a UUID from a byte array in little-endian order.
    #[must_use]
    pub const fn from_bytes_le(bytes: [u8; 16]) -> Self {
        Self {
            bytes: [
                bytes[3], bytes[2], bytes[1], bytes[0], bytes[5], bytes[4], bytes[7], bytes[6],
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
        }
    }

    /// Returns the bytes of the UUID in little-endian order.
    #[must_use]
    pub const fn to_bytes_le(&self) -> [u8; 16] {
        [
            self.bytes[3],
            self.bytes[2],
            self.bytes[1],
            self.bytes[0],
            self.bytes[5],
            self.bytes[4],
            self.bytes[7],
            self.bytes[6],
            self.bytes[8],
            self.bytes[9],
            self.bytes[10],
            self.bytes[11],
            self.bytes[12],
            self.bytes[13],
            self.bytes[14],
            self.bytes[15],
        ]
    }

    /// Creates a UUID from four field values in big-endian order.
    ///
    /// # Arguments
    /// * `d1` - The first field (32 bits)
    /// * `d2` - The second field (16 bits)
    /// * `d3` - The third field (16 bits)
    /// * `d4` - The fourth field (64 bits as 8 bytes)
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        Self {
            bytes: [
                (d1 >> 24) as u8,
                (d1 >> 16) as u8,
                (d1 >> 8) as u8,
                d1 as u8,
                (d2 >> 8) as u8,
                d2 as u8,
                (d3 >> 8) as u8,
                d3 as u8,
                d4[0],
                d4[1],
                d4[2],
                d4[3],
                d4[4],
                d4[5],
                d4[6],
                d4[7],
            ],
        }
    }

    /// Creates a UUID from four field values in little-endian order.
    ///
    /// # Arguments
    /// * `d1` - The first field (32 bits, little-endian)
    /// * `d2` - The second field (16 bits, little-endian)
    /// * `d3` - The third field (16 bits, little-endian)
    /// * `d4` - The fourth field (64 bits as 8 bytes)
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_fields_le(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        Self {
            bytes: [
                d1 as u8,
                (d1 >> 8) as u8,
                (d1 >> 16) as u8,
                (d1 >> 24) as u8,
                d2 as u8,
                (d2 >> 8) as u8,
                d3 as u8,
                (d3 >> 8) as u8,
                d4[0],
                d4[1],
                d4[2],
                d4[3],
                d4[4],
                d4[5],
                d4[6],
                d4[7],
            ],
        }
    }

    /// Returns the four field values of the UUID in big-endian order.
    ///
    /// # Returns
    /// A tuple of `(d1, d2, d3, d4)` where:
    /// * `d1` - The first field (32 bits)
    /// * `d2` - The second field (16 bits)
    /// * `d3` - The third field (16 bits)
    /// * `d4` - The fourth field (64 bits as 8 bytes)
    #[must_use]
    pub const fn as_fields(&self) -> (u32, u16, u16, [u8; 8]) {
        let d1 = ((self.bytes[0] as u32) << 24)
            | ((self.bytes[1] as u32) << 16)
            | ((self.bytes[2] as u32) << 8)
            | (self.bytes[3] as u32);

        let d2 = ((self.bytes[4] as u16) << 8) | (self.bytes[5] as u16);

        let d3 = ((self.bytes[6] as u16) << 8) | (self.bytes[7] as u16);

        let d4 = [
            self.bytes[8],
            self.bytes[9],
            self.bytes[10],
            self.bytes[11],
            self.bytes[12],
            self.bytes[13],
            self.bytes[14],
            self.bytes[15],
        ];

        (d1, d2, d3, d4)
    }
}

impl From<uuid::Uuid> for UUID {
    fn from(uuid: uuid::Uuid) -> Self {
        Self::from_bytes(*uuid.as_bytes())
    }
}

impl From<UUID> for uuid::Uuid {
    fn from(uuid: UUID) -> Self {
        Self::from_bytes(uuid.bytes)
    }
}

impl From<&uuid::Uuid> for UUID {
    fn from(uuid: &uuid::Uuid) -> Self {
        Self::from_bytes(*uuid.as_bytes())
    }
}

impl From<&UUID> for uuid::Uuid {
    fn from(uuid: &UUID) -> Self {
        Self::from_bytes(uuid.bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u128_le_roundtrip() {
        let value: u128 = 0x0123_4567_89ab_cdef_0123_4567_89ab_cdef;
        let uuid = UUID::from_u128_le(value);
        assert_eq!(uuid.to_u128_le(), value);
    }

    #[test]
    fn from_bytes_le_roundtrip() {
        let bytes: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let uuid = UUID::from_bytes_le(bytes);
        assert_eq!(uuid.to_bytes_le(), bytes);
    }

    #[test]
    fn from_fields_roundtrip() {
        let d1: u32 = 0x0123_4567;
        let d2: u16 = 0x89ab;
        let d3: u16 = 0xcdef;
        let d4: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

        let uuid = UUID::from_fields(d1, d2, d3, &d4);
        let (rd1, rd2, rd3, rd4) = uuid.as_fields();

        assert_eq!(d1, rd1);
        assert_eq!(d2, rd2);
        assert_eq!(d3, rd3);
        assert_eq!(d4, rd4);
    }

    #[test]
    fn from_fields_le_creates_correct_bytes() {
        let d1: u32 = 0x0123_4567;
        let d2: u16 = 0x89ab;
        let d3: u16 = 0xcdef;
        let d4: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];

        let uuid = UUID::from_fields_le(d1, d2, d3, &d4);

        // First field (d1) should be byte-swapped
        assert_eq!(uuid.bytes[0], 0x67);
        assert_eq!(uuid.bytes[1], 0x45);
        assert_eq!(uuid.bytes[2], 0x23);
        assert_eq!(uuid.bytes[3], 0x01);

        // Second field (d2) should be byte-swapped
        assert_eq!(uuid.bytes[4], 0xab);
        assert_eq!(uuid.bytes[5], 0x89);

        // Third field (d3) should be byte-swapped
        assert_eq!(uuid.bytes[6], 0xef);
        assert_eq!(uuid.bytes[7], 0xcd);

        // Fourth field (d4) should be unchanged
        assert_eq!(&uuid.bytes[8..], &d4);
    }

    #[test]
    fn uuid_crate_conversion_roundtrip() {
        let original = UUID::gen_v4();
        let external: uuid::Uuid = original.into();
        let back: UUID = external.into();
        assert_eq!(original, back);
    }

    #[test]
    fn uuid_crate_conversion_by_ref() {
        let original = UUID::gen_v4();
        let external: uuid::Uuid = (&original).into();
        let back: UUID = (&external).into();
        assert_eq!(original, back);
    }

    #[test]
    fn type_alias_works() {
        let uuid: Uuid = Uuid::gen_v4();
        assert!(uuid.is_v4());
    }

    #[test]
    #[allow(clippy::similar_names)]
    fn le_be_conversion_consistency() {
        let uuid = UUID::gen_v4();

        // Big-endian roundtrip
        let be_value = uuid.to_u128();
        let uuid_from_be = UUID::from_u128(be_value);
        assert_eq!(uuid, uuid_from_be);

        // Little-endian roundtrip
        let le_value = uuid.to_u128_le();
        let uuid_from_le = UUID::from_u128_le(le_value);
        assert_eq!(uuid, uuid_from_le);

        // BE and LE values should differ (unless the UUID is palindromic)
        // For a random v4 UUID, they should almost certainly differ
        assert_ne!(be_value, le_value);
    }
}
