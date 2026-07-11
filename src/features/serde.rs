use std::{fmt, str::FromStr};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{UUID, UUID_BYTES};

impl Serialize for UUID {
    /// Serializes as the canonical hyphenated string for human-readable
    /// formats (such as JSON), and as the inner 16-byte array for binary
    /// formats (such as bincode or postcard). Fixed-size arrays carry no
    /// length prefix, so the binary encoding is exactly the 16 raw bytes.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            self.bytes.serialize(serializer)
        }
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

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u128(u128::from(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let v = u64::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Signed(v), &self))?;

        self.visit_u64(v)
    }
}

impl<'de> Deserialize<'de> for UUID {
    /// Human-readable formats keep the permissive
    /// [`deserialize_any`](Deserializer::deserialize_any) path, so JSON
    /// accepts a canonical string, a 16-element byte array, or a nonnegative
    /// integer. Binary formats cannot support `deserialize_any`, so they read
    /// back the 16-byte array that [`Serialize`] writes.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_any(UUIDVisitor)
        } else {
            <[u8; UUID_BYTES]>::deserialize(deserializer).map(Self::from_bytes)
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    #![allow(clippy::expect_used)]
    use super::UUID;

    const fn sample_uuid() -> UUID {
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
        let s = serde_json::to_string(&uuid)
            .expect("serde_json roundtrip should succeed for valid UUID values");
        assert_eq!(s, "\"550e8400-e29b-41d4-a716-446655440000\"");
    }

    #[test]
    fn deserialize_from_canonical_string() {
        let s = "\"550e8400-e29b-41d4-a716-446655440000\"";
        let uuid: UUID = serde_json::from_str(s)
            .expect("serde_json roundtrip should succeed for valid UUID values");
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_string_without_hyphens() {
        let s = "\"550e8400e29b41d4a716446655440000\"";
        let uuid: UUID = serde_json::from_str(s)
            .expect("serde_json roundtrip should succeed for valid UUID values");
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_string_with_braces() {
        let s = "\"{550e8400-e29b-41d4-a716-446655440000}\"";
        let uuid: UUID = serde_json::from_str(s)
            .expect("serde_json roundtrip should succeed for valid UUID values");
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_uppercase_string() {
        let s = "\"550E8400-E29B-41D4-A716-446655440000\"";
        let uuid: UUID = serde_json::from_str(s)
            .expect("serde_json roundtrip should succeed for valid UUID values");
        assert_eq!(uuid, sample_uuid());
    }

    #[test]
    fn deserialize_from_bytes_array() {
        let bytes = sample_uuid().bytes;
        let json = serde_json::to_string(&bytes)
            .expect("serde_json roundtrip should succeed for valid UUID values");
        let uuid: UUID = serde_json::from_str(&json)
            .expect("serde_json roundtrip should succeed for valid UUID values");
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
        let s = serde_json::to_string(&uuid)
            .expect("serde_json roundtrip should succeed for valid UUID values");
        let back: UUID = serde_json::from_str(&s)
            .expect("serde_json roundtrip should succeed for valid UUID values");
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
    fn deserialize_from_u64_number() {
        let uuid: UUID = serde_json::from_str("42")
            .expect("serde_json should deserialize a nonnegative integer");

        assert_eq!(uuid, UUID::from(42u128));
    }

    #[test]
    fn deserialize_from_u64_max() {
        let uuid: UUID = serde_json::from_str(&u64::MAX.to_string())
            .expect("serde_json should deserialize a nonnegative integer");

        let mut expected = [0u8; 16];

        expected[8..].fill(0xFF);

        assert_eq!(uuid.bytes, expected);
    }

    #[test]
    fn deserialize_from_positive_i64() {
        use serde::de::value::{Error as ValueError, I64Deserializer};
        use serde::Deserialize;

        // serde_json routes nonnegative integers to visit_u64, so the i64
        // path needs a signed-integer deserializer to be exercised.
        let uuid = UUID::deserialize(I64Deserializer::<ValueError>::new(42))
            .expect("a positive i64 should deserialize");

        assert_eq!(uuid, UUID::from(42u128));
    }

    #[test]
    fn invalid_negative_number() {
        let json = "-42";
        let res: Result<UUID, _> = serde_json::from_str(json);
        assert!(res.is_err());
    }

    // bincode and postcard are not self-describing and report
    // is_human_readable() == false, so the following tests exercise the
    // binary path that deserialize_any cannot support.

    #[test]
    fn binary_serialization_is_the_identity() {
        let uuid = sample_uuid();

        let encoded =
            bincode::serialize(&uuid).expect("bincode should serialize a UUID as its 16 raw bytes");

        assert_eq!(encoded, uuid.bytes);
    }

    #[test]
    fn round_trip_bincode_binary_format() {
        let uuid = sample_uuid();

        let encoded =
            bincode::serialize(&uuid).expect("bincode should serialize a UUID as its 16 raw bytes");
        let back: UUID = bincode::deserialize(&encoded).expect("bincode should round-trip a UUID");

        assert_eq!(uuid, back);
    }

    #[test]
    fn bincode_embeds_uuids_without_a_length_prefix() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct Wrapper {
            before: u8,
            uuid: UUID,
            after: u8,
        }

        let wrapper = Wrapper {
            before: 0xAA,
            uuid: sample_uuid(),
            after: 0xBB,
        };

        let encoded = bincode::serialize(&wrapper).expect("bincode should serialize the wrapper");

        assert_eq!(encoded.len(), 18);
        assert_eq!(encoded[0], 0xAA);
        assert_eq!(encoded[1..17], wrapper.uuid.bytes);
        assert_eq!(encoded[17], 0xBB);

        let back: Wrapper =
            bincode::deserialize(&encoded).expect("bincode should round-trip the wrapper");

        assert_eq!(wrapper, back);
    }

    #[test]
    fn postcard_encodes_as_the_sixteen_raw_bytes() {
        let uuid = sample_uuid();

        let encoded = postcard::to_allocvec(&uuid).expect("postcard should serialize a UUID");

        assert_eq!(encoded, uuid.bytes);
    }

    #[test]
    fn round_trip_postcard_binary_format() {
        let uuid = sample_uuid();

        let encoded = postcard::to_allocvec(&uuid).expect("postcard should serialize a UUID");
        let back: UUID = postcard::from_bytes(&encoded).expect("postcard should round-trip a UUID");

        assert_eq!(uuid, back);
    }

    #[test]
    fn postcard_rejects_truncated_input() {
        let res: Result<UUID, _> = postcard::from_bytes(&sample_uuid().bytes[..10]);

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
            let s = serde_json::to_string(&uuid)
                .expect("serde_json roundtrip should succeed for valid UUID values");
            let back: UUID = serde_json::from_str(&s)
                .expect("serde_json roundtrip should succeed for valid UUID values");
            assert_eq!(uuid, back);
        }
    }
}
