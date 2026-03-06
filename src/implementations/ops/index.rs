//! Index operator implementations for UUID.
//!
//! This module provides implementations for:
//!
//! | Trait | Description |
//! |-------|-------------|
//! | [`Index`] | Immutable byte access via `uuid[i]` |
//! | [`IndexMut`] | Mutable byte access via `uuid[i]` |
//!
//! # Byte Order
//!
//! UUID uses **big-endian** byte order:
//!
//! - `uuid[0]` is the **most significant** byte
//! - `uuid[15]` is the **least significant** byte
//!
//! # Supported Index Types
//!
//! Single element access:
//! - `usize` - Access a single byte
//!
//! Range access:
//! - `Range<usize>` (`a..b`) - Exclusive range
//! - `RangeFrom<usize>` (`a..`) - From index to end
//! - `RangeTo<usize>` (`..b`) - From start to index
//! - `RangeFull` (`..`) - All bytes
//! - `RangeInclusive<usize>` (`a..=b`) - Inclusive range
//! - `RangeToInclusive<usize>` (`..=b`) - From start to index (inclusive)
//!
//! # Panics
//!
//! Indexing operations will panic if the index is out of bounds (>= 16 for
//! single access, or if the range exceeds the bounds).
//!
//! # Examples
//!
//! ```
//! use ps_uuid::UUID;
//!
//! let uuid = UUID::from(0xFF00_0000_0000_0000_0000_0000_0000_00FFu128);
//!
//! // Single byte access (big-endian)
//! assert_eq!(uuid[0], 0xFF);   // Most significant byte
//! assert_eq!(uuid[15], 0xFF);  // Least significant byte
//! assert_eq!(uuid[1], 0x00);
//!
//! // Range access
//! assert_eq!(&uuid[0..2], &[0xFF, 0x00]);
//! assert_eq!(&uuid[14..], &[0x00, 0xFF]);
//!
//! // Mutable access
//! let mut uuid = UUID::nil();
//! uuid[0] = 0xAB;
//! uuid[15] = 0xCD;
//! assert_eq!(uuid[0], 0xAB);
//! assert_eq!(uuid[15], 0xCD);
//!
//! // Mutable range access
//! let mut uuid = UUID::nil();
//! uuid[0..4].copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
//! assert_eq!(&uuid[0..4], &[0x01, 0x02, 0x03, 0x04]);
//! ```

use core::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use crate::UUID;

// ============================================================================
// Single Element Indexing
// ============================================================================

impl Index<usize> for UUID {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_bytes()[index]
    }
}

impl IndexMut<usize> for UUID {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_bytes()[index]
    }
}

// ============================================================================
// Range Indexing: Range<usize> (a..b)
// ============================================================================

impl Index<Range<usize>> for UUID {
    type Output = [u8];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.as_bytes()[index]
    }
}

impl IndexMut<Range<usize>> for UUID {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.as_mut_bytes()[index]
    }
}

// ============================================================================
// Range Indexing: RangeFrom<usize> (a..)
// ============================================================================

impl Index<RangeFrom<usize>> for UUID {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.as_bytes()[index]
    }
}

impl IndexMut<RangeFrom<usize>> for UUID {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.as_mut_bytes()[index]
    }
}

// ============================================================================
// Range Indexing: RangeTo<usize> (..b)
// ============================================================================

impl Index<RangeTo<usize>> for UUID {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.as_bytes()[index]
    }
}

impl IndexMut<RangeTo<usize>> for UUID {
    #[inline]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.as_mut_bytes()[index]
    }
}

// ============================================================================
// Range Indexing: RangeFull (..)
// ============================================================================

impl Index<RangeFull> for UUID {
    type Output = [u8];

    #[inline]
    fn index(&self, _index: RangeFull) -> &Self::Output {
        self.as_bytes()
    }
}

impl IndexMut<RangeFull> for UUID {
    #[inline]
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        self.as_mut_bytes()
    }
}

// ============================================================================
// Range Indexing: RangeInclusive<usize> (a..=b)
// ============================================================================

impl Index<RangeInclusive<usize>> for UUID {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.as_bytes()[index]
    }
}

impl IndexMut<RangeInclusive<usize>> for UUID {
    #[inline]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.as_mut_bytes()[index]
    }
}

// ============================================================================
// Range Indexing: RangeToInclusive<usize> (..=b)
// ============================================================================

impl Index<RangeToInclusive<usize>> for UUID {
    type Output = [u8];

    #[inline]
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.as_bytes()[index]
    }
}

impl IndexMut<RangeToInclusive<usize>> for UUID {
    #[inline]
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.as_mut_bytes()[index]
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Single element access tests
    // -------------------------------------------------------------------------

    #[test]
    fn index_first_byte() {
        let uuid = UUID::from(0xFF00_0000_0000_0000_0000_0000_0000_0000u128);
        assert_eq!(uuid[0], 0xFF);
    }

    #[test]
    fn index_last_byte() {
        let uuid = UUID::from(0x0000_0000_0000_0000_0000_0000_0000_00FFu128);
        assert_eq!(uuid[15], 0xFF);
    }

    #[test]
    fn index_middle_bytes() {
        let uuid = UUID::from(0x0000_0000_0000_00FF_FF00_0000_0000_0000u128);
        assert_eq!(uuid[7], 0xFF);
        assert_eq!(uuid[8], 0xFF);
    }

    #[test]
    fn index_all_bytes() {
        let uuid = UUID::from(0x0011_2233_4455_6677_8899_aabb_ccdd_eeffu128);
        assert_eq!(uuid[0], 0x00);
        assert_eq!(uuid[1], 0x11);
        assert_eq!(uuid[2], 0x22);
        assert_eq!(uuid[3], 0x33);
        assert_eq!(uuid[4], 0x44);
        assert_eq!(uuid[5], 0x55);
        assert_eq!(uuid[6], 0x66);
        assert_eq!(uuid[7], 0x77);
        assert_eq!(uuid[8], 0x88);
        assert_eq!(uuid[9], 0x99);
        assert_eq!(uuid[10], 0xaa);
        assert_eq!(uuid[11], 0xbb);
        assert_eq!(uuid[12], 0xcc);
        assert_eq!(uuid[13], 0xdd);
        assert_eq!(uuid[14], 0xee);
        assert_eq!(uuid[15], 0xff);
    }

    #[test]
    fn index_nil() {
        let uuid = UUID::nil();
        for i in 0..16 {
            assert_eq!(uuid[i], 0);
        }
    }

    #[test]
    fn index_max() {
        let uuid = UUID::max();
        for i in 0..16 {
            assert_eq!(uuid[i], 0xFF);
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_out_of_bounds() {
        let uuid = UUID::nil();
        let _ = uuid[16];
    }

    // -------------------------------------------------------------------------
    // Mutable single element access tests
    // -------------------------------------------------------------------------

    #[test]
    fn index_mut_first_byte() {
        let mut uuid = UUID::nil();
        uuid[0] = 0xAB;
        assert_eq!(uuid[0], 0xAB);
        assert_eq!(uuid.as_bytes()[0], 0xAB);
    }

    #[test]
    fn index_mut_last_byte() {
        let mut uuid = UUID::nil();
        uuid[15] = 0xCD;
        assert_eq!(uuid[15], 0xCD);
        assert_eq!(uuid.as_bytes()[15], 0xCD);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn index_mut_all_bytes() {
        let mut uuid = UUID::nil();
        for i in 0..16 {
            uuid[i] = i as u8;
        }
        for i in 0..16 {
            assert_eq!(uuid[i], i as u8);
        }
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_mut_out_of_bounds() {
        let mut uuid = UUID::nil();
        uuid[16] = 0xFF;
    }

    // -------------------------------------------------------------------------
    // Range indexing tests: Range<usize> (a..b)
    // -------------------------------------------------------------------------

    #[test]
    fn index_range() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);
        assert_eq!(&uuid[0..4], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[4..8], &[0x05, 0x06, 0x07, 0x08]);
        assert_eq!(&uuid[12..16], &[0x0d, 0x0e, 0x0f, 0x10]);
    }

    #[test]
    fn index_range_empty() {
        let uuid = UUID::nil();
        assert_eq!(&uuid[0..0], &[] as &[u8]);
        assert_eq!(&uuid[8..8], &[] as &[u8]);
    }

    #[test]
    fn index_range_full() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);
        assert_eq!(uuid[0..16].len(), 16);
    }

    #[test]
    fn index_mut_range() {
        let mut uuid = UUID::nil();
        uuid[0..4].copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[0..4], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[4..16], &[0; 12]);
    }

    // -------------------------------------------------------------------------
    // Range indexing tests: RangeFrom<usize> (a..)
    // -------------------------------------------------------------------------

    #[test]
    fn index_range_from() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);
        assert_eq!(&uuid[14..], &[0x0f, 0x10]);
        assert_eq!(&uuid[0..].len(), &16);
    }

    #[test]
    fn index_mut_range_from() {
        let mut uuid = UUID::nil();
        uuid[14..].copy_from_slice(&[0xAB, 0xCD]);
        assert_eq!(&uuid[14..], &[0xAB, 0xCD]);
    }

    // -------------------------------------------------------------------------
    // Range indexing tests: RangeTo<usize> (..b)
    // -------------------------------------------------------------------------

    #[test]
    fn index_range_to() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);
        assert_eq!(&uuid[..2], &[0x01, 0x02]);
        assert_eq!(&uuid[..16].len(), &16);
    }

    #[test]
    fn index_mut_range_to() {
        let mut uuid = UUID::nil();
        uuid[..2].copy_from_slice(&[0xAB, 0xCD]);
        assert_eq!(&uuid[..2], &[0xAB, 0xCD]);
    }

    // -------------------------------------------------------------------------
    // Range indexing tests: RangeFull (..)
    // -------------------------------------------------------------------------

    #[test]
    fn index_range_full_slice() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);
        assert_eq!(&uuid[..], uuid.as_bytes().as_slice());
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn index_mut_range_full() {
        let mut uuid = UUID::nil();
        uuid[..].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        for i in 0..16 {
            assert_eq!(uuid[i], (i + 1) as u8);
        }
    }

    // -------------------------------------------------------------------------
    // Range indexing tests: RangeInclusive<usize> (a..=b)
    // -------------------------------------------------------------------------

    #[test]
    fn index_range_inclusive() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);
        assert_eq!(&uuid[0..=3], &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[14..=15], &[0x0f, 0x10]);
    }

    #[test]
    fn index_mut_range_inclusive() {
        let mut uuid = UUID::nil();
        uuid[0..=3].copy_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(&uuid[0..=3], &[0x01, 0x02, 0x03, 0x04]);
    }

    // -------------------------------------------------------------------------
    // Range indexing tests: RangeToInclusive<usize> (..=b)
    // -------------------------------------------------------------------------

    #[test]
    fn index_range_to_inclusive() {
        let uuid = UUID::from(0x0102_0304_0506_0708_090a_0b0c_0d0e_0f10u128);
        assert_eq!(&uuid[..=1], &[0x01, 0x02]);
        assert_eq!(&uuid[..=15].len(), &16);
    }

    #[test]
    fn index_mut_range_to_inclusive() {
        let mut uuid = UUID::nil();
        uuid[..=1].copy_from_slice(&[0xAB, 0xCD]);
        assert_eq!(&uuid[..=1], &[0xAB, 0xCD]);
    }

    // -------------------------------------------------------------------------
    // Big-endian byte order verification
    // -------------------------------------------------------------------------

    #[test]
    fn big_endian_byte_order() {
        // UUID stores bytes in big-endian order
        // The value 0x0123456789ABCDEF should have:
        // - bytes[0..8] = 00 00 00 00 00 00 00 00 (high bytes)
        // - bytes[8..16] = 01 23 45 67 89 AB CD EF (low bytes)
        let uuid = UUID::from(0x0123_4567_89AB_CDEFu128);

        // First 8 bytes are zero (high bits)
        for i in 0..8 {
            assert_eq!(uuid[i], 0x00, "byte {i} should be 0");
        }

        // Last 8 bytes contain the value
        assert_eq!(uuid[8], 0x01);
        assert_eq!(uuid[9], 0x23);
        assert_eq!(uuid[10], 0x45);
        assert_eq!(uuid[11], 0x67);
        assert_eq!(uuid[12], 0x89);
        assert_eq!(uuid[13], 0xAB);
        assert_eq!(uuid[14], 0xCD);
        assert_eq!(uuid[15], 0xEF);
    }

    // -------------------------------------------------------------------------
    // Consistency with as_bytes()
    // -------------------------------------------------------------------------

    #[test]
    fn index_consistent_with_as_bytes() {
        let uuid = UUID::from(0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210u128);
        let bytes = uuid.as_bytes();

        for i in 0..16 {
            assert_eq!(uuid[i], bytes[i], "index[{i}] differs from as_bytes()[{i}]");
        }
    }

    #[test]
    fn index_range_consistent_with_as_bytes() {
        let uuid = UUID::from(0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210u128);
        let bytes = uuid.as_bytes();

        assert_eq!(&uuid[0..8], &bytes[0..8]);
        assert_eq!(&uuid[8..16], &bytes[8..16]);
        assert_eq!(&uuid[..], bytes.as_slice());
    }
}
