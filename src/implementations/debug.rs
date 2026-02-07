use std::fmt;

use crate::UUID;

impl fmt::Debug for UUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{self}}}")
    }
}

#[cfg(test)]
mod tests {
    use super::UUID;
    use core::str::FromStr;

    #[test]
    fn test_uuid_debug() {
        let uuid = UUID {
            bytes: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
        };
        assert_eq!(
            format!("{:?}", uuid),
            "{01020304-0506-0708-090a-0b0c0d0e0f10}"
        );
    }

    #[test]
    fn test_uuid_debug_roundtrip() {
        let uuid = UUID {
            bytes: [
                0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4,
                0x30, 0xc8,
            ],
        };
        let debug = format!("{:?}", uuid);
        let parsed = UUID::from_str(&debug).unwrap();
        assert_eq!(parsed, uuid);
    }
}
