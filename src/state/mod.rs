mod implementations;
mod methods;

use std::{sync::Arc, time::SystemTime};

use parking_lot::Mutex;

use crate::NodeId;

/// The generator state shared across time-based UUID constructors.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    /// The timestamp of the most recently issued tick. It never decreases,
    /// and may run slightly ahead of the wall clock while the clock stands
    /// still or moves backward.
    pub last_ts: SystemTime,
    /// The node identifier embedded in generated UUIDs.
    pub node_id: NodeId,
    /// The clock sequence, incremented to disambiguate UUIDs sharing a timestamp.
    pub seq: u16,
    /// The number of clock-sequence values issued for the current tick.
    stalled: u16,
    /// A dedicated counter for the six clock-sequence bits a version-2 UUID
    /// retains. Stepped only by [`State::next_v2`], so traffic through
    /// [`State::next`] cannot realign the surviving bits and duplicate a
    /// version-2 UUID within a timestamp window.
    seq_v2: u8,
}

/// The process-wide [`State`] guarding time-based UUID generation.
///
/// The state is per-process, so a `fork()` without a following `exec()`
/// duplicates it into the child. Until either side advances far enough to
/// diverge, the parent and child draw from identical clock sequences and can
/// emit identical version-1, version-6, and DCOM UUIDs. Programs that fork and
/// keep generating UUIDs in both processes should reseed the child, for example
/// by assigning a fresh [`State::node_id`] after the fork.
pub static STATE: std::sync::LazyLock<Arc<Mutex<State>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(State::default())));
