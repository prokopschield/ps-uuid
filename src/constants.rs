//! Well-known UUID constants.

use crate::{uuid, UUID};

impl UUID {
    // =========================================================================
    // Fundamental
    // =========================================================================

    /// The nil UUID, where all bits are set to zero.
    ///
    /// ```text
    /// 00000000-0000-0000-0000-000000000000
    /// ```
    pub const NIL: Self = Self { bytes: [0; 16] };

    /// The max UUID, where all bits are set to one.
    ///
    /// ```text
    /// ffffffff-ffff-ffff-ffff-ffffffffffff
    /// ```
    pub const MAX: Self = Self { bytes: [0xFF; 16] };

    // =========================================================================
    // RFC 4122 Namespaces
    // =========================================================================

    /// The DNS namespace UUID, for generating v3/v5 UUIDs from domain names.
    ///
    /// Defined in RFC 4122 Appendix C.
    ///
    /// ```text
    /// 6ba7b810-9dad-11d1-80b4-00c04fd430c8
    /// ```
    pub const NS_DNS: Self = uuid!("6ba7b810-9dad-11d1-80b4-00c04fd430c8");

    /// The URL namespace UUID, for generating v3/v5 UUIDs from URLs.
    ///
    /// Defined in RFC 4122 Appendix C.
    ///
    /// ```text
    /// 6ba7b811-9dad-11d1-80b4-00c04fd430c8
    /// ```
    pub const NS_URL: Self = uuid!("6ba7b811-9dad-11d1-80b4-00c04fd430c8");

    /// The OID namespace UUID, for generating v3/v5 UUIDs from ISO OIDs.
    ///
    /// Defined in RFC 4122 Appendix C.
    ///
    /// ```text
    /// 6ba7b812-9dad-11d1-80b4-00c04fd430c8
    /// ```
    pub const NS_OID: Self = uuid!("6ba7b812-9dad-11d1-80b4-00c04fd430c8");

    /// The X.500 namespace UUID, for generating v3/v5 UUIDs from X.500 DNs.
    ///
    /// Defined in RFC 4122 Appendix C.
    ///
    /// ```text
    /// 6ba7b814-9dad-11d1-80b4-00c04fd430c8
    /// ```
    pub const NS_X500: Self = uuid!("6ba7b814-9dad-11d1-80b4-00c04fd430c8");

    // =========================================================================
    // Bluetooth
    // =========================================================================

    /// The Bluetooth Base UUID.
    ///
    /// Used to construct full 128-bit UUIDs from 16-bit and 32-bit Bluetooth
    /// identifiers. To convert a 16-bit UUID `xxxx`: `0000xxxx-0000-1000-8000-00805f9b34fb`.
    ///
    /// Defined in the Bluetooth Core Specification.
    ///
    /// ```text
    /// 00000000-0000-1000-8000-00805f9b34fb
    /// ```
    pub const BLUETOOTH_BASE: Self = uuid!("00000000-0000-1000-8000-00805f9b34fb");

    // =========================================================================
    // GPT Partition Types
    // =========================================================================

    /// EFI System Partition type GUID.
    ///
    /// The FAT-formatted partition containing UEFI bootloaders and drivers.
    ///
    /// ```text
    /// c12a7328-f81f-11d2-ba4b-00a0c93ec93b
    /// ```
    pub const GPT_EFI_SYSTEM: Self = uuid!("c12a7328-f81f-11d2-ba4b-00a0c93ec93b");

    /// Linux filesystem data partition type GUID.
    ///
    /// The generic type for Linux filesystem partitions (ext4, xfs, btrfs, etc.).
    ///
    /// ```text
    /// 0fc63daf-8483-4772-8e79-3d69d8477de4
    /// ```
    pub const GPT_LINUX_FS: Self = uuid!("0fc63daf-8483-4772-8e79-3d69d8477de4");

    /// Linux swap partition type GUID.
    ///
    /// ```text
    /// 0657fd6d-a4ab-43c4-84e5-0933c84b4f4f
    /// ```
    pub const GPT_LINUX_SWAP: Self = uuid!("0657fd6d-a4ab-43c4-84e5-0933c84b4f4f");

    /// Linux root partition (x86-64) type GUID.
    ///
    /// Used by systemd-gpt-auto-generator for automatic root discovery.
    ///
    /// ```text
    /// 4f68bce3-e8cd-4db1-96e7-fbcaf984b709
    /// ```
    pub const GPT_LINUX_ROOT_X86_64: Self = uuid!("4f68bce3-e8cd-4db1-96e7-fbcaf984b709");

    /// Linux home partition type GUID.
    ///
    /// Used by systemd-gpt-auto-generator for automatic /home discovery.
    ///
    /// ```text
    /// 933ac7e1-2eb4-4f13-b844-0e14e2aef915
    /// ```
    pub const GPT_LINUX_HOME: Self = uuid!("933ac7e1-2eb4-4f13-b844-0e14e2aef915");

    /// Microsoft Basic Data partition type GUID.
    ///
    /// Used for NTFS and FAT partitions on GPT disks.
    ///
    /// ```text
    /// ebd0a0a2-b9e5-4433-87c0-68b6b72699c7
    /// ```
    pub const GPT_MS_BASIC_DATA: Self = uuid!("ebd0a0a2-b9e5-4433-87c0-68b6b72699c7");

    // =========================================================================
    // Microsoft COM
    // =========================================================================

    /// The `IUnknown` interface GUID.
    ///
    /// The fundamental COM interface that all COM objects must implement.
    ///
    /// ```text
    /// 00000000-0000-0000-c000-000000000046
    /// ```
    pub const COM_IUNKNOWN: Self = uuid!("00000000-0000-0000-c000-000000000046");
}

#[cfg(test)]
mod tests {
    use crate::UUID;

    #[test]
    fn nil_constant_matches_nil_method() {
        assert_eq!(UUID::NIL, UUID::nil());
    }

    #[test]
    fn max_constant_matches_max_method() {
        assert_eq!(UUID::MAX, UUID::max());
    }

    #[test]
    fn nil_is_all_zeros() {
        assert_eq!(
            UUID::NIL.to_string(),
            "00000000-0000-0000-0000-000000000000"
        );
    }

    #[test]
    fn max_is_all_ones() {
        assert_eq!(
            UUID::MAX.to_string(),
            "ffffffff-ffff-ffff-ffff-ffffffffffff"
        );
    }

    #[test]
    fn ns_dns_matches_rfc() {
        assert_eq!(
            UUID::NS_DNS.to_string(),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn ns_url_matches_rfc() {
        assert_eq!(
            UUID::NS_URL.to_string(),
            "6ba7b811-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn ns_oid_matches_rfc() {
        assert_eq!(
            UUID::NS_OID.to_string(),
            "6ba7b812-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn ns_x500_matches_rfc() {
        assert_eq!(
            UUID::NS_X500.to_string(),
            "6ba7b814-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn namespaces_are_v1_osf() {
        assert!(UUID::NS_DNS.is_v1() && UUID::NS_DNS.is_osf());
        assert!(UUID::NS_URL.is_v1() && UUID::NS_URL.is_osf());
        assert!(UUID::NS_OID.is_v1() && UUID::NS_OID.is_osf());
        assert!(UUID::NS_X500.is_v1() && UUID::NS_X500.is_osf());
    }

    #[test]
    fn bluetooth_base_matches_spec() {
        assert_eq!(
            UUID::BLUETOOTH_BASE.to_string(),
            "00000000-0000-1000-8000-00805f9b34fb"
        );
    }

    #[test]
    fn gpt_efi_system_matches_spec() {
        assert_eq!(
            UUID::GPT_EFI_SYSTEM.to_string(),
            "c12a7328-f81f-11d2-ba4b-00a0c93ec93b"
        );
    }

    #[test]
    fn gpt_linux_fs_matches_spec() {
        assert_eq!(
            UUID::GPT_LINUX_FS.to_string(),
            "0fc63daf-8483-4772-8e79-3d69d8477de4"
        );
    }

    #[test]
    fn gpt_linux_swap_matches_spec() {
        assert_eq!(
            UUID::GPT_LINUX_SWAP.to_string(),
            "0657fd6d-a4ab-43c4-84e5-0933c84b4f4f"
        );
    }

    #[test]
    fn gpt_linux_root_x86_64_matches_spec() {
        assert_eq!(
            UUID::GPT_LINUX_ROOT_X86_64.to_string(),
            "4f68bce3-e8cd-4db1-96e7-fbcaf984b709"
        );
    }

    #[test]
    fn gpt_linux_home_matches_spec() {
        assert_eq!(
            UUID::GPT_LINUX_HOME.to_string(),
            "933ac7e1-2eb4-4f13-b844-0e14e2aef915"
        );
    }

    #[test]
    fn gpt_ms_basic_data_matches_spec() {
        assert_eq!(
            UUID::GPT_MS_BASIC_DATA.to_string(),
            "ebd0a0a2-b9e5-4433-87c0-68b6b72699c7"
        );
    }

    #[test]
    fn com_iunknown_matches_spec() {
        assert_eq!(
            UUID::COM_IUNKNOWN.to_string(),
            "00000000-0000-0000-c000-000000000046"
        );
    }
}
