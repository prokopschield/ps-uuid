use std::{fmt, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{UUID, UUID_BYTES};

impl Serialize for UUID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct UUIDVisitor;

impl<'de> Visitor<'de> for UUIDVisitor {
    type Value = UUID;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a UUID as a string, 16-byte array, or u128")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UUID::from_str(v).map_err(E::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() != UUID_BYTES {
            return Err(E::invalid_length(v.len(), &self));
        }

        let mut bytes = [0u8; UUID_BYTES];

        bytes.copy_from_slice(v);

        Ok(UUID { bytes })
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut bytes = [0u8; UUID_BYTES];

        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(index, &self))?;
        }

        Ok(UUID { bytes })
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UUID::from_bytes(v.to_be_bytes()))
    }
}

impl<'de> Deserialize<'de> for UUID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UUIDVisitor)
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::UUID;

    fn sample_uuid() -> UUID {
        UUID {
            bytes: [
                0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
                0x00, 0x00,
            ],
        }
    }

    #[test]
    fn serialize_to_canonical_string() {
        let uuid = sample_uuid();
        let s = serde_json::to_string(&uuid).unwrap();
        assert_eq!(s, "\"550e8400-e29b-41d4-a716-446655440000\"");
    }

    #[test]
    fn deserialize_from_canonical_string() {
        let s = "\"550e8400-e29b-41d4-a716-446655440000\"";
        let uuid: UUID = serde_json::from_str(s).unwrap();
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_string_without_hyphens() {
        let s = "\"550e8400e29b41d4a716446655440000\"";
        let uuid: UUID = serde_json::from_str(s).unwrap();
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_string_with_braces() {
        let s = "\"{550e8400-e29b-41d4-a716-446655440000}\"";
        let uuid: UUID = serde_json::from_str(s).unwrap();
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_uppercase_string() {
        let s = "\"550E8400-E29B-41D4-A716-446655440000\"";
        let uuid: UUID = serde_json::from_str(s).unwrap();
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_bytes_array() {
        let bytes = sample_uuid().bytes;
        let json = serde_json::to_string(&bytes).unwrap();
        let uuid: UUID = serde_json::from_str(&json).unwrap();
        assert_eq!(uuid.bytes, bytes);
    }

    #[test]
    fn round_trip_json() {
        let uuid = UUID {
            bytes: [
                0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc,
                0xde, 0xf0,
            ],
        };
        let s = serde_json::to_string(&uuid).unwrap();
        let back: UUID = serde_json::from_str(&s).unwrap();
        assert_eq!(uuid, back);
    }

    #[test]
    fn invalid_string_too_short() {
        let s = "\"1234\"";
        let res: Result<UUID, _> = serde_json::from_str(s);
        assert!(res.is_err());
    }

    #[test]
    fn invalid_string_non_hex() {
        let s = "\"550e8400-e29b-41d4-a716-44665544zzzz\"";
        let res: Result<UUID, _> = serde_json::from_str(s);
        assert!(res.is_err());
    }

    #[test]
    fn invalid_bytes_wrong_length() {
        let json = "[1,2,3]";
        let res: Result<UUID, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    #[test]
    fn invalid_sequence_wrong_length() {
        let json = "[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]";
        let res: Result<UUID, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    #[test]
    fn invalid_negative_number() {
        let json = "-42";
        let res: Result<UUID, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    #[test]
    fn round_trip_multiple_randoms() {
        use rand::RngCore;
        let mut rng = rand::rng();
        for _ in 0..100 {
            let mut bytes = [0u8; 16];
            rng.fill_bytes(&mut bytes);
            let uuid = UUID { bytes };
            let s = serde_json::to_string(&uuid).unwrap();
            let back: UUID = serde_json::from_str(&s).unwrap();
            assert_eq!(uuid, back);
        }
    }
}
