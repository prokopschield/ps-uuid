use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::UUID;

const NCS_EPOCH: Duration = Duration::from_secs(315_532_800); // January 1, 1980 (Unix epoch + 10 years)
const MAX_TIMESTAMP: u64 = (1u64 << 48) - 1; // Max value for 48-bit timestamp
const NCS_VARIANT_MASK: u8 = 0b1000_0000; // NCS variant bit (0xxx₂)

#[allow(clippy::module_name_repetitions)]
#[derive(thiserror::Error, Debug)]
pub enum NcsUuidError {
    #[error("Address family out of range")]
    AddressFamilyOutOfRange,
    #[error("Timestamp is before 1980-01-01")]
    TimestampBeforeEpoch,
    #[error("Timestamp is after 2015-09-05T05:58:26.842Z")]
    TimestampOverflow,
}

impl UUID {
    /// Generates a new NCS UUID (Variant 0) from a timestamp, address family, and node ID.
    ///
    /// # Parameters
    /// - `timestamp`: System time for UUID generation.
    /// - `address_family`: Network address family (0–13, per NCS specification).
    /// - `address`: 7-byte node ID (e.g., extended MAC address or unique host ID).
    ///
    /// # NCS UUID Structure
    /// - Timestamp (48 bits): 4-microsecond units since 1980-01-01 00:00 UTC.
    /// - Reserved (16 bits): Set to 0.
    /// - Address Family (8 bits): Network type (0–13).
    /// - Node ID (56 bits): Unique host identifier.
    ///
    /// # Returns
    /// - `Ok(UUID)` on success.
    /// - `Err(GenNcsError)` if the timestamp is invalid or the address family is out of range.
    ///
    /// # Errors
    /// - `AddressFamilyOutOfRange` is returned if `address_family` doesn't satisfy `0..=13`
    /// - `TimestampBeforeEpoch` is returned if `timestamp` is before `1980-01-01`
    /// - `TimestampOverflow` is returned if `timestamp` is after `2015-09-05T05:58:26.842Z`
    ///
    /// # Example
    /// ```rust
    /// use std::time::{SystemTime, Duration};
    /// let time = SystemTime::UNIX_EPOCH + Duration::from_secs(315532800 + 3600);
    /// let uuid = ps_uuid::UUID::new_ncs(time, 2, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    /// ```
    pub fn new_ncs(
        timestamp: SystemTime,
        address_family: u8,
        address: &[u8; 7],
    ) -> Result<Self, NcsUuidError> {
        // Validate address family (0–13 for NCS compatibility)
        if address_family > 13 {
            return Err(NcsUuidError::AddressFamilyOutOfRange);
        }

        // Compute duration since NCS epoch (1980-01-01)
        let duration = timestamp
            .duration_since(UNIX_EPOCH + NCS_EPOCH)
            .map_err(|_| NcsUuidError::TimestampBeforeEpoch)?;

        // Convert to 4-microsecond units
        let timestamp = duration.as_micros() / 4;

        // Check if timestamp fits in 48 bits
        if timestamp > MAX_TIMESTAMP.into() {
            return Err(NcsUuidError::TimestampOverflow);
        }

        // Initialize 128-bit UUID bytes
        // Set 48-bit timestamp (big-endian, first 6 bytes)
        let mut bytes = (timestamp << 80).to_be_bytes();

        // Set address family (byte 8)
        bytes[8] = address_family;

        // Set node ID (bytes 9–15)
        bytes[9..16].copy_from_slice(address);

        // Set NCS variant (0xxx₂ in most significant bits of byte 8)
        bytes[8] &= !NCS_VARIANT_MASK;

        Ok(Self { bytes })
    }

    /// Returns the UUID as a byte array.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ncs_uuid() -> Result<(), NcsUuidError> {
        let time = UNIX_EPOCH + NCS_EPOCH + Duration::from_secs(3600); // 1 hour after epoch
        let address = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let uuid = UUID::new_ncs(time, 2, &address)?;
        let bytes = uuid.as_bytes();
        // Check timestamp (3600s = 900,000,000 4μs units)
        let expected_timestamp = (900_000_000u64).to_be_bytes();
        assert_eq!(&bytes[0..6], &expected_timestamp[2..8]);
        // Check reserved bits
        assert_eq!(&bytes[6..8], &[0, 0]);
        // Check address family and variant
        assert_eq!(bytes[8], 2);
        // Check node ID
        assert_eq!(&bytes[9..16], &address);
        Ok(())
    }

    #[test]
    fn test_timestamp_before_epoch() {
        let time = UNIX_EPOCH; // Before 1980
        let address = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let result = UUID::new_ncs(time, 2, &address);
        assert!(matches!(result, Err(NcsUuidError::TimestampBeforeEpoch)));
    }

    #[test]
    fn test_invalid_address_family() {
        let time = UNIX_EPOCH + NCS_EPOCH + Duration::from_secs(3600);
        let address = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let result = UUID::new_ncs(time, 14, &address);
        assert!(matches!(result, Err(NcsUuidError::AddressFamilyOutOfRange)));
    }

    #[test]
    fn test_nil_uuid() -> Result<(), NcsUuidError> {
        let ncs_nil = UUID::new_ncs(UNIX_EPOCH + NCS_EPOCH, 0, &[0, 0, 0, 0, 0, 0, 0])?;
        let nil = UUID::nil();

        assert_eq!(ncs_nil, nil, "UUIDs should be equal.");

        Ok(())
    }
}
