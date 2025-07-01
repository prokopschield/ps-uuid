use crate::{UuidConstructionError, UUID};

impl UUID {
    /// Generate v2 UUID (DCE Security)
    ///
    /// # Errors
    /// - `TimestampBeforeEpoch` is returned if the current time predates 1582-10-15.
    /// - `TimestampOverflow` is returned if the current time exceeds 5236-03-31.
    pub fn gen_v2(domain: u8, local_id: u32) -> Result<Self, UuidConstructionError> {
        let mut uuid = Self::gen_v1()?;

        // Replace time_low with local_id (in big-endian order)
        uuid.bytes[0..4].copy_from_slice(&local_id.to_be_bytes());
        uuid.bytes[9] = domain;

        Ok(uuid.with_version(2))
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------

    /// Convenience wrapper that panics on construction failure.
    fn v2(domain: u8, local_id: u32) -> UUID {
        UUID::gen_v2(domain, local_id).expect("failed to build v2 UUID")
    }

    /// Extract the 32-bit value stored in the first four bytes
    /// (\( \text{time\_low} \) / Local-ID field).
    fn local_id(u: &UUID) -> u32 {
        u32::from_be_bytes(u.bytes[0..4].try_into().unwrap())
    }

    /// True iff the RFC-4122 variant bits are `10`.
    const fn is_rfc4122_variant(u: &UUID) -> bool {
        (u.bytes[8] & 0b1100_0000) == 0b1000_0000
    }

    // ---------------------------------------------------------------------
    // Tests
    // ---------------------------------------------------------------------

    #[test]
    fn version_is_always_2() {
        for &domain in &[0, 1, 2, 42] {
            let u = v2(domain, 0xDEAD_BEEF);
            assert_eq!(u.get_version(), Some(2));
        }
    }

    #[test]
    fn variant_bits_remain_rfc4122() {
        let u = v2(1, 123);
        assert!(is_rfc4122_variant(&u));
    }

    #[test]
    fn local_id_is_encoded_big_endian() {
        for &id in &[0, 1, 0x1234_5678, u32::MAX] {
            let u = v2(2, id);
            assert_eq!(local_id(&u), id);
        }
    }

    #[test]
    fn domain_is_written_to_clock_seq_low() {
        for domain in 0u8..=10 {
            let u = v2(domain, 7);
            assert_eq!(u.bytes[9], domain);
        }
    }

    #[test]
    fn different_local_ids_produce_distinct_uuids() {
        let u1 = v2(1, 0xAAAA_BBBB);
        let u2 = v2(1, 0xCCCC_DDDD);
        assert_ne!(
            u1.bytes, u2.bytes,
            "UUIDs must differ when the Local-ID field changes"
        );
    }

    #[test]
    fn different_domains_produce_distinct_uuids() {
        let u1 = v2(0, 42);
        let u2 = v2(7, 42);
        assert_ne!(
            u1.bytes, u2.bytes,
            "UUIDs must differ when the Domain field changes"
        );
    }
}
