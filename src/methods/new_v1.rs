use crate::{UuidConstructionError, UUID};
use std::time::SystemTime;

impl UUID {
    /// Create an RFC 4122 version-1 (time-based) UUID
    /// from a `SystemTime` and a 48-bit node identifier.
    ///
    /// A pseudo-random clock sequence value is used.
    ///
    /// # Errors
    /// - `TimestampBeforeEpoch` is returned if `time` predates 1582-10-15.
    /// - `TimestampOverflow` is returned if `time` exceeds 5236-03-31.
    pub fn new_v1(time: SystemTime, node_id: [u8; 6]) -> Result<Self, UuidConstructionError> {
        // ------------------------------------------------------------------
        // 1. Convert time -> 100 ns ticks since Gregorian epoch
        // ------------------------------------------------------------------
        let ticks = Self::system_time_to_ticks(time)?;

        // ------------------------------------------------------------------
        // 2. Split the 60-bit timestamp into the three wire fields
        // ------------------------------------------------------------------
        let time_low: u32 = (ticks & 0xFFFF_FFFF) as u32;
        let time_mid: u16 = ((ticks >> 32) & 0xFFFF) as u16;
        let time_hi: u16 = ((ticks >> 48) & 0x0FFF) as u16; // only 12 bits

        // ------------------------------------------------------------------
        // 3. Construct UUID
        // ------------------------------------------------------------------

        let clock_seq: u16 = rand::random::<u16>() & 0x3FFF;

        Ok(Self::from_parts_v1(
            time_low, time_mid, time_hi, clock_seq, node_id,
        ))
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};

    use super::*;
    use crate::{Gregorian, Variant, UUID};

    // Helper: build the “ground truth” through from_parts_v1
    fn manual(time: SystemTime, node: [u8; 6]) -> UUID {
        let dur = time.duration_since(Gregorian::epoch()).unwrap();
        let ticks = dur.as_secs() * 10_000_000 + u64::from(dur.subsec_nanos() / 100);

        let time_low = (ticks & 0xFFFF_FFFF) as u32;
        let time_mid = ((ticks >> 32) & 0xFFFF) as u16;
        let time_hi = ((ticks >> 48) & 0x0FFF) as u16;
        let cs = 0x2A3Bu16; // arbitrary but deterministic for manual build

        UUID::from_parts_v1(time_low, time_mid, time_hi, cs, node)
    }

    #[test]
    fn builds_same_bytes_as_manual_version() {
        let t = UNIX_EPOCH + Duration::from_secs(1_700_000_000); // 2023-11-14
        let mac = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        let auto = UUID::new_v1(t, mac).unwrap();
        let bytes = auto.as_bytes();

        // Structural guarantees -------------------------------------------
        assert_eq!(auto.version(), Some(1));
        assert_eq!(auto.variant(), Variant::OSF);

        // Check timestamp and node id equality except for the clock sequence
        let manual = manual(t, mac);
        // time_low, time_mid, time_hi
        assert_eq!(&bytes[0..8], &manual.as_bytes()[0..8]);
        // node id
        assert_eq!(&bytes[10..16], &mac);
    }

    #[test]
    fn timestamp_before_gregorian_is_rejected() {
        // 31 Dec 1400 00:00:00 UTC
        let ancient = UNIX_EPOCH - Duration::from_secs(17_834_668_800);
        let err = UUID::new_v1(ancient, [0; 6]).unwrap_err();
        assert_eq!(err, UuidConstructionError::TimestampBeforeEpoch);
    }

    #[test]
    fn timestamp_overflow_is_rejected() {
        // 60-bit tick field can hold ~ 36 089  days  … choose a far-future date
        let too_far = UNIX_EPOCH
            + Duration::from_secs(
                u64::try_from(1u128 << 60).unwrap() / 10_000_000 + 12_219_292_800 + 10,
            );

        let err = UUID::new_v1(too_far, [0; 6]).unwrap_err();
        assert_eq!(err, UuidConstructionError::TimestampOverflow);
    }

    #[test]
    fn variant_and_version_bits_are_correct() {
        let uuid = UUID::new_v1(SystemTime::now(), [1, 2, 3, 4, 5, 6]).unwrap();
        let b = uuid.as_bytes();

        // Variant = 10xxxxxx
        assert_eq!(b[8] >> 6, 0b10);
        // Version = 1
        assert_eq!(b[6] >> 4, 0b0001);
    }

    #[test]
    fn new_v1_rejects_time_before_1582_10_15() {
        // 1582-10-15 00:00:00 UTC is 12 216 652 800 s before the Unix epoch.
        let before_gregorian = Gregorian::epoch() - Duration::from_secs(1);

        eprintln!("{before_gregorian:?}");

        let err = UUID::new_v1(before_gregorian, [0; 6])
            .expect_err("timestamp prior to Gregorian epoch must fail");

        assert!(
            matches!(err, UuidConstructionError::TimestampBeforeEpoch),
            "wrong error variant: got {err:?}"
        );
    }

    #[test]
    fn new_v1_rejects_time_after_5236_03_31() {
        // One tick past the maximum 0x0FFF_FFFF_FFFF_FFFF timestamp (ticks are 100 ns).
        const MAX_TICKS: u64 = 0x0FFF_FFFF_FFFF_FFFF;
        let overflow = SystemTime::UNIX_EPOCH + Duration::from_nanos(MAX_TICKS + 1) * 100;

        let err = UUID::new_v1(overflow, [0; 6]).expect_err("timestamp overflow must fail");

        assert!(
            matches!(err, UuidConstructionError::TimestampOverflow),
            "wrong error variant: got {err:?}"
        );
    }
}
