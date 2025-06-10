mod implementations;
mod methods;

pub const NODE_ID_BYTES: usize = 6;

/// A unique node identifier represented as a 6-byte array.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId {
    pub bytes: [u8; NODE_ID_BYTES],
}
