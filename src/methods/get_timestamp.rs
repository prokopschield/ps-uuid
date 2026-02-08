use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{Gregorian, UUID};

// FILETIME epoch: 1601-01-01T00:00:00Z
const FILETIME_EPOCH_OFFSET: u64 = 116_444_736_000_000_000;

// NCS epoch: 1980-01-01T00:00:00Z
const NCS_EPOCH: Duration = Duration::from_secs(315_532_800);

impl UUID {
    /// Extract the embedded timestamp as a `SystemTime`, if present.
    ///
    /// Returns `None` if the UUID does not encode a timestamp.
    #[must_use]
    pub fn get_timestamp(&self) -> Option<SystemTime> {
        match (self.get_version(), self.get_variant()) {
            // v1/v2: 60-bit timestamp, 100ns intervals since 1582-10-15
            (Some(1 | 2), crate::Variant::OSF) => {
                let time_low = u32::from_be_bytes([
                    self.bytes[0],
                    self.bytes[1],
                    self.bytes[2],
                    self.bytes[3],
                ]);
                let time_mid = u16::from_be_bytes([self.bytes[4], self.bytes[5]]);
                let time_hi = u16::from_be_bytes([self.bytes[6], self.bytes[7]]) & 0x0FFF;
                let timestamp: u64 =
                    (u64::from(time_hi) << 48) | (u64::from(time_mid) << 32) | u64::from(time_low);

                #[allow(clippy::cast_possible_truncation)]
                Some(
                    Gregorian::epoch()
                        + Duration::new(
                            timestamp / 10_000_000,
                            ((timestamp % 10_000_000) * 100) as u32,
                        ),
                )
            }
            // v6: 60-bit timestamp, 100ns intervals since 1582-10-15, reordered
            (Some(6), crate::Variant::OSF) => {
                let time_high = u32::from_be_bytes([
                    self.bytes[0],
                    self.bytes[1],
                    self.bytes[2],
                    self.bytes[3],
                ]);
                let time_mid = u16::from_be_bytes([self.bytes[4], self.bytes[5]]);
                let time_low = u16::from_be_bytes([self.bytes[6], self.bytes[7]]) & 0x0FFF;
                let timestamp: u64 = (u64::from(time_high) << 28)
                    | (u64::from(time_mid) << 12)
                    | u64::from(time_low);
                Some(Gregorian::epoch() + Duration::from_nanos(timestamp * 100))
            }
            // v7: 48-bit Unix ms timestamp, bytes 0..6
            (Some(7), crate::Variant::OSF) => {
                let mut ms_bytes = [0u8; 8];
                ms_bytes[2..8].copy_from_slice(&self.bytes[0..6]);
                let ms = u64::from_be_bytes(ms_bytes);
                Some(UNIX_EPOCH + Duration::from_millis(ms))
            }
            // DCOM: FILETIME, 100ns since 1601-01-01, little-endian
            (_, crate::Variant::DCOM) => {
                // time_low: bytes 0..4 (LE), time_mid: 4..6 (LE), time_hi: 6..8 (LE)
                let time_low = u32::from_le_bytes([
                    self.bytes[0],
                    self.bytes[1],
                    self.bytes[2],
                    self.bytes[3],
                ]);
                let time_mid = u16::from_le_bytes([self.bytes[4], self.bytes[5]]);
                let time_hi = u16::from_le_bytes([self.bytes[6], self.bytes[7]]);
                let filetime: u64 =
                    (u64::from(time_hi) << 48) | (u64::from(time_mid) << 32) | u64::from(time_low);

                #[allow(clippy::cast_possible_truncation)]
                if filetime < FILETIME_EPOCH_OFFSET {
                    let unix_100ns = FILETIME_EPOCH_OFFSET - filetime;

                    Some(
                        UNIX_EPOCH
                            - Duration::new(
                                unix_100ns / 10_000_000,
                                (unix_100ns % 10_000_000) as u32 * 100,
                            ),
                    )
                } else {
                    let unix_100ns = filetime - FILETIME_EPOCH_OFFSET;

                    Some(
                        UNIX_EPOCH
                            + Duration::new(
                                unix_100ns / 10_000_000,
                                (unix_100ns % 10_000_000) as u32 * 100,
                            ),
                    )
                }
            }
            // NCS: 48-bit timestamp, 4μs units since 1980-01-01, big-endian
            (_, crate::Variant::NCS) => {
                // timestamp: bytes 0..6 (BE)
                let mut ts_bytes = [0u8; 8];
                ts_bytes[2..8].copy_from_slice(&self.bytes[0..6]);
                let ts = u64::from_be_bytes(ts_bytes);

                let ncs_epoch = UNIX_EPOCH + NCS_EPOCH;
                Some(ncs_epoch + Duration::from_micros(ts * 4))
            }
            _ => None,
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use crate::Variant;

    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn v1_and_v2_timestamp_roundtrip() {
        let t = Gregorian::epoch() + Duration::from_secs(1_000_000_000);
        let uuid = UUID::from_parts_v1(0x6fc1_0000, 0x86f2, 0x23, 0x1234, [1, 2, 3, 4, 5, 6]);
        let ts = uuid.get_timestamp().unwrap();
        assert_eq!(ts, t, "v1 timestamp roundtrip failed");
    }

    #[test]
    fn v6_timestamp_roundtrip() {
        let t = Gregorian::epoch() + Duration::from_secs(1_000_000_000);
        let uuid = UUID::from_parts_v6(0x0238_6f26, 0xfc10, 0x6000, 0x1234, [1, 2, 3, 4, 5, 6]);
        let ts = uuid.get_timestamp().unwrap();
        assert_eq!(ts, t, "v6 timestamp roundtrip failed");
    }

    #[test]
    fn v7_timestamp_roundtrip() {
        let ms = 1_700_000_000_000u64;
        let uuid = UUID::from_parts_v7(ms, 0, 0);
        let ts = uuid.get_timestamp().unwrap();
        let expected = UNIX_EPOCH + Duration::from_millis(ms);
        assert_eq!(ts, expected, "v7 timestamp roundtrip failed");
    }

    #[test]
    fn dcom_timestamp_roundtrip() {
        // FILETIME: 100ns since 1601-01-01
        let t = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
        let node = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let uuid = UUID::new_dcom(t, node).unwrap();
        let ts = uuid.get_timestamp().unwrap();
        assert_eq!(ts, t, "DCOM timestamp roundtrip failed");
    }

    #[test]
    fn ncs_timestamp_roundtrip() {
        // NCS epoch: 1980-01-01
        let ncs_epoch = UNIX_EPOCH + Duration::from_secs(315_532_800);
        let t = ncs_epoch + Duration::from_secs(1_000_000);
        let address = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let uuid = UUID::new_ncs(t, 1, &address).unwrap();
        let ts = uuid.get_timestamp().unwrap();
        assert_eq!(ts, t, "NCS timestamp roundtrip failed");
    }

    #[test]
    fn non_time_based_versions_return_none() {
        for v in [3, 4, 5, 8] {
            let mut bytes = [0u8; 16];
            bytes[6] = v << 4;
            let uuid = UUID::from_parts_v8(bytes);
            assert!(
                uuid.get_timestamp().is_none(),
                "Non-time-based v{v} should return None"
            );
        }
    }

    #[test]
    fn v1_from_bytes_and_timestamp() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x10; // version 1 (time-based)
        bytes[8] = 0x80; // RFC 4122 variant (10xx xxxx)
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_version(), Some(1));
        assert!(uuid.get_timestamp().is_some());
    }

    #[test]
    fn v2_from_bytes_and_timestamp() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x20; // version 2 (DCE security)
        bytes[8] = 0x80; // RFC 4122 variant
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_version(), Some(2));
        assert!(uuid.get_timestamp().is_some());
    }

    #[test]
    fn v3_from_bytes_and_timestamp() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x30; // version 3 (name-based MD5)
        bytes[8] = 0x80; // RFC 4122 variant
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_version(), Some(3));
        assert_eq!(uuid.get_timestamp(), None);
    }

    #[test]
    fn v4_from_bytes_and_timestamp() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x40; // version 4 (random)
        bytes[8] = 0x80; // RFC 4122 variant
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_version(), Some(4));
        assert_eq!(uuid.get_timestamp(), None);
    }

    #[test]
    fn v5_from_bytes_and_timestamp() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x50; // version 5 (name-based SHA-1)
        bytes[8] = 0x80; // RFC 4122 variant
        let uuid: UUID = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_version(), Some(5));
        assert_eq!(uuid.get_timestamp(), None);
    }

    // ---------- variant tests -------------------------------------------------

    #[test]
    fn variant_ncs() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x40; // any version; keep v4 for consistency
        bytes[8] = 0x00; // NCS variant (0xxx xxxx)
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_variant(), Variant::NCS);
    }

    #[test]
    fn variant_rfc4122() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x40; // version 4
        bytes[8] = 0x80; // RFC 4122 variant (10xx xxxx)
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_variant(), Variant::OSF);
    }

    #[test]
    fn variant_microsoft() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x40; // version 4
        bytes[8] = 0xC0; // Microsoft variant (110x xxxx)
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_variant(), Variant::DCOM);
    }

    #[test]
    fn variant_future() {
        let mut bytes = [0u8; 16];
        bytes[6] = 0x40; // version 4
        bytes[8] = 0xE0; // Future variant (111x xxxx)
        let uuid = UUID::from_bytes(bytes);
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.get_variant(), Variant::Reserved);
    }

    // UUID (and Microsoft file-time) tick = 100 ns
    const HUNDRED_NS_PER_SEC: u64 = 10_000_000;

    // Difference 1582-10-15 → 1970-01-01 in 100 ns ticks
    const UUID_UNIX_TICKS: u64 = 0x01B2_1DD2_1381_4000; // 122192928000000000

    // Difference 1970-01-01 → 1980-01-01 in whole seconds
    const SECS_1970_TO_1980: u64 = 315_532_800;

    // ---------- version 1, 2, and 6 (100 ns since Gregorian epoch) ----------

    #[test]
    fn v1_timestamp_exact_unix_epoch() {
        let ticks = UUID_UNIX_TICKS;
        let time_low = (ticks & 0xFFFF_FFFF) as u32;
        let time_mid = ((ticks >> 32) & 0xFFFF) as u16;
        let time_hi_ver = (((ticks >> 48) & 0x0FFF) as u16) | 0x1000; // ver = 1

        let mut b = [0u8; 16];
        b[0] = (time_low >> 24) as u8;
        b[1] = (time_low >> 16) as u8;
        b[2] = (time_low >> 8) as u8;
        b[3] = time_low as u8;
        b[4] = (time_mid >> 8) as u8;
        b[5] = time_mid as u8;
        b[6] = (time_hi_ver >> 8) as u8;
        b[7] = time_hi_ver as u8;
        b[8] = 0x80; // RFC-4122 variant
        let uuid = UUID::from_bytes(b);
        assert_eq!(uuid.get_timestamp(), Some(SystemTime::UNIX_EPOCH));
    }

    #[test]
    fn v2_timestamp_exact_unix_epoch() {
        let ticks = UUID_UNIX_TICKS;
        let time_low = (ticks & 0xFFFF_FFFF) as u32;
        let time_mid = ((ticks >> 32) & 0xFFFF) as u16;
        let time_hi_ver = (((ticks >> 48) & 0x0FFF) as u16) | 0x2000; // ver = 2

        let mut b = [0u8; 16];
        b[0] = (time_low >> 24) as u8;
        b[1] = (time_low >> 16) as u8;
        b[2] = (time_low >> 8) as u8;
        b[3] = time_low as u8;
        b[4] = (time_mid >> 8) as u8;
        b[5] = time_mid as u8;
        b[6] = (time_hi_ver >> 8) as u8;
        b[7] = time_hi_ver as u8;
        b[8] = 0x80;
        let uuid = UUID::from_bytes(b);
        assert_eq!(uuid.get_timestamp(), Some(SystemTime::UNIX_EPOCH));
    }

    #[test]
    fn v6_timestamp_exact_unix_epoch() {
        let ticks = UUID_UNIX_TICKS;
        let hi = (ticks >> 28) as u32;
        let mid = ((ticks >> 12) & 0xFFFF) as u16;
        let lo_ver = ((ticks & 0x0FFF) as u16) | 0x6000; // ver = 6

        let mut b = [0u8; 16];
        b[0] = (hi >> 24) as u8;
        b[1] = (hi >> 16) as u8;
        b[2] = (hi >> 8) as u8;
        b[3] = hi as u8;
        b[4] = (mid >> 8) as u8;
        b[5] = mid as u8;
        b[6] = (lo_ver >> 8) as u8;
        b[7] = lo_ver as u8;
        b[8] = 0x80;
        let uuid = UUID::from_bytes(b);
        assert_eq!(uuid.get_timestamp(), Some(SystemTime::UNIX_EPOCH));
    }

    // ------------------------- version 7 (msec since UNIX) -------------------

    #[test]
    fn v7_timestamp_zero() {
        // first 48 bits (bytes 0-5 & low nibble of byte 6) are zero
        let mut b = [0u8; 16];
        b[6] = 0x70; // set version 7
        b[8] = 0x80; // RFC-4122 variant
        let uuid = UUID::from_bytes(b);
        assert_eq!(uuid.get_timestamp(), Some(SystemTime::UNIX_EPOCH));
    }

    // ------------------------- NCS variant (4 µs since 1980) ------------------

    #[test]
    fn ncs_timestamp_epoch() {
        // ticks = 0 → 1980-01-01
        let uuid = UUID::from_bytes([0u8; 16]); // variant bit = 0
        let expected = SystemTime::UNIX_EPOCH + Duration::from_secs(SECS_1970_TO_1980);
        assert_eq!(uuid.get_timestamp(), Some(expected));
    }

    // --------------------- Microsoft / DCOM variant (file-time) --------------

    #[test]
    fn dcom_timestamp_exact_unix_epoch() {
        // 100 ns ticks from 1601-01-01 to 1970-01-01
        let ticks = 11_644_473_600u64 * HUNDRED_NS_PER_SEC; // = 116444736000000000
        let lo = (ticks & 0xFFFF_FFFF) as u32;
        let mid = ((ticks >> 32) & 0xFFFF) as u16;
        let hi = ((ticks >> 48) & 0xFFFF) as u16;

        let mut b = [0u8; 16];
        // Little-endian encoding for DCOM UUIDs
        b[0] = lo as u8;
        b[1] = (lo >> 8) as u8;
        b[2] = (lo >> 16) as u8;
        b[3] = (lo >> 24) as u8;
        b[4] = mid as u8;
        b[5] = (mid >> 8) as u8;
        b[6] = hi as u8;
        b[7] = (hi >> 8) as u8;
        b[8] = 0xC0; // Microsoft variant (110x xxxx)
        let uuid = UUID::from_bytes(b);
        assert_eq!(uuid.get_timestamp(), Some(SystemTime::UNIX_EPOCH));
    }
}
