use std::time::SystemTime;

use crate::{UuidConstructionError, UUID};

impl UUID {
    /// Generates a new DCOM UUID using the current system time and the provided node ID.
    ///
    /// # Arguments
    /// * `node_id` - The node ID (6 bytes)
    ///
    /// # Errors
    /// - [`UuidConstructionError`] is returned if the system time is out of range.
    pub fn gen_dcom(node_id: [u8; 6]) -> Result<Self, UuidConstructionError> {
        Self::new_dcom(SystemTime::now(), node_id)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn test_gen_dcom_success() {
        let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];
        let uuid = UUID::gen_dcom(node_id);
        assert!(
            uuid.is_ok(),
            "gen_dcom should succeed with current time and valid node_id"
        );
    }

    #[test]
    fn test_new_dcom_with_specific_node_id() {
        let timestamp = SystemTime::now();
        let node_id = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let uuid = UUID::new_dcom(timestamp, node_id);
        assert!(uuid.is_ok(), "new_dcom should succeed with valid node_id");
        // Optionally, check that the node_id is encoded in the UUID (if accessible)
    }

    #[test]
    fn test_new_dcom_timestamp_before_epoch() {
        let timestamp = UNIX_EPOCH - Duration::from_secs(1);
        let node_id = [0u8; 6];
        let result = UUID::new_dcom(timestamp, node_id);
        assert!(
            matches!(result, Err(UuidConstructionError::TimestampBeforeEpoch)),
            "Should error with TimestampBeforeEpoch"
        );
    }

    #[test]
    fn test_new_dcom_timestamp_overflow() {
        // Use a timestamp far in the future to trigger overflow
        let timestamp = UNIX_EPOCH + Duration::from_secs(u64::MAX / 2);
        let node_id = [0u8; 6];
        let result = UUID::new_dcom(timestamp, node_id);
        assert!(
            matches!(result, Err(UuidConstructionError::TimestampOverflow)),
            "Should error with TimestampOverflow"
        );
    }

    #[test]
    fn test_gen_dcom_determinism() {
        // Same node_id and close timestamps should yield different UUIDs
        let node_id = [1, 2, 3, 4, 5, 6];
        let uuid1 = UUID::gen_dcom(node_id).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let uuid2 = UUID::gen_dcom(node_id).unwrap();
        assert_ne!(
            uuid1, uuid2,
            "UUIDs generated at different times should differ"
        );
    }

    #[test]
    fn test_new_dcom_determinism() {
        // Same timestamp and node_id should yield the same UUID
        let timestamp = UNIX_EPOCH + Duration::from_secs(1_600_000_000);
        let node_id = [1, 2, 3, 4, 5, 6];
        let uuid1 = UUID::new_dcom(timestamp, node_id).unwrap();
        let uuid2 = UUID::new_dcom(timestamp, node_id).unwrap();
        assert_eq!(uuid1, uuid2, "Same input should yield same UUID");
    }
}
