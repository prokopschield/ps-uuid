pub const BYTES: usize = 16;

/// A UUID represented as a 16-byte array
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UUID {
    bytes: [u8; BYTES],
}
