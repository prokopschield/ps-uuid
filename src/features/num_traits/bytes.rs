//! Byte conversion trait implementations for UUID.
//!
//! UUID is internally stored as big-endian bytes, so `to_be_bytes` is
//! a no-op that returns the internal representation directly.

use num_traits::{FromBytes, ToBytes};

use crate::{UUID, UUID_BYTES};

impl ToBytes for UUID {
    type Bytes = [u8; UUID_BYTES];

    #[inline]
    fn to_be_bytes(&self) -> Self::Bytes {
        *self.as_bytes()
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Bytes {
        self.to_u128().to_le_bytes()
    }
}

impl FromBytes for UUID {
    type Bytes = [u8; UUID_BYTES];

    #[inline]
    fn from_be_bytes(bytes: &Self::Bytes) -> Self {
        Self::from_bytes(*bytes)
    }

    #[inline]
    fn from_le_bytes(bytes: &Self::Bytes) -> Self {
        Self::from_bytes(u128::from_le_bytes(*bytes).to_be_bytes())
    }
}
