const LUT: &[u8; 16] = b"0123456789abcdef";

#[must_use]
pub fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);

    for &b in bytes {
        out.push(LUT[(b >> 4) as usize] as char);
        out.push(LUT[(b & 0x0f) as usize] as char);
    }

    out
}

pub trait ToHex
where
    Self: Sized,
{
    fn to_hex(self) -> String;
}

impl<T> ToHex for T
where
    T: AsRef<[u8]>,
{
    fn to_hex(self) -> String {
        to_hex(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::{to_hex, ToHex};

    #[test]
    fn empty_slice() {
        let bytes: [u8; 0] = [];
        assert_eq!(to_hex(&bytes), ""); // Empty input should yield empty string
    }

    #[test]
    fn single_byte_zero() {
        assert_eq!(to_hex(&[0]), "00"); // Zero byte
    }

    #[test]
    fn single_byte_max() {
        assert_eq!(to_hex(&[255]), "ff"); // Maximum byte value
    }

    #[test]
    fn single_byte_mid_range() {
        assert_eq!(to_hex(&[42]), "2a"); // Mid-range byte (42 in decimal is 2A in hex)
    }

    #[test]
    fn multiple_bytes() {
        let bytes = [0x00, 0x01, 0x0A, 0xFF];
        assert_eq!(to_hex(&bytes), "00010aff"); // Should concatenate hex values
    }

    #[test]
    fn all_zeros() {
        let bytes = [0u8; 10]; // Array of ten zeros
        assert_eq!(to_hex(&bytes), "00000000000000000000"); // Ten zeros in hex
    }

    #[test]
    fn all_ones() {
        let bytes = [0xFFu8; 5]; // Array of five 0xFF bytes
        assert_eq!(to_hex(&bytes), "ffffffffff"); // Five 0xFF in hex (10 chars total)
    }

    #[test]
    fn mixed_bytes() {
        let bytes = [0x1A, 0x2B, 0x3C, 0x4D, 0x5E];
        assert_eq!(to_hex(&bytes), "1a2b3c4d5e"); // Mixed values, all lowercase
    }

    #[test]
    fn large_input() {
        let bytes: Vec<u8> = vec![0xAA; 100]; // 100 bytes of 0xAA
        let expected: String = "aa".repeat(100); // 100 * "aa" = 200 characters
        assert_eq!(to_hex(&bytes), expected); // Ensure it handles larger slices
    }

    #[test]
    fn ensures_lowercase() {
        let bytes = [0xAB, 0xCD];
        assert_eq!(to_hex(&bytes), "abcd"); // Should be lowercase, not "ABCD"
                                            // Note: The function uses a lookup table for lowercase letters
    }

    #[test]
    fn test_trait() {
        let bytes = b"\xde\xad\xbe\xef";
        let hex = "deadbeef";

        assert_eq!(hex, bytes.to_hex());
    }
}
