use rkyv::{
    bytecheck::CheckBytes,
    rancor::Fallible,
    traits::{CopyOptimization, NoUndef},
    Archive, Deserialize, Portable, Serialize,
};

use crate::UUID;

unsafe impl NoUndef for UUID {}
unsafe impl Portable for UUID {}

// The archived form is just the same as the original: [u8; 16]
impl Archive for UUID {
    type Archived = Self;
    type Resolver = ();

    const COPY_OPTIMIZATION: CopyOptimization<Self> = unsafe { CopyOptimization::enable() };

    fn resolve(&self, (): Self::Resolver, out: rkyv::Place<Self::Archived>) {
        out.write(*self);
    }
}

impl<S: Fallible + ?Sized> Serialize<S> for UUID {
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, <S as Fallible>::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<Self, D> for UUID {
    fn deserialize(&self, _: &mut D) -> Result<Self, <D as Fallible>::Error> {
        Ok(*self)
    }
}

unsafe impl<C: Fallible + ?Sized> CheckBytes<C> for UUID {
    unsafe fn check_bytes(_: *const Self, _: &mut C) -> Result<(), C::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rkyv::{deserialize, rancor::Error, to_bytes, vec::ArchivedVec};

    use crate::UUID;

    fn sample_uuid() -> UUID {
        UUID {
            bytes: [
                0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66,
                0x77, 0x88,
            ],
        }
    }

    #[test]
    fn serialize_and_deserialize_roundtrip() {
        let original = sample_uuid();

        let bytes = to_bytes::<rkyv::rancor::Error>(&original).expect("failed to serialize");
        let archived = unsafe { rkyv::access_unchecked::<UUID>(&bytes) };

        // Archived form should match
        assert_eq!(archived.bytes, original.bytes);

        // Deserialize back
        let deserialized = rkyv::deserialize::<UUID, Error>(archived).unwrap();

        assert_eq!(deserialized, original);
    }

    #[test]
    fn multiple_uuids_roundtrip() {
        let uuids = vec![
            sample_uuid(),
            UUID { bytes: [0u8; 16] },
            UUID { bytes: [0xFF; 16] },
        ];

        let bytes = to_bytes::<Error>(&uuids).expect("failed to serialize");
        let archived = unsafe { rkyv::access_unchecked::<ArchivedVec<UUID>>(&bytes) };

        assert_eq!(archived.len(), uuids.len());

        for (archived_uuid, original_uuid) in archived.iter().zip(&uuids) {
            assert_eq!(archived_uuid.bytes, original_uuid.bytes);
        }

        let deserialized = deserialize::<Vec<UUID>, Error>(archived).unwrap();

        assert_eq!(deserialized, uuids);
    }

    #[test]
    fn archived_equality_check() {
        let u1 = sample_uuid();
        let u2 = sample_uuid();

        let bytes1 = to_bytes::<Error>(&u1).unwrap();
        let bytes2 = to_bytes::<Error>(&u2).unwrap();

        let a1 = unsafe { rkyv::access_unchecked::<UUID>(&bytes1) };
        let a2 = unsafe { rkyv::access_unchecked::<UUID>(&bytes2) };

        // Archived equality works because of #[archive(compare(PartialEq))]
        assert_eq!(a1, a2);
    }

    #[test]
    fn validation_of_serialized_data() {
        let original = sample_uuid();
        let bytes = to_bytes::<Error>(&original).unwrap();

        // Validate the serialized buffer
        let check_result = rkyv::access::<UUID, Error>(&bytes);

        assert!(check_result.is_ok());
    }

    #[test]
    fn zero_and_max_uuid() {
        let zero = UUID { bytes: [0u8; 16] };
        let max = UUID { bytes: [0xFF; 16] };

        for uuid in [zero, max] {
            let bytes = to_bytes::<Error>(&uuid).unwrap();
            let archived = unsafe { rkyv::access_unchecked::<UUID>(&bytes) };

            assert_eq!(archived.bytes, uuid.bytes);

            let deserialized: UUID = deserialize::<UUID, Error>(archived).unwrap();

            assert_eq!(deserialized, uuid);
        }
    }

    #[test]
    fn clone_and_eq_work() {
        let u1 = sample_uuid();
        let u2 = u1.clone();

        assert_eq!(u1, u2);
    }
}
