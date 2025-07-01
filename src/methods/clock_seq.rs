use crate::UUID;

impl UUID {
    #[must_use]
    pub const fn clock_seq(&self) -> Option<u16> {
        match self.version() {
            Some(1 | 2 | 6) => Some(u16::from_be_bytes([self.bytes[8], self.bytes[9]]) & 0x3FFF),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::UUID;

    // Helper to create a UUID with a specific version and clock_seq
    const fn make_uuid_with_version_and_clock_seq(version: u8, clock_seq: u16) -> UUID {
        let mut uuid = UUID::nil();

        // Set clock_seq in bytes 8 and 9 (big-endian)
        let clock_seq_bytes = clock_seq.to_be_bytes();

        uuid.bytes[8] = clock_seq_bytes[0];
        uuid.bytes[9] = clock_seq_bytes[1];

        // set variant and version bits
        uuid.with_version(version)
    }

    #[test]
    fn test_get_clock_seq_version_1() {
        let clock_seq = 0x3ABC;
        let uuid = make_uuid_with_version_and_clock_seq(1, clock_seq);
        assert_eq!(uuid.clock_seq(), Some(clock_seq & 0x3FFF));
    }

    #[test]
    fn test_get_clock_seq_version_2() {
        let clock_seq = 0x1234;
        let uuid = make_uuid_with_version_and_clock_seq(2, clock_seq);
        assert_eq!(uuid.clock_seq(), Some(clock_seq & 0x3FFF));
    }

    #[test]
    fn test_get_clock_seq_version_6() {
        let clock_seq = 0x3FFF;
        let uuid = make_uuid_with_version_and_clock_seq(6, clock_seq);
        assert_eq!(uuid.clock_seq(), Some(clock_seq & 0x3FFF));
    }

    #[test]
    fn test_get_clock_seq_other_version() {
        let clock_seq = 0x3ABC;
        let uuid = make_uuid_with_version_and_clock_seq(4, clock_seq);
        assert_eq!(uuid.clock_seq(), None);
    }

    #[test]
    fn test_get_clock_seq_invalid_version() {
        // UUID with version() returning None
        let uuid = UUID::nil();

        assert_eq!(uuid.version(), None);
        assert_eq!(uuid.clock_seq(), None);
    }
}
