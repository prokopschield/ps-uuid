use std::fmt;

use crate::UUID;

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This format is standard for all UUID variants.
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3],
            self.bytes[4], self.bytes[5],
            self.bytes[6], self.bytes[7],
            self.bytes[8], self.bytes[9],
            self.bytes[10], self.bytes[11], self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::UUID;
    use std::string::ToString;

    #[test]
    fn test_uuid_display() {
        // Test case 1: Nil UUID (all zeros)
        let nil_uuid = UUID { bytes: [0u8; 16] };
        let expected_nil = "00000000-0000-0000-0000-000000000000";
        assert_eq!(nil_uuid.to_string(), expected_nil);

        // Test case 2: Sample UUID with various values
        let sample_uuid = UUID {
            bytes: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                0x0F, 0x10,
            ],
        };
        let expected_sample = "01020304-0506-0708-090a-0b0c0d0e0f10";
        assert_eq!(sample_uuid.to_string(), expected_sample);
    }
}
