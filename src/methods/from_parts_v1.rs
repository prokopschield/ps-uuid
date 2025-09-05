use crate::UUID;

impl UUID {
    /// Build a RFC 4122 version-1 (time-based) UUID from its
    /// individual wire-format fields.
    ///
    /// Arguments must already be laid out exactly as described in
    /// RFC 4122 §4.1 (big-endian/network order):
    ///
    /// - `time_low`:         32 least-significant bits of the 60-bit timestamp
    /// - `time_mid`:         next 16 bits of the timestamp
    /// - `time_hi`:          next 12 bits of the timestamp (upper 4 bits will be overwritten with the version by `with_version`)
    /// - `clock_seq`:        14-bit clock sequence; the two variant control bits are set by `with_version`
    /// - `node_id`:          48-bit IEEE 802 MAC address (or random host id)
    ///
    /// The function never fails; all bit fiddling is guaranteed to be
    /// valid by construction. The returned value satisfies
    /// `uuid.version() == Version::Time` and
    /// `uuid.variant() == Variant::OSF`.
    #[inline]
    #[must_use]
    pub fn from_parts_v1(
        time_low: u32,
        time_mid: u16,
        time_hi: u16,
        clock_seq: u16,
        node_id: [u8; 6],
    ) -> Self {
        let mut uuid = Self::nil();

        // Timestamp ---------------------------------------------------------
        uuid.bytes[0..4].copy_from_slice(&time_low.to_be_bytes());
        uuid.bytes[4..6].copy_from_slice(&time_mid.to_be_bytes());
        uuid.bytes[6..8].copy_from_slice(&time_hi.to_be_bytes());

        // Clock sequence ----------------------------------------------------
        uuid.bytes[8..10].copy_from_slice(&clock_seq.to_be_bytes());

        // Node id (48 bits) -------------------------------------------------
        uuid.bytes[10..16].copy_from_slice(&node_id);

        // Insert version + variant bits -------------------------------------
        uuid.with_version(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Variant, UUID};

    #[test]
    fn builds_correct_static_example() {
        // Timestamp: 0x0123456789abcdef
        // Clock-seq: 0x1234
        // Node id : 00-01-02-03-04-05
        let uuid = UUID::from_parts_v1(
            0x0123_4567,
            0x89ab,
            0xcdef,
            0x1234,
            [0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
        );
        let b = uuid.as_bytes();

        // Raw fields -----------------------------------------------------------
        assert_eq!(&b[0..4], &[0x01, 0x23, 0x45, 0x67]); // time_low
        assert_eq!(&b[4..6], &[0x89, 0xab]); // time_mid

        // time_hi: original 0xcd ef => after version patch => 0x1d ef
        assert_eq!(b[6] & 0x0f, 0x0d);
        assert_eq!(b[6] >> 4, 0x1); // version = 1
        assert_eq!(b[7], 0xef);

        // clock_seq: original 0x12 34 => variant patch => 0x92 34
        assert_eq!(b[9], 0x34);
        assert_eq!(b[8] & 0x3f, 0x12); // low 6 bits stay
        assert_eq!(b[8] >> 6, 0b10); // variant OSF

        // node id
        assert_eq!(&b[10..16], &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);

        // High-level helpers ---------------------------------------------------
        assert_eq!(uuid.get_version(), Some(1));
        assert_eq!(uuid.get_variant(), Variant::OSF);
    }

    #[test]
    fn nil_timestamp_yields_valid_uuid() {
        let uuid = UUID::from_parts_v1(0, 0, 0, 0, [0; 6]);
        assert_eq!(uuid.get_version(), Some(1));
        assert_eq!(uuid.get_variant(), Variant::OSF);
        // Only version & variant bits should be non-zero.
        let mut except = [0u8; 16];
        except[6] = 0x10; // version 1 nibble
        except[8] = 0x80; // variant 10xx_xxxx
        assert_eq!(uuid.as_bytes(), &except);
    }

    #[test]
    fn byte_order_is_big_endian() {
        let uuid = UUID::from_parts_v1(1, 1, 1, 1, [0; 6]);
        let b = uuid.as_bytes();
        // Each field must appear in network order.
        eprintln!("{uuid:}");
        assert_eq!(&b[0..4], &[0x00, 0x00, 0x00, 0x01]);
        assert_eq!(&b[4..6], &[0x00, 0x01]);
        assert_eq!(b[6], 0x01 << 4);
        assert_eq!(b[7], 0x01);
        assert_eq!(b[9], 0x01);
    }
}
