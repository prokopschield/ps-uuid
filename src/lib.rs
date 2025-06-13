mod gregorian;
mod methods;
mod node_id;

pub use gregorian::Gregorian;
pub use node_id::{NodeId, NODE_ID_BYTES};

pub const UUID_BYTES: usize = 16;

/// A UUID represented as a 16-byte array
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UUID {
    bytes: [u8; UUID_BYTES],
}
