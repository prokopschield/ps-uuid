#![allow(clippy::cast_possible_truncation)]
use std::{
    ops::BitXor,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{UuidConstructionError, UUID};

impl UUID {
    /// Creates a new DCOM UUID using the specified timestamp and node ID.
    ///
    /// This generates a DCOM UUID following Microsoft's DCOM specification,
    /// which predates RFC 4122 and uses its own format and algorithms.
    ///
    /// # Arguments
    /// * `timestamp` - The system time to use for the UUID
    /// * `node_id` - The node ID (6 bytes)
    ///
    /// # Errors
    /// - [`UuidConstructionError`] is returned if `timestamp` is out of range.
    pub fn new_dcom(
        timestamp: SystemTime,
        node_id: [u8; 6],
    ) -> Result<Self, UuidConstructionError> {
        // DCOM uses Windows FILETIME format: 100ns intervals since Jan 1, 1601
        const FILETIME_EPOCH_OFFSET: u64 = 116_444_736_000_000_000;

        let duration_since_unix = timestamp
            .duration_since(UNIX_EPOCH)
            .map_err(|_| UuidConstructionError::TimestampBeforeEpoch)?;

        let filetime = u64::try_from(duration_since_unix.as_nanos() / 100)
            .map_err(|_| UuidConstructionError::TimestampOverflow)?
            .checked_add(FILETIME_EPOCH_OFFSET)
            .ok_or(UuidConstructionError::TimestampOverflow)?;

        // DCOM time layout (different from RFC 4122)
        let time_low = (filetime & 0xFFFF_FFFF) as u32;
        let time_mid = ((filetime >> 32) & 0xFFFF) as u16;
        let time_hi_and_version = ((filetime >> 48) & 0xFFFF) as u16;

        // DCOM clock sequence generation
        let clock_seq = ((filetime & 0xFFFF) as u16)
            .wrapping_mul(0x1234)
            .bitxor(((filetime >> 16) & 0xFFFF) as u16);

        Ok(Self::from_parts_dcom(
            time_low,
            time_mid,
            time_hi_and_version,
            clock_seq,
            node_id,
        ))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::{
        ops::BitXor,
        time::{Duration, UNIX_EPOCH},
    };

    use crate::{UuidConstructionError, Variant, UUID};

    const FILETIME_EPOCH_OFFSET: u64 = 116_444_736_000_000_000;

    const fn sample_node_id() -> [u8; 6] {
        [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]
    }

    #[test]
    fn test_new_dcom_basic_functionality() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1_000_000);
        let node_id = sample_node_id();

        let result = UUID::new_dcom(timestamp, node_id);
        assert!(result.is_ok());

        let uuid = result.unwrap();
        assert_eq!(uuid.variant(), Variant::DCOM);
    }

    #[test]
    fn test_new_dcom_unix_epoch() {
        let timestamp = UNIX_EPOCH;
        let node_id = sample_node_id();

        let result = UUID::new_dcom(timestamp, node_id);
        assert!(result.is_ok());

        let uuid = result.unwrap();
        assert_eq!(uuid.variant(), Variant::DCOM);
    }

    #[test]
    fn test_new_dcom_before_unix_epoch() {
        let timestamp = UNIX_EPOCH - Duration::from_secs(1);
        let node_id = sample_node_id();

        let result = UUID::new_dcom(timestamp, node_id);
        assert!(matches!(
            result,
            Err(UuidConstructionError::TimestampBeforeEpoch)
        ));
    }

    #[test]
    fn test_new_dcom_filetime_calculation() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1);
        let node_id = sample_node_id();

        let uuid = UUID::new_dcom(timestamp, node_id).unwrap();

        // Expected FILETIME: 1 second * 10_000_000 (100ns units) + offset
        let expected_filetime = 10_000_000 + FILETIME_EPOCH_OFFSET;

        // Extract the timestamp components and verify
        let time_low =
            u32::from_le_bytes([uuid.bytes[0], uuid.bytes[1], uuid.bytes[2], uuid.bytes[3]]);
        let time_mid = u16::from_le_bytes([uuid.bytes[4], uuid.bytes[5]]);
        let time_hi = u16::from_le_bytes([uuid.bytes[6], uuid.bytes[7]]);

        let reconstructed_filetime =
            u64::from(time_low) | (u64::from(time_mid) << 32) | (u64::from(time_hi) << 48);

        assert_eq!(reconstructed_filetime, expected_filetime);
    }

    #[test]
    fn test_new_dcom_clock_sequence_algorithm() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(12345);
        let node_id = sample_node_id();

        let uuid = UUID::new_dcom(timestamp, node_id).unwrap();

        // Calculate expected clock sequence
        let filetime = 12345 * 10_000_000 + FILETIME_EPOCH_OFFSET;
        let expected_clock_seq = ((filetime & 0xFFFF) as u16)
            .wrapping_mul(0x1234)
            .bitxor(((filetime >> 16) & 0xFFFF) as u16);

        // Extract clock sequence from UUID (bytes 8-9, big-endian)
        let actual_clock_seq = u16::from_be_bytes([uuid.bytes[8], uuid.bytes[9]]);

        // Mask to 14 bits (DCOM variant sets upper 2 bits)
        let actual_clock_seq_masked = actual_clock_seq & 0x3FFF;
        let expected_clock_seq_masked = expected_clock_seq & 0x3FFF;

        assert_eq!(actual_clock_seq_masked, expected_clock_seq_masked);
    }

    #[test]
    fn test_new_dcom_deterministic() {
        let timestamp = UNIX_EPOCH + Duration::from_millis(123_456_789);
        let node_id = sample_node_id();

        let uuid1 = UUID::new_dcom(timestamp, node_id).unwrap();
        let uuid2 = UUID::new_dcom(timestamp, node_id).unwrap();

        assert_eq!(uuid1, uuid2);
    }

    #[test]
    fn test_new_dcom_different_timestamps() {
        let node_id = sample_node_id();

        let uuid1 = UUID::new_dcom(UNIX_EPOCH + Duration::from_secs(1), node_id).unwrap();
        let uuid2 = UUID::new_dcom(UNIX_EPOCH + Duration::from_secs(2), node_id).unwrap();

        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_new_dcom_different_node_ids() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1000);
        let node_id1 = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let node_id2 = [0x06, 0x05, 0x04, 0x03, 0x02, 0x01];

        let uuid1 = UUID::new_dcom(timestamp, node_id1).unwrap();
        let uuid2 = UUID::new_dcom(timestamp, node_id2).unwrap();

        assert_ne!(uuid1, uuid2);

        // Verify node IDs are correctly set (bytes 10-15)
        assert_eq!(&uuid1.bytes[10..16], &node_id1);
        assert_eq!(&uuid2.bytes[10..16], &node_id2);
    }

    #[test]
    fn test_new_dcom_nanosecond_precision() {
        let node_id = sample_node_id();

        // Test with nanosecond precision
        let timestamp1 = UNIX_EPOCH + Duration::new(1000, 100); // 100 ns
        let timestamp2 = UNIX_EPOCH + Duration::new(1000, 199); // 199 ns

        let uuid1 = UUID::new_dcom(timestamp1, node_id).unwrap();
        let uuid2 = UUID::new_dcom(timestamp2, node_id).unwrap();

        // Should be the same due to 100ns precision truncation
        assert_eq!(uuid1, uuid2);

        // But 1000ns difference should be different
        let timestamp3 = UNIX_EPOCH + Duration::new(1000, 1000); // 1000 ns
        let uuid3 = UUID::new_dcom(timestamp3, node_id).unwrap();

        assert_ne!(uuid1, uuid3);
    }

    #[test]
    fn test_new_dcom_far_future() {
        let node_id = sample_node_id();

        // Test with a far future timestamp (year 2100)
        let far_future = UNIX_EPOCH + Duration::from_secs(365 * 24 * 3600 * 130); // ~130 years

        let result = UUID::new_dcom(far_future, node_id);
        assert!(result.is_ok());

        let uuid = result.unwrap();
        assert_eq!(uuid.variant(), Variant::DCOM);
    }

    #[test]
    fn test_new_dcom_timestamp_overflow() {
        let node_id = sample_node_id();

        // Create a timestamp that would cause overflow when converted to nanoseconds
        // This is near the limit of what Duration can represent
        let max_duration = Duration::new(u64::MAX / 10_000_000, 999_999_999);
        let overflow_timestamp = UNIX_EPOCH + max_duration;

        let result = UUID::new_dcom(overflow_timestamp, node_id);

        assert_eq!(result, Err(UuidConstructionError::TimestampOverflow));
    }

    #[test]
    fn test_new_dcom_all_zero_node_id() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1000);
        let node_id = [0x00; 6];

        let result = UUID::new_dcom(timestamp, node_id);
        assert!(result.is_ok());

        let uuid = result.unwrap();
        assert_eq!(&uuid.bytes[10..16], &node_id);
    }

    #[test]
    fn test_new_dcom_all_ff_node_id() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1000);
        let node_id = [0xFF; 6];

        let result = UUID::new_dcom(timestamp, node_id);
        assert!(result.is_ok());

        let uuid = result.unwrap();
        assert_eq!(&uuid.bytes[10..16], &node_id);
    }

    #[test]
    fn test_new_dcom_endianness() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(0x1234_5678);
        let node_id = sample_node_id();

        let uuid = UUID::new_dcom(timestamp, node_id).unwrap();

        // Verify little-endian encoding for time fields
        let expected_filetime = 0x1234_5678u64 * 10_000_000 + FILETIME_EPOCH_OFFSET;

        // time_low should be little-endian in bytes 0-3
        let time_low_bytes = (expected_filetime as u32).to_le_bytes();
        assert_eq!(&uuid.bytes[0..4], &time_low_bytes);

        // time_mid should be little-endian in bytes 4-5
        let time_mid_bytes = (((expected_filetime >> 32) & 0xFFFF) as u16).to_le_bytes();
        assert_eq!(&uuid.bytes[4..6], &time_mid_bytes);

        // time_hi should be little-endian in bytes 6-7
        let time_hi_bytes = (((expected_filetime >> 48) & 0xFFFF) as u16).to_le_bytes();
        assert_eq!(&uuid.bytes[6..8], &time_hi_bytes);
    }

    #[test]
    fn test_new_dcom_variant_bits() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1000);
        let node_id = sample_node_id();

        let uuid = UUID::new_dcom(timestamp, node_id).unwrap();

        // Verify DCOM variant bits (110) are set in byte 8, bits 7-5
        let byte_8 = uuid.bytes[8];
        let variant_bits = (byte_8 & 0xE0) >> 5; // Extract bits 7-5
        assert_eq!(variant_bits, 0b110); // DCOM variant
    }
}
