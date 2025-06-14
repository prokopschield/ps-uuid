use crate::{UUID, UUID_BYTES};

impl UUID {
    #[must_use]
    pub const fn max() -> Self {
        Self {
            bytes: [0xFF; UUID_BYTES],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{UUID, UUID_BYTES};

    /// Tests that the `max()` function correctly produces a UUID with all
    /// bytes set to `0xFF`.
    #[test]
    fn test_max_is_all_ones() {
        let max_uuid = UUID::max();
        let expected_bytes = [0xFFu8; UUID_BYTES];
        assert_eq!(
            max_uuid.bytes, expected_bytes,
            "The bytes of a max UUID should all be 0xFF"
        );
    }

    /// Tests that the `max()` function is deterministic and that two generated
    /// max UUIDs are equal to each other.
    #[test]
    fn test_max_is_deterministic() {
        let max1 = UUID::max();
        let max2 = UUID::max();
        assert_eq!(max1, max2, "Two max UUIDs should be equal");
    }

    /// Verifies the `const` nature of the `max()` function by using it to
    /// initialize a `const` item. The successful compilation of this test
    /// is the primary validation.
    #[test]
    fn test_max_can_be_used_in_const_context() {
        const COMPILE_TIME_MAX: UUID = UUID::max();
        const EXPECTED_MAX: UUID = UUID {
            bytes: [0xFF; UUID_BYTES],
        };
        assert_eq!(
            COMPILE_TIME_MAX, EXPECTED_MAX,
            "A const-evaluated max UUID should match the expected value"
        );
        match UUID::max() {
            COMPILE_TIME_MAX => (),
            _ => panic!("A const max UUID should match UUID::max()"),
        }
    }
}
