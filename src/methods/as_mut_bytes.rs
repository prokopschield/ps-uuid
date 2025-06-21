use crate::UUID;

impl UUID {
    #[must_use]
    pub fn as_mut_bytes(&mut self) -> &mut [u8; 16] {
        &mut self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::UUID;

    #[test]
    fn identity() {
        let mut uuid = UUID::max();

        uuid.as_mut_bytes()[12] = 3;

        assert_eq!(uuid.bytes[12], 3, "Byte should be modified.");
    }
}
