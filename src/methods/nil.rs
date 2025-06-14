use crate::{UUID, UUID_BYTES};

impl UUID {
    #[must_use]
    pub const fn nil() -> Self {
        Self {
            bytes: [0; UUID_BYTES],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `nil()` function correctly produces a UUID with all
    /// zero bytes.
    #[test]
    fn test_nil_is_all_zeros() {
        let nil_uuid = UUID::nil();
        let expected_bytes = [0u8; UUID_BYTES];
        assert_eq!(
            nil_uuid.bytes, expected_bytes,
            "The bytes of a nil UUID should all be zero"
        );
    }

    /// Tests that the `nil()` function is deterministic and that two generated
    /// nil UUIDs are equal to each other.
    #[test]
    fn test_nil_is_deterministic() {
        let nil1 = UUID::nil();
        let nil2 = UUID::nil();
        assert_eq!(nil1, nil2, "Two nil UUIDs should be equal");
    }

    /// This test verifies the `const` nature of the `nil()` function.
    /// The test logic itself is trivial; the key is that the code *compiles*.
    /// By successfully assigning the result of `UUID::nil()` to a `const`
    /// item, we prove that it can be evaluated at compile time.
    #[test]
    fn test_nil_can_be_used_in_const_context() {
        const COMPILE_TIME_NIL: UUID = UUID::nil();
        const EXPECTED_NIL: UUID = UUID {
            bytes: [0; UUID_BYTES],
        };

        // This assertion is somewhat redundant if the other tests pass,
        // but it completes the test case.
        assert_eq!(
            COMPILE_TIME_NIL, EXPECTED_NIL,
            "A const-evaluated nil UUID should match the expected value"
        );

        // We can also use it directly in a match arm or other const places.
        match UUID::nil() {
            COMPILE_TIME_NIL => (), // This arm must match
            _ => panic!("A const nil UUID should match UUID::nil()"),
        }
    }
}
