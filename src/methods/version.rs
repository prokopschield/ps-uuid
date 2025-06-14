use crate::{Variant, UUID};

impl UUID {
    #[must_use]
    pub fn version(&self) -> Option<u8> {
        if self.variant() != Variant::OSF {
            return None;
        }

        Some(self.bytes[6] >> 4)
    }
}

#[cfg(test)]
mod tests {
    use super::UUID;

    #[test]
    fn test_version_osf() {
        // Test for Version 0 (OSF variant)
        let uuid_v0 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v0.version(), Some(0));

        // Test for Version 1 (OSF variant)
        let uuid_v1 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x10, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v1.version(), Some(1));

        // Test for Version 2 (OSF variant)
        let uuid_v2 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x20, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v2.version(), Some(2));

        // Test for Version 3 (OSF variant)
        let uuid_v3 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x30, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v3.version(), Some(3));

        // Test for Version 4 (OSF variant)
        let uuid_v4 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x40, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v4.version(), Some(4));

        // Test for Version 5 (OSF variant)
        let uuid_v5 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x50, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v5.version(), Some(5));

        // Test for Version 6 (OSF variant)
        let uuid_v6 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x60, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v6.version(), Some(6));

        // Test for Version 7 (OSF variant)
        let uuid_v7 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x70, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v7.version(), Some(7));

        // Test for Version 8 (OSF variant)
        let uuid_v8 = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x80, 0x80, 0x86, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_v8.version(), Some(8));
    }

    #[test]
    fn test_version_non_osf() {
        // Test for non-OSF variant (e.g., NCS variant, byte 8 set to 0x00)
        let uuid_non_osf = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x40, 0x06, 0x00, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_non_osf.version(), None);

        // Additional test for another non-OSF variant (e.g., Microsoft variant)
        let uuid_non_osf_ms = UUID {
            bytes: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x40, 0x06, 0x60, 0x07, 0x08, 0x09, 0x0A, 0x0B,
                0x0C, 0x0D,
            ],
        };
        assert_eq!(uuid_non_osf_ms.version(), None);
    }
}
