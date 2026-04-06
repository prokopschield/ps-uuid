use std::str::FromStr;

use crate::{UuidParseError, UUID, UUID_BYTES};

impl TryFrom<&[u8]> for UUID {
    type Error = UuidParseError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() == UUID_BYTES {
            let bytes: [u8; UUID_BYTES] = slice
                .try_into()
                .map_err(|_| UuidParseError::InvalidLength)?;

            return Ok(Self::from_bytes(bytes));
        }

        let s = std::str::from_utf8(slice).map_err(|_| UuidParseError::InvalidLength)?;

        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use crate::{UuidParseError, UUID, UUID_BYTES};

    #[test]
    fn try_from_slice_valid() {
        let bytes: [u8; UUID_BYTES] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let slice: &[u8] = &bytes;

        let uuid = UUID::try_from(slice).expect("valid 16-byte slice should parse");

        assert_eq!(uuid.as_bytes(), &bytes);
    }

    #[test]
    fn try_from_slice_too_short() {
        let bytes: [u8; 15] = [0; 15];
        let slice: &[u8] = &bytes;

        let result = UUID::try_from(slice);

        assert_eq!(result, Err(UuidParseError::InvalidLength));
    }

    #[test]
    fn try_from_slice_too_long() {
        let bytes: [u8; 17] = [0; 17];
        let slice: &[u8] = &bytes;

        let result = UUID::try_from(slice);

        assert_eq!(result, Err(UuidParseError::InvalidLength));
    }

    #[test]
    fn try_from_slice_empty() {
        let slice: &[u8] = &[];

        let result = UUID::try_from(slice);

        assert_eq!(result, Err(UuidParseError::InvalidLength));
    }

    #[test]
    fn try_from_slice_nil() {
        let bytes = [0u8; UUID_BYTES];
        let slice: &[u8] = &bytes;

        let uuid = UUID::try_from(slice).expect("nil bytes should parse");

        assert_eq!(uuid, UUID::nil());
    }

    #[test]
    fn try_from_slice_max() {
        let bytes = [0xFFu8; UUID_BYTES];
        let slice: &[u8] = &bytes;

        let uuid = UUID::try_from(slice).expect("max bytes should parse");

        assert_eq!(uuid, UUID::max());
    }

    #[test]
    fn try_from_slice_matches_from_array() {
        let bytes: [u8; UUID_BYTES] = [
            0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
            0xde, 0xf0,
        ];
        let slice: &[u8] = &bytes;

        let from_slice = UUID::try_from(slice).expect("valid slice should parse");
        let from_array = UUID::from(bytes);

        assert_eq!(from_slice, from_array);
    }

    #[test]
    fn try_from_slice_parses_uuid_string_hyphenated() {
        let string = "550e8400-e29b-41d4-a716-446655440000";
        let slice: &[u8] = string.as_bytes();

        let uuid = UUID::try_from(slice).expect("hyphenated string bytes should parse");

        assert_eq!(
            uuid,
            string
                .parse::<UUID>()
                .expect("hyphenated string should parse")
        );
    }

    #[test]
    fn try_from_slice_parses_uuid_string_simple() {
        let string = "550e8400e29b41d4a716446655440000";
        let slice: &[u8] = string.as_bytes();

        let uuid = UUID::try_from(slice).expect("simple string bytes should parse");

        assert_eq!(
            uuid,
            string.parse::<UUID>().expect("simple string should parse")
        );
    }

    #[test]
    fn try_from_slice_parses_uuid_string_braced() {
        let string = "{550e8400-e29b-41d4-a716-446655440000}";
        let slice: &[u8] = string.as_bytes();

        let uuid = UUID::try_from(slice).expect("braced string bytes should parse");

        assert_eq!(
            uuid,
            string.parse::<UUID>().expect("braced string should parse")
        );
    }

    #[test]
    fn try_from_slice_parses_uuid_string_urn() {
        let string = "urn:uuid:550e8400-e29b-41d4-a716-446655440000";
        let slice: &[u8] = string.as_bytes();

        let uuid = UUID::try_from(slice).expect("URN string bytes should parse");

        assert_eq!(
            uuid,
            string.parse::<UUID>().expect("URN string should parse")
        );
    }

    #[test]
    fn try_from_slice_rejects_invalid_utf8() {
        let bytes: &[u8] = &[0xFF, 0xFE, 0x00, 0x01];

        let result = UUID::try_from(bytes);

        assert_eq!(result, Err(UuidParseError::InvalidLength));
    }

    #[test]
    fn try_from_slice_rejects_invalid_uuid_string() {
        let string = "not-a-valid-uuid-string-at-all!!";
        let slice: &[u8] = string.as_bytes();

        let result = UUID::try_from(slice);

        assert!(result.is_err());
    }
}
