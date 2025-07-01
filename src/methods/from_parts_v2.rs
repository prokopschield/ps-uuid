use crate::UUID;

impl UUID {
    /// Build a DCE Security (v2) UUID from its individual fields.
    #[must_use]
    pub fn from_parts_v2(
        domain: u8,
        local_id: u32,
        time_mid: u16,
        time_hi: u16,
        clock_seq: u16,
        node_id: [u8; 6],
    ) -> Self {
        // `local_id` occupies the time-low field, so just forward it.
        let mut uuid = Self::from_parts_v1(local_id, time_mid, time_hi, clock_seq, node_id);

        uuid.bytes[9] = domain; // clock_seq_low -> Domain
        uuid.with_version(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{NodeId, UUID};

    const fn variant_rfc_4122(byte: u8) -> bool {
        (byte & 0b1100_0000) == 0b1000_0000
    }

    #[test]
    fn from_parts_v2_encodes_every_field() {
        let domain = 0xAC;
        let local_id = 0x1234_5678;
        let time_mid = 0x9ABC;
        let time_hi = 0xDEF0;
        let clock_seq = 0x1357;
        let node_id = NodeId::random();

        let u = UUID::from_parts_v2(domain, local_id, time_mid, time_hi, clock_seq, *node_id);

        // time_low == local_id (big-endian)
        assert_eq!(&u.bytes[0..4], &local_id.to_be_bytes());

        // time_mid
        assert_eq!(&u.bytes[4..6], &time_mid.to_be_bytes());

        // time_hi_and_version (lower 12 bits from `time_hi`, upper nibble = 2)
        let expected_hi = ((time_hi & 0x0FFF) | 0x2000).to_be_bytes();
        assert_eq!(&u.bytes[6..8], &expected_hi);

        // Variant bits = 10
        assert!(variant_rfc_4122(u.bytes[8]));

        // Domain
        assert_eq!(u.bytes[9], domain);

        // Node ID
        assert_eq!(u.bytes[10..16], *node_id);

        // Public API
        assert_eq!(u.get_version(), Some(2));
    }
}
