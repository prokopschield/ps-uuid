use crate::UUID;

impl UUID {
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::UUID;

    #[test]
    fn identity() {
        let uuid = UUID::max();

        assert_eq!(&uuid.bytes, uuid.as_bytes(), "Bytes should be identical.");

        assert_eq!(
            uuid.bytes.as_ptr(),
            uuid.as_bytes().as_ptr(),
            "Pointers should be identical."
        );
    }
}
