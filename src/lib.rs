mod error;
mod gregorian;
mod implementations;
mod methods;
mod node_id;
mod variant;

pub use error::UuidConstructionError;
pub use gregorian::Gregorian;
pub use methods::NcsUuidError;
pub use node_id::{NodeId, NODE_ID_BYTES};
pub use variant::Variant;

pub const UUID_BYTES: usize = 16;

/// A UUID represented as a 16-byte array
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UUID {
    bytes: [u8; UUID_BYTES],
}
