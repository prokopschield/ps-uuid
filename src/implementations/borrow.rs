use std::borrow::Borrow;

use crate::{UUID, UUID_BYTES};

impl Borrow<[u8; UUID_BYTES]> for UUID {
    fn borrow(&self) -> &[u8; UUID_BYTES] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use std::borrow::Borrow;
    use std::collections::HashMap;

    use crate::{UUID, UUID_BYTES};

    #[test]
    fn borrow_returns_same_as_as_bytes() {
        let uuid = UUID::from(0x0123_4567_89ab_cdef_u128);

        let borrowed: &[u8; UUID_BYTES] = uuid.borrow();

        assert_eq!(borrowed, uuid.as_bytes());
    }

    #[test]
    fn borrow_nil() {
        let uuid = UUID::nil();

        let borrowed: &[u8; UUID_BYTES] = uuid.borrow();

        assert!(borrowed.iter().all(|&b| b == 0));
    }

    #[test]
    fn borrow_max() {
        let uuid = UUID::max();

        let borrowed: &[u8; UUID_BYTES] = uuid.borrow();

        assert!(borrowed.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn hashmap_lookup_by_bytes() {
        let uuid = UUID::from(0x1234_5678_9abc_def0_u128);
        let bytes: [u8; UUID_BYTES] = *uuid.as_bytes();

        let mut map: HashMap<UUID, &str> = HashMap::new();
        map.insert(uuid, "found");

        assert_eq!(map.get(&bytes), Some(&"found"));
    }

    #[test]
    fn works_with_borrow_trait_bound() {
        fn accepts_borrow<T: Borrow<[u8; UUID_BYTES]>>(v: &T) -> u8 {
            v.borrow()[0]
        }

        let uuid = UUID::from(0xFF00_0000_0000_0000_0000_0000_0000_0000_u128);

        assert_eq!(accepts_borrow(&uuid), 0xFF);
    }
}
