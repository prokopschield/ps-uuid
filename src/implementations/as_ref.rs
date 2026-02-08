use crate::{UUID, UUID_BYTES};

impl AsRef<[u8; UUID_BYTES]> for UUID {
    fn as_ref(&self) -> &[u8; UUID_BYTES] {
        self.as_bytes()
    }
}

impl AsRef<[u8]> for UUID {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{UUID, UUID_BYTES};

    #[test]
    fn as_ref_slice_matches_as_bytes() {
        let uuid = UUID::from(0x0123_4567_89ab_cdef_u128);
        let slice: &[u8] = uuid.as_ref();
        assert_eq!(slice, uuid.as_bytes().as_slice());
    }

    #[test]
    fn as_ref_array_matches_as_bytes() {
        let uuid = UUID::from(0x0123_4567_89ab_cdef_u128);
        let arr: &[u8; UUID_BYTES] = uuid.as_ref();
        assert_eq!(arr, uuid.as_bytes());
        assert_eq!(std::ptr::eq(arr, uuid.as_bytes()), true);
    }

    #[test]
    fn as_ref_slice_length() {
        let uuid = UUID::nil();
        let slice: &[u8] = uuid.as_ref();
        assert_eq!(slice.len(), UUID_BYTES);
    }

    #[test]
    fn as_ref_nil() {
        let uuid = UUID::nil();
        let slice: &[u8] = uuid.as_ref();
        assert!(slice.iter().all(|&b| b == 0));
    }

    #[test]
    fn as_ref_max() {
        let uuid = UUID::max();
        let slice: &[u8] = uuid.as_ref();
        assert!(slice.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn works_with_generic_as_ref_u8() {
        fn accepts_as_ref(v: &impl AsRef<[u8]>) -> usize {
            v.as_ref().len()
        }
        let uuid = UUID::from(42u128);
        assert_eq!(accepts_as_ref(&uuid), UUID_BYTES);
    }

    #[test]
    fn works_with_generic_as_ref_array() {
        fn accepts_as_ref(v: &impl AsRef<[u8; UUID_BYTES]>) -> u8 {
            v.as_ref()[0]
        }
        let uuid = UUID::from(0xFF00_0000_0000_0000_0000_0000_0000_0000_u128);
        assert_eq!(accepts_as_ref(&uuid), 0xFF);
    }
}
