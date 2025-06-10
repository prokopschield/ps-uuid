use crate::UUID;

impl UUID {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }
}

#[cfg(test)]
mod tests {
    use crate::{UUID, UUID_BYTES};

    #[test]
    fn identity() {
        let bytes = [42u8; UUID_BYTES];
        let uuid = UUID::from_bytes(bytes);

        assert_eq!(bytes, uuid.bytes, "Arrays should be identical.");
    }
}
