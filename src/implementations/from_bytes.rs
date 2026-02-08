use crate::{UUID, UUID_BYTES};

impl From<[u8; UUID_BYTES]> for UUID {
    fn from(bytes: [u8; UUID_BYTES]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl From<UUID> for [u8; UUID_BYTES] {
    fn from(uuid: UUID) -> Self {
        *uuid.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{UUID, UUID_BYTES};

    #[test]
    fn from_array_roundtrip() {
        let bytes: [u8; UUID_BYTES] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let uuid = UUID::from(bytes);
        let back: [u8; UUID_BYTES] = uuid.into();
        assert_eq!(bytes, back);
    }

    #[test]
    fn from_array_matches_from_bytes() {
        let bytes = [0xAB; UUID_BYTES];
        assert_eq!(UUID::from(bytes), UUID::from_bytes(bytes));
    }

    #[test]
    fn into_array_matches_as_bytes() {
        let uuid = UUID::from(0x0123_4567_89ab_cdef_u128);
        let arr: [u8; UUID_BYTES] = uuid.into();
        assert_eq!(&arr, uuid.as_bytes());
    }

    #[test]
    fn nil_roundtrip() {
        let uuid = UUID::nil();
        let arr: [u8; UUID_BYTES] = uuid.into();
        assert_eq!(arr, [0u8; UUID_BYTES]);
        assert_eq!(UUID::from(arr), uuid);
    }

    #[test]
    fn max_roundtrip() {
        let uuid = UUID::max();
        let arr: [u8; UUID_BYTES] = uuid.into();
        assert_eq!(arr, [0xFF; UUID_BYTES]);
        assert_eq!(UUID::from(arr), uuid);
    }
}
