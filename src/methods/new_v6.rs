use crate::{UuidConstructionError, UUID};
use std::time::SystemTime;

impl UUID {
    /// Create an RFC 4122 **version-6** (time-ordered) UUID from a
    /// `SystemTime`, a 14-bit clock sequence and a 48-bit node identifier.
    ///
    /// # Errors
    /// - `TimestampBeforeEpoch` if `time` predates 1582-10-15.
    /// - `TimestampOverflow`    if the 60-bit tick counter would overflow.
    pub fn new_v6(
        time: SystemTime,
        clock_seq: u16,
        node_id: [u8; 6],
    ) -> Result<Self, UuidConstructionError> {
        // 1. Convert wall-clock time → 100 ns ticks since 1582-10-15
        let ticks = Self::system_time_to_ticks(time)?;

        // 2. Split the 60-bit timestamp into (high, mid, low) pieces
        let time_high: u32 = ((ticks >> 28) & 0xFFFF_FFFF) as u32; // most-significant 32
        let time_mid: u16 = ((ticks >> 12) & 0xFFFF) as u16; // next 16
        let time_low: u16 = (ticks & 0x0FFF) as u16; // least-significant 12

        // 3. Assemble the UUID
        Ok(Self::from_parts_v6(
            time_high, time_mid, time_low, clock_seq, node_id,
        ))
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Gregorian, Variant};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    // Manual reference builder using from_parts_v6 -------------------------
    fn manual(time: SystemTime, node: [u8; 6]) -> UUID {
        let dur = time.duration_since(Gregorian::epoch()).unwrap();
        let ticks = dur.as_secs() * 10_000_000 + u64::from(dur.subsec_nanos() / 100);

        let time_high = ((ticks >> 28) & 0xFFFF_FFFF) as u32;
        let time_mid = ((ticks >> 12) & 0xFFFF) as u16;
        let time_low = (ticks & 0x0FFF) as u16;
        let cs = 0x2A3Bu16; // deterministic for testing

        UUID::from_parts_v6(time_high, time_mid, time_low, cs, node)
    }

    #[test]
    fn builds_same_bytes_as_manual_version() {
        let t = UNIX_EPOCH + Duration::from_secs(1_700_000_000); // 2023-11-14
        let mac = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let auto = UUID::new_v6(t, rand::random(), mac).unwrap();
        let bytes = auto.as_bytes();

        // Structural guarantees
        assert_eq!(auto.get_version(), Some(6));
        assert_eq!(auto.get_variant(), Variant::OSF);

        // Timestamp (first 8 bytes) must match reference
        let manual = manual(t, mac);
        assert_eq!(&bytes[0..8], &manual.as_bytes()[0..8]);

        // Node identifier
        assert_eq!(&bytes[10..16], &mac);
    }

    #[test]
    fn timestamp_before_gregorian_is_rejected() {
        // 31 Dec 1400 00:00:00 UTC
        let ancient = UNIX_EPOCH - Duration::from_secs(17_834_668_800);
        let err = UUID::new_v6(ancient, rand::random(), [0; 6]).unwrap_err();
        assert_eq!(err, UuidConstructionError::TimestampBeforeEpoch);
    }

    #[test]
    fn timestamp_overflow_is_rejected() {
        const MAX_TICKS: u64 = 0x0FFF_FFFF_FFFF_FFFF; // 60 bits all 1
        let too_far = UNIX_EPOCH + Duration::from_nanos(MAX_TICKS + 1) * 100;

        let err = UUID::new_v6(too_far, rand::random(), [0; 6]).unwrap_err();
        assert_eq!(err, UuidConstructionError::TimestampOverflow);
    }

    #[test]
    fn variant_and_version_bits_are_correct() {
        let uuid = UUID::new_v6(SystemTime::now(), rand::random(), [1, 2, 3, 4, 5, 6]).unwrap();
        let b = uuid.as_bytes();

        // Variant = 10xxxxxx
        assert_eq!(b[8] >> 6, 0b10);
        // Version = 6
        assert_eq!(b[6] >> 4, 0b0110);
    }

    #[test]
    fn new_v6_rejects_time_before_1582_10_15() {
        let before_gregorian = Gregorian::epoch() - Duration::from_secs(1);
        let err = UUID::new_v6(before_gregorian, rand::random(), [0; 6]).unwrap_err();
        assert_eq!(err, UuidConstructionError::TimestampBeforeEpoch);
    }

    #[test]
    fn new_v6_rejects_time_after_5236_03_31() {
        const MAX_TICKS: u64 = 0x0FFF_FFFF_FFFF_FFFF;
        let overflow = UNIX_EPOCH + Duration::from_nanos(MAX_TICKS + 1) * 100;
        let err = UUID::new_v6(overflow, rand::random(), [0; 6]).unwrap_err();
        assert_eq!(err, UuidConstructionError::TimestampOverflow);
    }
}
