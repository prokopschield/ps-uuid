use std::time::SystemTime;

use crate::UUID;

use super::NcsUuidError;

impl UUID {
    /// Generates a new NCS UUID (Variant 0).
    ///
    /// # Errors
    ///
    /// This method returns [`NcsUuidError::TimestampOverflow`] after 2015.
    pub fn gen_ncs(address_family: u8, address: &[u8; 7]) -> Result<Self, NcsUuidError> {
        Self::new_ncs(SystemTime::now(), address_family, address)
    }
}

#[cfg(test)]
mod tests {
    use crate::{NcsUuidError, UUID};

    #[test]
    fn overflow() {
        assert!(matches!(
            UUID::gen_ncs(5, &[5, 4, 8, 6, 0, 0, 0]),
            Err(NcsUuidError::TimestampOverflow)
        ));
    }
}
