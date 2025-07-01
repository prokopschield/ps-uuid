use std::time::SystemTime;

use crate::{UuidConstructionError, UUID};

impl UUID {
    /// Generate a v2 UUID from wall-clock time plus the extra DCE fields.
    ///
    /// # Errors
    /// - `TimestampBeforeEpoch` is returned if `time` predates 1582-10-15.
    /// - `TimestampOverflow` is returned if `time` exceeds 5236-03-31.
    pub fn new_v2(
        domain: u8,
        local_id: u32,
        time: SystemTime,
        clock_seq: u16,
        node_id: [u8; 6],
    ) -> Result<Self, UuidConstructionError> {
        let mut uuid = Self::new_v1(time, clock_seq, node_id)?;

        uuid.bytes[0..4].copy_from_slice(&local_id.to_be_bytes());
        uuid.bytes[9] = domain;

        Ok(uuid.with_version(2))
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use crate::{NodeId, UUID};

    const fn variant_rfc_4122(byte: u8) -> bool {
        (byte & 0b1100_0000) == 0b1000_0000
    }

    #[test]
    fn new_v2_sets_local_id_and_domain() {
        let domain = 3;
        let local_id = 0xDEAD_BEEF;
        let time = SystemTime::UNIX_EPOCH + Duration::from_secs(1);
        let clock_seq = 0x1FFF;
        let node_id = NodeId::random();

        let u =
            UUID::new_v2(domain, local_id, time, clock_seq, *node_id).expect("new_v2 must succeed");

        // Same fixed-field checks as above
        assert_eq!(&u.bytes[0..4], &local_id.to_be_bytes());
        assert_eq!(u.bytes[9], domain);
        assert!(variant_rfc_4122(u.bytes[8]));
        assert_eq!(u.bytes[10..16], *node_id);
        assert_eq!(u.get_version(), Some(2));
    }

    #[test]
    fn new_v2_distinguishes_different_local_ids() {
        let base_time = SystemTime::UNIX_EPOCH;
        let node_id = NodeId::random();
        let a = UUID::new_v2(0, 1, base_time, 1, *node_id).unwrap();
        let b = UUID::new_v2(0, 2, base_time, 1, *node_id).unwrap();
        assert_ne!(a.bytes, b.bytes);
    }

    #[test]
    fn new_v2_distinguishes_different_domains() {
        let base_time = SystemTime::UNIX_EPOCH;
        let node_id = NodeId::random();
        let a = UUID::new_v2(1, 0, base_time, 1, *node_id).unwrap();
        let b = UUID::new_v2(2, 0, base_time, 1, *node_id).unwrap();
        assert_ne!(a.bytes, b.bytes);
    }
}
