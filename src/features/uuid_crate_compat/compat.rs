use std::time::SystemTime;

use crate::{Braced, Hyphenated, NodeId, Simple, Urn, Variant, UUID};

/// Extension trait that adds all [`UUID`] methods to external UUID types.
pub trait UuidCompat {
    /// Converts to [`UUID`].
    fn to_uuid(&self) -> UUID;

    fn get_variant(&self) -> Variant {
        self.to_uuid().get_variant()
    }

    fn get_version(&self) -> Option<u8> {
        self.to_uuid().get_version()
    }

    fn get_timestamp(&self) -> Option<SystemTime> {
        self.to_uuid().get_timestamp()
    }

    fn get_node_id(&self) -> Option<NodeId> {
        self.to_uuid().get_node_id()
    }

    fn get_clock_seq(&self) -> Option<u16> {
        self.to_uuid().get_clock_seq()
    }

    fn to_u128(&self) -> u128 {
        self.to_uuid().to_u128()
    }

    fn is_nil(&self) -> bool {
        self.to_uuid().is_nil()
    }

    fn is_max(&self) -> bool {
        self.to_uuid().is_max()
    }

    fn is_ncs(&self) -> bool {
        self.to_uuid().is_ncs()
    }

    fn is_osf(&self) -> bool {
        self.to_uuid().is_osf()
    }

    fn is_dcom(&self) -> bool {
        self.to_uuid().is_dcom()
    }

    fn is_reserved(&self) -> bool {
        self.to_uuid().is_reserved()
    }

    fn is_v1(&self) -> bool {
        self.to_uuid().is_v1()
    }

    fn is_v2(&self) -> bool {
        self.to_uuid().is_v2()
    }

    fn is_v3(&self) -> bool {
        self.to_uuid().is_v3()
    }

    fn is_v4(&self) -> bool {
        self.to_uuid().is_v4()
    }

    fn is_v5(&self) -> bool {
        self.to_uuid().is_v5()
    }

    fn is_v6(&self) -> bool {
        self.to_uuid().is_v6()
    }

    fn is_v7(&self) -> bool {
        self.to_uuid().is_v7()
    }

    fn is_v8(&self) -> bool {
        self.to_uuid().is_v8()
    }

    fn simple(&self) -> Simple {
        self.to_uuid().simple()
    }

    fn hyphenated(&self) -> Hyphenated {
        self.to_uuid().hyphenated()
    }

    fn urn(&self) -> Urn {
        self.to_uuid().urn()
    }

    fn braced(&self) -> Braced {
        self.to_uuid().braced()
    }
}

impl UuidCompat for uuid::Uuid {
    fn to_uuid(&self) -> UUID {
        UUID::from(self)
    }
}

impl UuidCompat for UUID {
    fn to_uuid(&self) -> UUID {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_methods() {
        let internal = UUID::gen_v4();
        let external: uuid::Uuid = internal.into();

        // Conversion
        assert_eq!(external.to_uuid(), internal);

        // Getters
        assert_eq!(UuidCompat::get_variant(&external), internal.get_variant());
        assert_eq!(UuidCompat::get_version(&external), internal.get_version());
        assert_eq!(
            UuidCompat::get_timestamp(&external),
            internal.get_timestamp()
        );
        assert_eq!(UuidCompat::get_node_id(&external), internal.get_node_id());
        assert_eq!(
            UuidCompat::get_clock_seq(&external),
            internal.get_clock_seq()
        );
        assert_eq!(UuidCompat::to_u128(&external), internal.to_u128());

        // Predicates
        assert_eq!(UuidCompat::is_nil(&external), internal.is_nil());
        assert_eq!(UuidCompat::is_max(&external), internal.is_max());
        assert_eq!(UuidCompat::is_ncs(&external), internal.is_ncs());
        assert_eq!(UuidCompat::is_osf(&external), internal.is_osf());
        assert_eq!(UuidCompat::is_dcom(&external), internal.is_dcom());
        assert_eq!(UuidCompat::is_reserved(&external), internal.is_reserved());
        assert_eq!(UuidCompat::is_v1(&external), internal.is_v1());
        assert_eq!(UuidCompat::is_v2(&external), internal.is_v2());
        assert_eq!(UuidCompat::is_v3(&external), internal.is_v3());
        assert_eq!(UuidCompat::is_v4(&external), internal.is_v4());
        assert_eq!(UuidCompat::is_v5(&external), internal.is_v5());
        assert_eq!(UuidCompat::is_v6(&external), internal.is_v6());
        assert_eq!(UuidCompat::is_v7(&external), internal.is_v7());
        assert_eq!(UuidCompat::is_v8(&external), internal.is_v8());

        // Formatters
        assert_eq!(
            UuidCompat::simple(&external).to_string(),
            internal.simple().to_string()
        );
        assert_eq!(
            UuidCompat::hyphenated(&external).to_string(),
            internal.hyphenated().to_string()
        );
        assert_eq!(
            UuidCompat::urn(&external).to_string(),
            internal.urn().to_string()
        );
        assert_eq!(
            UuidCompat::braced(&external).to_string(),
            internal.braced().to_string()
        );
    }
}
