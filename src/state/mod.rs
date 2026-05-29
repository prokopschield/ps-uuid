mod implementations;
mod methods;

use std::{sync::Arc, time::SystemTime};

use parking_lot::Mutex;

use crate::NodeId;

/// The generator state shared across time-based UUID constructors.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    /// The timestamp of the most recently generated UUID.
    pub last_ts: SystemTime,
    /// The node identifier embedded in generated UUIDs.
    pub node_id: NodeId,
    /// The clock sequence, incremented to disambiguate UUIDs sharing a timestamp.
    pub seq: u16,
}

/// The process-wide [`State`] guarding time-based UUID generation.
pub static STATE: std::sync::LazyLock<Arc<Mutex<State>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(State::default())));
