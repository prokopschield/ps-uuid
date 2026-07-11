use crate::{Variant, UUID};

impl UUID {
    /// Returns the clock sequence, or `None` if the UUID does not carry one.
    ///
    /// Versions 1 and 6 carry a 14-bit clock sequence. A version-2 UUID
    /// overwrites `clock_seq_low` with the local domain, so only the six high
    /// bits survive; they are returned at their original positions (bits 8..14),
    /// with the low byte cleared. UUIDs of the DCOM variant carry a 13-bit clock
    /// sequence, as the variant occupies the top three bits. This decoding is
    /// this crate's own (see [`UUID::new_dcom`]); for Microsoft GUIDs from
    /// other sources, such as COM interface identifiers, the returned value is
    /// meaningless.
    #[must_use]
    pub const fn get_clock_seq(&self) -> Option<u16> {
        match (self.get_version(), self.get_variant()) {
            (Some(2), _) => Some(u16::from_be_bytes([self.bytes[8], self.bytes[9]]) & 0x3F00),
            (Some(1 | 6), _) => Some(u16::from_be_bytes([self.bytes[8], self.bytes[9]]) & 0x3FFF),
            (_, Variant::DCOM) => Some(u16::from_be_bytes([self.bytes[8], self.bytes[9]]) & 0x1FFF),
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
        assert_eq!(uuid.get_clock_seq(), Some(clock_seq & 0x3FFF));
    }

    #[test]
    fn test_get_clock_seq_version_2() {
        // v2 overwrites clock_seq_low with the domain, so only the six high bits
        // survive; the low byte is masked away.
        let clock_seq = 0x1234;
        let uuid = make_uuid_with_version_and_clock_seq(2, clock_seq);
        assert_eq!(uuid.get_clock_seq(), Some(clock_seq & 0x3F00));
    }

    #[test]
    fn test_get_clock_seq_version_2_ignores_domain_byte() {
        // A genuine v2 UUID stores the domain in byte 9; it must not leak into
        // the returned clock sequence.
        let mut uuid = make_uuid_with_version_and_clock_seq(2, 0x1234);
        uuid.bytes[9] = 0xFF; // domain byte

        assert_eq!(uuid.get_clock_seq(), Some(0x1200));
    }

    #[test]
    fn test_get_clock_seq_version_6() {
        let clock_seq = 0x3FFF;
        let uuid = make_uuid_with_version_and_clock_seq(6, clock_seq);
        assert_eq!(uuid.get_clock_seq(), Some(clock_seq & 0x3FFF));
    }

    #[test]
    fn test_get_clock_seq_other_version() {
        let clock_seq = 0x3ABC;
        let uuid = make_uuid_with_version_and_clock_seq(4, clock_seq);
        assert_eq!(uuid.get_clock_seq(), None);
    }

    #[test]
    fn test_get_clock_seq_invalid_version() {
        // UUID with version() returning None
        let uuid = UUID::nil();

        assert_eq!(uuid.get_version(), None);
        assert_eq!(uuid.get_clock_seq(), None);
    }

    #[test]
    fn test_get_clock_seq_dcom() {
        let uuid = UUID::from_parts_dcom(0, 0, 0, 0x1ABC, [0; 6]);

        assert_eq!(uuid.get_clock_seq(), Some(0x1ABC));
    }

    #[test]
    fn test_get_clock_seq_dcom_masks_variant_bits() {
        // The variant overwrites the top three bits, leaving 13 bits.
        let uuid = UUID::from_parts_dcom(0, 0, 0, 0xFFFF, [0; 6]);

        assert_eq!(uuid.get_clock_seq(), Some(0x1FFF));
    }
}
