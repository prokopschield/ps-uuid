use crate::UUID;

impl UUID {
    /// Constructs a new NCS UUID (Variant 0) from pre-computed timestamp bytes, address family, and address.
    ///
    /// # Parameters
    /// - `timestamp`: 6-byte timestamp in 4-microsecond units since 1980-01-01 00:00 UTC (big-endian).
    /// - `address_family`: Network address family (0-13).
    /// - `address`: 7-byte node ID (e.g., extended MAC address or unique host identifier).
    ///
    /// # NCS UUID Structure
    /// - Timestamp (48 bits): Raw timestamp bytes in 4-microsecond units since 1980-01-01 00:00 UTC.
    /// - Reserved (16 bits): Set to 0.
    /// - Address Family (8 bits): Network type identifier.
    /// - Node ID (56 bits): Unique host identifier.
    ///
    /// # Returns
    /// A new NCS UUID with the specified components.
    ///
    /// # Example
    /// ```rust
    /// let timestamp = [93, 16, 39, 53, 62, 58]; // Pre-computed timestamp bytes
    /// let address = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    /// let uuid = ps_uuid::UUID::from_parts_ncs(&timestamp, 2, &address);
    /// ```
    #[must_use]
    pub fn from_parts_ncs(timestamp: &[u8; 6], address_family: u8, address: &[u8; 7]) -> Self {
        let mut uuid = Self::nil();

        uuid.bytes[0..6].copy_from_slice(timestamp);

        // Set address family (byte 8)
        uuid.bytes[8] = address_family;

        // Set node ID (bytes 9–15)
        uuid.bytes[9..16].copy_from_slice(address);

        uuid.with_variant(crate::Variant::NSC)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ncs_uuid() {
        let timestamp = &[93, 16, 39, 53, 62, 58];
        let address = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let uuid = UUID::from_parts_ncs(timestamp, 2, &address);
        let bytes = uuid.as_bytes();

        assert_eq!(&bytes[0..6], timestamp);
        // Check reserved bits
        assert_eq!(&bytes[6..8], &[0, 0]);
        // Check address family and variant
        assert_eq!(bytes[8], 2);
        // Check node ID
        assert_eq!(&bytes[9..16], &address);
    }

    #[test]
    fn test_nil_uuid() {
        let ncs_nil = UUID::from_parts_ncs(&[0, 0, 0, 0, 0, 0], 0, &[0, 0, 0, 0, 0, 0, 0]);
        let nil = UUID::nil();

        assert_eq!(ncs_nil, nil, "UUIDs should be equal.");
    }
}
