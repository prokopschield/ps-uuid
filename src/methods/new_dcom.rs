#![allow(clippy::cast_possible_truncation)]
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    methods::{FILETIME_EPOCH_OFFSET, TICK},
    UuidConstructionError, UUID,
};

impl UUID {
    /// Creates a Microsoft (DCOM) variant UUID from a timestamp, clock sequence, and node ID.
    ///
    /// The timestamp is encoded as a Windows `FILETIME`, the number of
    /// 100-nanosecond intervals since 1601-01-01 00:00 UTC, stored
    /// little-endian across the first three fields to match the in-memory
    /// layout of a Microsoft `GUID`. The variant bits are set to
    /// [`Variant::DCOM`](crate::Variant::DCOM), so the result carries no RFC
    /// version. An input aligned to a 100-nanosecond interval round-trips
    /// through [`UUID::get_timestamp`]; a finer-grained input is floored to the
    /// start of its enclosing interval.
    ///
    /// The `FILETIME` layout is this crate's own encoding for time-based
    /// Microsoft-variant UUIDs. Microsoft-variant GUIDs in the wild, such as
    /// the COM interface identifiers `IUnknown` and `IDispatch`, are
    /// hand-allocated constants that embed no timestamp; Microsoft's own
    /// time-based GUIDs historically used the version-1 algorithm under the
    /// [`Variant::OSF`](crate::Variant::OSF) encoding. The little-endian field
    /// storage also means the canonical string form of the result differs from
    /// how Windows renders the same in-memory `GUID`: the first three groups
    /// appear byte-swapped.
    ///
    /// The clock sequence and node ID are supplied by the caller and serve the
    /// same disambiguating role as in a version-1 UUID; the variant bits
    /// overwrite the top three bits of the clock sequence, leaving 13 bits.
    /// See [`UUID::gen_dcom`] for a generator that manages the clock sequence
    /// automatically.
    ///
    /// # Errors
    /// - [`UuidConstructionError::TimestampBeforeEpoch`] is returned if `time` is before 1601-01-01, the start of the `FILETIME` epoch.
    /// - [`UuidConstructionError::TimestampOverflow`] is returned if `time` is too far in the future to encode.
    pub fn new_dcom(
        time: SystemTime,
        clock_seq: u16,
        node_id: [u8; 6],
    ) -> Result<Self, UuidConstructionError> {
        // FILETIME counts 100 ns ticks from 1601, so a time before the Unix
        // epoch sits below the offset rather than out of range.
        let filetime = match time.duration_since(UNIX_EPOCH) {
            Ok(after_epoch) => u64::try_from(after_epoch.as_nanos() / TICK.as_nanos())
                .map_err(|_| UuidConstructionError::TimestampOverflow)?
                .checked_add(FILETIME_EPOCH_OFFSET)
                .ok_or(UuidConstructionError::TimestampOverflow)?,
            Err(before_epoch) => {
                // Round toward the past so the result is the number of whole
                // 100 ns intervals elapsed since 1601, matching the post-epoch
                // branch. Rounding the magnitude up also rejects any instant
                // within 100 ns before 1601 rather than admitting it as 1601.
                let ticks_before_unix =
                    u64::try_from(before_epoch.duration().as_nanos().div_ceil(TICK.as_nanos()))
                        .map_err(|_| UuidConstructionError::TimestampBeforeEpoch)?;

                FILETIME_EPOCH_OFFSET
                    .checked_sub(ticks_before_unix)
                    .ok_or(UuidConstructionError::TimestampBeforeEpoch)?
            }
        };

        // Split the 64-bit FILETIME across the three time fields.
        let time_low = (filetime & 0xFFFF_FFFF) as u32;
        let time_mid = ((filetime >> 32) & 0xFFFF) as u16;
        let time_hi = ((filetime >> 48) & 0xFFFF) as u16;

        Ok(Self::from_parts_dcom(
            time_low, time_mid, time_hi, clock_seq, node_id,
        ))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use std::time::{Duration, UNIX_EPOCH};

    use crate::{methods::FILETIME_EPOCH_OFFSET, UuidConstructionError, Variant, UUID};

    const fn sample_node_id() -> [u8; 6] {
        [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]
    }

    #[test]
    fn sets_dcom_variant() {
        let uuid = UUID::new_dcom(
            UNIX_EPOCH + Duration::from_secs(1_000_000),
            0x1234,
            sample_node_id(),
        )
        .expect("new_dcom should succeed for a valid timestamp");

        assert_eq!(uuid.get_variant(), Variant::DCOM);
    }

    #[test]
    fn accepts_timestamp_before_unix_epoch() {
        let time = UNIX_EPOCH - Duration::from_secs(1);
        let uuid = UUID::new_dcom(time, 0, sample_node_id())
            .expect("new_dcom should succeed for timestamps within the FILETIME epoch");

        assert_eq!(
            uuid.get_timestamp(),
            Some(time),
            "a pre-1970 timestamp should round-trip through get_timestamp"
        );
    }

    #[test]
    fn accepts_filetime_epoch_boundary() {
        // The FILETIME epoch (1601-01-01T00:00:00Z) is FILETIME_EPOCH_OFFSET
        // ticks of 100 ns before the Unix epoch.
        let filetime_epoch = UNIX_EPOCH - Duration::from_secs(FILETIME_EPOCH_OFFSET / 10_000_000);
        let uuid = UUID::new_dcom(filetime_epoch, 0, sample_node_id())
            .expect("new_dcom should succeed at the start of the FILETIME epoch");

        assert_eq!(
            &uuid.bytes[0..8],
            &[0u8; 8],
            "the FILETIME epoch should encode to zero ticks"
        );
    }

    #[test]
    fn rejects_timestamp_before_filetime_epoch() {
        let before_filetime_epoch =
            UNIX_EPOCH - Duration::from_secs(FILETIME_EPOCH_OFFSET / 10_000_000 + 1);
        let result = UUID::new_dcom(before_filetime_epoch, 0, sample_node_id());

        assert_eq!(result, Err(UuidConstructionError::TimestampBeforeEpoch));
    }

    #[test]
    fn rejects_timestamp_just_before_filetime_epoch() {
        // 1 ns before 1601-01-01 must not round up into the representable range.
        let just_before = UNIX_EPOCH - Duration::new(FILETIME_EPOCH_OFFSET / 10_000_000, 1);
        let result = UUID::new_dcom(just_before, 0, sample_node_id());

        assert_eq!(result, Err(UuidConstructionError::TimestampBeforeEpoch));
    }

    #[test]
    fn rejects_timestamp_overflow() {
        let overflow = UNIX_EPOCH + Duration::new(u64::MAX / 10_000_000, 999_999_999);
        let result = UUID::new_dcom(overflow, 0, sample_node_id());

        assert_eq!(result, Err(UuidConstructionError::TimestampOverflow));
    }

    #[test]
    fn encodes_filetime_little_endian() {
        let uuid = UUID::new_dcom(UNIX_EPOCH + Duration::from_secs(1), 0, sample_node_id())
            .expect("new_dcom should succeed for a valid timestamp");

        let expected_filetime = 10_000_000 + FILETIME_EPOCH_OFFSET;

        let time_low =
            u32::from_le_bytes([uuid.bytes[0], uuid.bytes[1], uuid.bytes[2], uuid.bytes[3]]);
        let time_mid = u16::from_le_bytes([uuid.bytes[4], uuid.bytes[5]]);
        let time_hi = u16::from_le_bytes([uuid.bytes[6], uuid.bytes[7]]);
        let filetime =
            u64::from(time_low) | (u64::from(time_mid) << 32) | (u64::from(time_hi) << 48);

        assert_eq!(filetime, expected_filetime);
    }

    #[test]
    fn embeds_clock_seq_and_node_id() {
        let clock_seq = 0x0123;
        let uuid = UUID::new_dcom(
            UNIX_EPOCH + Duration::from_secs(1),
            clock_seq,
            sample_node_id(),
        )
        .expect("new_dcom should succeed for a valid timestamp");

        // The DCOM variant occupies the top three bits of byte 8, leaving the
        // low 13 bits of the clock sequence intact.
        let stored_clock_seq = u16::from_be_bytes([uuid.bytes[8], uuid.bytes[9]]) & 0x1FFF;

        assert_eq!(stored_clock_seq, clock_seq & 0x1FFF);
        assert_eq!(&uuid.bytes[10..16], &sample_node_id());
    }

    #[test]
    fn is_deterministic() {
        let time = UNIX_EPOCH + Duration::from_millis(123_456_789);

        let first = UUID::new_dcom(time, 0x2A3B, sample_node_id())
            .expect("new_dcom should succeed for a valid timestamp");
        let second = UUID::new_dcom(time, 0x2A3B, sample_node_id())
            .expect("new_dcom should succeed for a valid timestamp");

        assert_eq!(first, second);
    }

    #[test]
    fn distinct_timestamps_differ() {
        let node_id = sample_node_id();

        let first = UUID::new_dcom(UNIX_EPOCH + Duration::from_secs(1), 0, node_id)
            .expect("new_dcom should succeed for a valid timestamp");
        let second = UUID::new_dcom(UNIX_EPOCH + Duration::from_secs(2), 0, node_id)
            .expect("new_dcom should succeed for a valid timestamp");

        assert_ne!(first, second);
    }
}
