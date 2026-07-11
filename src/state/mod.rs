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
    /// still or moves backward. Clock readings that are not representable as
    /// a 60-bit RFC 4122 tick count are never adopted, so adoption alone can
    /// never move the value out of the range the time-based constructors
    /// accept.
    last_ts: SystemTime,
    /// The node identifier embedded in generated UUIDs.
    node_id: NodeId,
    /// The clock sequence, incremented to disambiguate UUIDs sharing a timestamp.
    seq: u16,
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
/// duplicates it into the child, including the dedicated version-2 counter.
/// Until either side advances far enough to diverge, the parent and child draw
/// from identical clock sequences and can emit identical version-1, version-2,
/// version-6, and DCOM UUIDs. Programs that fork and keep generating UUIDs in
/// both processes should reseed the child, for example by calling
/// `STATE.lock().set_node_id(NodeId::random())` after the fork. Reseeding the
/// node ID does not protect DCOM generation, which embeds a caller-supplied
/// node ID: forked callers of [`UUID::gen_dcom`](crate::UUID::gen_dcom) must
/// pass the child a distinct node ID instead.
pub static STATE: std::sync::LazyLock<Arc<Mutex<State>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(State::default())));
