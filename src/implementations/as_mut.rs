use crate::{UUID, UUID_BYTES};

impl AsMut<[u8; UUID_BYTES]> for UUID {
    fn as_mut(&mut self) -> &mut [u8; UUID_BYTES] {
        self.as_mut_bytes()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_literal, clippy::cast_possible_truncation)]
    use crate::{UUID, UUID_BYTES};

    #[test]
    fn as_mut_returns_same_reference_as_as_mut_bytes() {
        let mut uuid = UUID::from(0x0123_4567_89ab_cdef_u128);

        let ptr_via_as_mut: *mut [u8; UUID_BYTES] = uuid.as_mut();
        let ptr_via_method: *mut [u8; UUID_BYTES] = uuid.as_mut_bytes();

        assert!(std::ptr::eq(ptr_via_as_mut, ptr_via_method));
    }

    #[test]
    fn as_mut_content_matches_as_bytes() {
        let mut uuid = UUID::from(0xfedc_ba98_7654_3210_u128);
        let expected = *uuid.as_bytes();

        let arr: &mut [u8; UUID_BYTES] = uuid.as_mut();

        assert_eq!(*arr, expected);
    }

    #[test]
    fn mutation_through_as_mut_modifies_uuid() {
        let mut uuid = UUID::nil();

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();
        bytes[0] = 0xAB;
        bytes[15] = 0xCD;

        assert_eq!(uuid.as_bytes()[0], 0xAB);
        assert_eq!(uuid.as_bytes()[15], 0xCD);
    }

    #[test]
    fn mutation_persists_across_multiple_as_mut_calls() {
        let mut uuid = UUID::nil();

        uuid.as_mut()[0] = 0x11;
        uuid.as_mut()[1] = 0x22;
        uuid.as_mut()[2] = 0x33;

        assert_eq!(uuid.as_bytes()[0], 0x11);
        assert_eq!(uuid.as_bytes()[1], 0x22);
        assert_eq!(uuid.as_bytes()[2], 0x33);
    }

    #[test]
    fn full_overwrite_via_as_mut() {
        let mut uuid = UUID::nil();

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();
        bytes.fill(0xFF);

        assert_eq!(uuid, UUID::max());
    }

    #[test]
    fn as_mut_nil_uuid() {
        let mut uuid = UUID::nil();

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();

        assert!(bytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn as_mut_max_uuid() {
        let mut uuid = UUID::max();

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();

        assert!(bytes.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn as_mut_length() {
        let mut uuid = UUID::from(12345_u128);

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();

        assert_eq!(bytes.len(), UUID_BYTES);
    }

    #[test]
    fn works_with_generic_as_mut_bound() {
        fn accepts_as_mut<T: AsMut<[u8; UUID_BYTES]>>(v: &mut T) {
            v.as_mut()[0] = 0x99;
        }

        let mut uuid = UUID::nil();
        accepts_as_mut(&mut uuid);
        assert_eq!(uuid.as_bytes()[0], 0x99);
    }

    #[test]
    fn mutation_via_slice_coercion() {
        let mut uuid = UUID::nil();

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();
        let slice: &mut [u8] = bytes.as_mut_slice();
        slice[7] = 0x42;

        assert_eq!(uuid.as_bytes()[7], 0x42);
    }

    #[test]
    fn copy_from_slice_via_as_mut() {
        let mut uuid = UUID::nil();
        let source: [u8; UUID_BYTES] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];

        uuid.as_mut().copy_from_slice(&source);
        assert_eq!(uuid.as_bytes(), &source);
    }

    #[test]
    fn swap_bytes_via_as_mut() {
        let mut uuid = UUID::from(0xFF00_0000_0000_0000_0000_0000_0000_00AA_u128);

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();
        bytes.swap(0, 15);

        assert_eq!(uuid.as_bytes()[0], 0xAA);
        assert_eq!(uuid.as_bytes()[15], 0xFF);
    }

    #[test]
    fn reverse_bytes_via_as_mut() {
        let mut uuid = UUID::from(0x0102_0304_0506_0708_090A_0B0C_0D0E_0F10_u128);
        uuid.as_mut().reverse();

        let expected = UUID::from(0x100F_0E0D_0C0B_0A09_0807_0605_0403_0201_u128);
        assert_eq!(uuid, expected);
    }

    #[test]
    fn iter_mut_via_as_mut() {
        let mut uuid = UUID::from(0x0101_0101_0101_0101_0101_0101_0101_0101_u128);

        for byte in uuid.as_mut().iter_mut() {
            *byte = byte.wrapping_add(1);
        }

        let expected = UUID::from(0x0202_0202_0202_0202_0202_0202_0202_0202_u128);
        assert_eq!(uuid, expected);
    }

    #[test]
    fn clear_via_as_mut() {
        let mut uuid = UUID::max();

        uuid.as_mut().fill(0);

        assert_eq!(uuid, UUID::nil());
    }

    #[test]
    fn sort_bytes_via_as_mut() {
        let mut uuid = UUID::from(0x0F0E_0D0C_0B0A_0908_0706_0504_0302_0100_u128);

        uuid.as_mut().sort_unstable();

        let expected = UUID::from(0x0001_0203_0405_0607_0809_0A0B_0C0D_0E0F_u128);
        assert_eq!(uuid, expected);
    }

    #[test]
    fn rotate_left_via_as_mut() {
        let mut uuid = UUID::from(0x0102_0304_0506_0708_090A_0B0C_0D0E_0F10_u128);

        uuid.as_mut().rotate_left(4);

        let expected = UUID::from(0x0506_0708_090A_0B0C_0D0E_0F10_0102_0304_u128);
        assert_eq!(uuid, expected);
    }

    #[test]
    fn rotate_right_via_as_mut() {
        let mut uuid = UUID::from(0x0102_0304_0506_0708_090A_0B0C_0D0E_0F10_u128);

        uuid.as_mut().rotate_right(4);

        let expected = UUID::from(0x0D0E_0F10_0102_0304_0506_0708_090A_0B0C_u128);
        assert_eq!(uuid, expected);
    }

    #[test]
    fn partial_fill_via_as_mut() {
        let mut uuid = UUID::nil();

        uuid.as_mut()[4..12].fill(0xAB);

        for i in 0..UUID_BYTES {
            if (4..12).contains(&i) {
                assert_eq!(uuid.as_bytes()[i], 0xAB);
            } else {
                assert_eq!(uuid.as_bytes()[i], 0x00);
            }
        }
    }

    #[test]
    fn split_at_mut_via_as_mut() {
        let mut uuid = UUID::nil();

        let (left, right) = uuid.as_mut().split_at_mut(8);
        left.fill(0x11);
        right.fill(0x22);

        assert!(uuid.as_bytes()[..8].iter().all(|&b| b == 0x11));
        assert!(uuid.as_bytes()[8..].iter().all(|&b| b == 0x22));
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn chunks_mut_via_as_mut() {
        let mut uuid = UUID::nil();

        for (i, chunk) in uuid.as_mut().chunks_mut(4).enumerate() {
            chunk.fill(i as u8);
        }

        assert_eq!(uuid.as_bytes()[0..4], [0, 0, 0, 0]);
        assert_eq!(uuid.as_bytes()[4..8], [1, 1, 1, 1]);
        assert_eq!(uuid.as_bytes()[8..12], [2, 2, 2, 2]);
        assert_eq!(uuid.as_bytes()[12..16], [3, 3, 3, 3]);
    }

    #[test]
    fn as_mut_does_not_affect_other_uuid_instances() {
        let mut uuid1 = UUID::from(0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_u128);
        let uuid2 = uuid1;

        uuid1.as_mut()[0] = 0xBB;

        assert_eq!(uuid1.as_bytes()[0], 0xBB);
        assert_eq!(uuid2.as_bytes()[0], 0xAA);
    }

    #[test]
    fn as_mut_on_default_uuid() {
        let mut uuid = UUID::default();

        let bytes: &mut [u8; UUID_BYTES] = uuid.as_mut();

        assert!(bytes.iter().all(|&b| b == 0));
    }

    #[test]
    fn bitwise_operations_via_as_mut() {
        let mut uuid = UUID::from(0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_u128);

        for byte in uuid.as_mut().iter_mut() {
            *byte &= 0x0F;
        }

        let expected = UUID::from(0x0000_0000_0000_0000_0000_0000_0000_0000_u128);
        assert_eq!(uuid, expected);
    }

    #[test]
    fn xor_pattern_via_as_mut() {
        let mut uuid = UUID::from(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128);

        for byte in uuid.as_mut().iter_mut() {
            *byte ^= 0xAA;
        }

        let expected = UUID::from(0x5555_5555_5555_5555_5555_5555_5555_5555_u128);
        assert_eq!(uuid, expected);
    }

    #[test]
    fn first_last_mut_via_as_mut() {
        let mut uuid = UUID::nil();

        if let Some(first) = uuid.as_mut().first_mut() {
            *first = 0x01;
        }
        if let Some(last) = uuid.as_mut().last_mut() {
            *last = 0x10;
        }

        assert_eq!(uuid.as_bytes()[0], 0x01);
        assert_eq!(uuid.as_bytes()[15], 0x10);
    }

    #[test]
    fn get_mut_valid_index() {
        let mut uuid = UUID::nil();

        if let Some(byte) = uuid.as_mut().get_mut(7) {
            *byte = 0x77;
        }

        assert_eq!(uuid.as_bytes()[7], 0x77);
    }

    #[test]
    fn get_mut_out_of_bounds_returns_none() {
        let mut uuid = UUID::nil();

        assert!(uuid.as_mut().get_mut(16).is_none());
        assert!(uuid.as_mut().get_mut(100).is_none());
    }

    #[test]
    fn try_into_works_after_as_mut() {
        let mut uuid = UUID::nil();

        uuid.as_mut().fill(0x42);

        let arr: [u8; UUID_BYTES] = *uuid.as_mut();
        assert!(arr.iter().all(|&b| b == 0x42));
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn as_mut_with_write_pattern() {
        let mut uuid = UUID::nil();
        let bytes = uuid.as_mut();

        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = (i * 17) as u8;
        }

        for i in 0..UUID_BYTES {
            assert_eq!(uuid.as_bytes()[i], (i * 17) as u8);
        }
    }

    #[test]
    fn clone_from_slice_via_as_mut() {
        let mut uuid = UUID::nil();
        let source = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        uuid.as_mut().clone_from_slice(&source);
        assert_eq!(uuid.as_bytes(), &source);
    }

    #[test]
    fn as_mut_ptr_is_stable() {
        let mut uuid = UUID::nil();

        let ptr1 = uuid.as_mut().as_mut_ptr();
        uuid.as_mut()[0] = 0xFF;
        let ptr2 = uuid.as_mut().as_mut_ptr();

        assert_eq!(ptr1, ptr2);
    }

    #[test]
    fn multiple_mutable_operations_in_sequence() {
        let mut uuid = UUID::nil();

        uuid.as_mut().fill(0x11);
        uuid.as_mut().reverse();
        uuid.as_mut()[0] = 0x22;
        uuid.as_mut().rotate_left(1);

        assert_eq!(uuid.as_bytes()[0], 0x11);
        assert_eq!(uuid.as_bytes()[UUID_BYTES - 1], 0x22);
    }
}
