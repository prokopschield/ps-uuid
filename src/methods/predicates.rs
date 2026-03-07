//! Predicate methods for UUID type checking.

use crate::{Variant, UUID};

impl UUID {
    /// Returns `true` if this is the nil UUID (all zeros).
    #[inline]
    #[must_use]
    pub const fn is_nil(&self) -> bool {
        matches!(self.bytes, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    /// Returns `true` if this is the max UUID (all ones).
    #[inline]
    #[must_use]
    pub const fn is_max(&self) -> bool {
        matches!(
            self.bytes,
            [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ]
        )
    }

    /// Returns `true` if this is an NCS variant UUID.
    #[inline]
    #[must_use]
    pub const fn is_ncs(&self) -> bool {
        matches!(self.get_variant(), Variant::NCS)
    }

    /// Returns `true` if this is an OSF/RFC 4122 variant UUID.
    #[inline]
    #[must_use]
    pub const fn is_osf(&self) -> bool {
        matches!(self.get_variant(), Variant::OSF)
    }

    /// Returns `true` if this is a Microsoft DCOM variant UUID.
    #[inline]
    #[must_use]
    pub const fn is_dcom(&self) -> bool {
        matches!(self.get_variant(), Variant::DCOM)
    }

    /// Returns `true` if this is a reserved variant UUID.
    #[inline]
    #[must_use]
    pub const fn is_reserved(&self) -> bool {
        matches!(self.get_variant(), Variant::Reserved)
    }

    /// Returns `true` if this is a version 1 (time-based) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v1(&self) -> bool {
        matches!(self.get_version(), Some(1))
    }

    /// Returns `true` if this is a version 2 (DCE Security) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v2(&self) -> bool {
        matches!(self.get_version(), Some(2))
    }

    /// Returns `true` if this is a version 3 (MD5 hash) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v3(&self) -> bool {
        matches!(self.get_version(), Some(3))
    }

    /// Returns `true` if this is a version 4 (random) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v4(&self) -> bool {
        matches!(self.get_version(), Some(4))
    }

    /// Returns `true` if this is a version 5 (SHA-1 hash) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v5(&self) -> bool {
        matches!(self.get_version(), Some(5))
    }

    /// Returns `true` if this is a version 6 (reordered time-based) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v6(&self) -> bool {
        matches!(self.get_version(), Some(6))
    }

    /// Returns `true` if this is a version 7 (Unix epoch time-based) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v7(&self) -> bool {
        matches!(self.get_version(), Some(7))
    }

    /// Returns `true` if this is a version 8 (custom) UUID.
    #[inline]
    #[must_use]
    pub const fn is_v8(&self) -> bool {
        matches!(self.get_version(), Some(8))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Variant, UUID};

    const fn uuid_with_variant_version(variant_byte: u8, version_nibble: u8) -> UUID {
        let mut bytes = [0u8; 16];
        bytes[6] = version_nibble << 4;
        bytes[8] = variant_byte;
        UUID { bytes }
    }

    #[test]
    fn is_nil_returns_true_only_for_nil() {
        assert!(UUID::nil().is_nil());
        assert!(!UUID::max().is_nil());
        assert!(!UUID::from(1u8).is_nil());
    }

    #[test]
    fn is_max_returns_true_only_for_max() {
        assert!(UUID::max().is_max());
        assert!(!UUID::nil().is_max());
        assert!(!UUID::from(1u8).is_max());
    }

    #[test]
    fn variant_predicates_match_get_variant() {
        let ncs = uuid_with_variant_version(0x00, 0);
        let osf = uuid_with_variant_version(0x80, 4);
        let dcom = uuid_with_variant_version(0xC0, 0);
        let reserved = uuid_with_variant_version(0xE0, 0);

        assert!(ncs.is_ncs());
        assert!(!ncs.is_osf());
        assert_eq!(ncs.get_variant(), Variant::NCS);

        assert!(osf.is_osf());
        assert!(!osf.is_ncs());
        assert_eq!(osf.get_variant(), Variant::OSF);

        assert!(dcom.is_dcom());
        assert!(!dcom.is_osf());
        assert_eq!(dcom.get_variant(), Variant::DCOM);

        assert!(reserved.is_reserved());
        assert!(!reserved.is_osf());
        assert_eq!(reserved.get_variant(), Variant::Reserved);
    }

    #[test]
    fn version_predicates_match_get_version() {
        for version in 1..=8 {
            let uuid = uuid_with_variant_version(0x80, version);
            assert_eq!(uuid.get_version(), Some(version));

            assert_eq!(uuid.is_v1(), version == 1);
            assert_eq!(uuid.is_v2(), version == 2);
            assert_eq!(uuid.is_v3(), version == 3);
            assert_eq!(uuid.is_v4(), version == 4);
            assert_eq!(uuid.is_v5(), version == 5);
            assert_eq!(uuid.is_v6(), version == 6);
            assert_eq!(uuid.is_v7(), version == 7);
            assert_eq!(uuid.is_v8(), version == 8);
        }
    }

    #[test]
    fn version_predicates_return_false_for_non_osf() {
        let ncs = uuid_with_variant_version(0x00, 4);
        assert!(!ncs.is_v4());
        assert_eq!(ncs.get_version(), None);
    }

    #[test]
    fn generated_uuids_have_correct_predicates() {
        let v4 = UUID::gen_v4();
        assert!(v4.is_v4());
        assert!(v4.is_osf());
        assert!(!v4.is_nil());
        assert!(!v4.is_max());
    }
}
