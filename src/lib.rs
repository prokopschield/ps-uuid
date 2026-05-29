#![deny(missing_docs)]
//! A UUID implementation with support for all standard versions and variants.
//!
//! # Features
//!
//! - All UUID versions: v1 (time-based), v2 (DCE), v3 (MD5), v4 (random),
//!   v5 (SHA-1), v6 (reordered time), v7 (Unix epoch time), v8 (custom)
//! - NCS and Microsoft DCOM/COM variants
//! - Full arithmetic and bitwise operations
//! - Zero-copy serialization with `rkyv`
//! - Interoperability with the `uuid` crate
//!
//! # Quick Start
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // Generate a time-sortable v7 UUID (recommended)
//! let id = UUID::gen_v7().unwrap();
//!
//! println!("{id}");
//!
//! // Parse from string
//! let parsed: UUID = "550e8400-e29b-41d4-a716-446655440000".parse().unwrap();
//!
//! // Convert to/from bytes
//! let bytes: [u8; 16] = parsed.into();
//! let restored = UUID::from(bytes);
//!
//! assert_eq!(parsed, restored);
//! ```
//!
//! # Time-Based UUIDs
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // v1: Gregorian time + node ID
//! let v1 = UUID::gen_v1().unwrap();
//!
//! // v7: Unix epoch milliseconds (recommended for databases)
//! let v7 = UUID::gen_v7().unwrap();
//! ```
//!
//! # Name-Based UUIDs
//!
//! ```
//! use ps_uuid::UUID;
//!
//! // v3: MD5 hash
//! let v3 = UUID::new_v3(&UUID::nil(), b"example");
//!
//! // v5: SHA-1 hash (preferred over v3)
//! let v5 = UUID::new_v5(&UUID::nil(), b"example");
//! ```
//!
//! # Feature Flags
//!
//! - `serde`: Serialization support via Serde
//! - `rkyv`: Zero-copy deserialization via rkyv
//! - `num_traits`: Numeric trait implementations
//! - `uuid-crate-compat`: Interop with the `uuid` crate via `UuidCompat`

mod constants;
mod error;
mod features;
mod gregorian;
mod helpers;
mod implementations;
mod macros;
mod methods;
mod node_id;
mod state;
mod variant;

pub use error::{DurationToTicksError, UuidConstructionError, UuidParseError};
pub use gregorian::Gregorian;
pub use helpers::{md5, sha1, to_hex, Md5, Sha1, ToHex};
pub use methods::{Braced, Hyphenated, NcsUuidError, Simple, Urn};
pub use node_id::{NodeId, NODE_ID_BYTES};
pub use state::{State, STATE};
pub use variant::Variant;

#[cfg(feature = "uuid-crate-compat")]
pub use features::uuid_crate_compat::{Uuid, UuidCompat};

/// The number of bytes in a [`UUID`].
pub const UUID_BYTES: usize = 16;

/// A UUID represented as a 16-byte array
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UUID {
    bytes: [u8; UUID_BYTES],
}
