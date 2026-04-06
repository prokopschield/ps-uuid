use std::borrow::BorrowMut;

use crate::{UUID, UUID_BYTES};

impl BorrowMut<[u8; UUID_BYTES]> for UUID {
    fn borrow_mut(&mut self) -> &mut [u8; UUID_BYTES] {
        self.as_mut_bytes()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use std::borrow::BorrowMut;

    use crate::{UUID, UUID_BYTES};

    #[test]
    fn borrow_mut_returns_same_as_as_mut_bytes() {
        let mut uuid = UUID::from(0x0123_4567_89ab_cdef_u128);

        let ptr_borrow: *mut [u8; UUID_BYTES] = uuid.borrow_mut();
        let ptr_method: *mut [u8; UUID_BYTES] = uuid.as_mut_bytes();

        assert!(std::ptr::eq(ptr_borrow, ptr_method));
    }

    #[test]
    fn borrow_mut_mutation_persists() {
        let mut uuid = UUID::nil();

        let borrowed: &mut [u8; UUID_BYTES] = uuid.borrow_mut();
        borrowed[0] = 0xAB;
        borrowed[15] = 0xCD;

        assert_eq!(uuid.as_bytes()[0], 0xAB);
        assert_eq!(uuid.as_bytes()[15], 0xCD);
    }

    #[test]
    fn works_with_borrow_mut_trait_bound() {
        fn accepts_borrow_mut<T: BorrowMut<[u8; UUID_BYTES]>>(v: &mut T) {
            v.borrow_mut()[0] = 0x99;
        }

        let mut uuid = UUID::nil();
        accepts_borrow_mut(&mut uuid);

        assert_eq!(uuid.as_bytes()[0], 0x99);
    }
}
