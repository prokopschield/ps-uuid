use std::time::SystemTime;

use crate::{methods::TICK, State};

/// The number of clock-sequence values that may be issued for one tick.
/// The DCOM variant overwrites the top 3 of the 14 sequence bits, keeping
/// 13, so a run of more than 2¹³ consecutive values would repeat mod 2¹³.
const TICK_CAPACITY: u16 = 1 << 13;

impl State {
    /// This method returns the next time-based [`UUID`](crate::UUID)'s
    /// timestamp and clock sequence, never issuing the same (100 ns tick,
    /// clock sequence) pair twice, even under a frozen, coarse, or
    /// backward-moving clock.
    ///
    /// 1. If the provided timestamp is more than one 100 ns tick after this
    ///    [`State`]'s last timestamp, it is stored as the new last timestamp
    ///    and returned with the clock sequence unchanged.
    /// 2. Otherwise (a same-tick call or a backward-moving clock), the last
    ///    timestamp is kept and the clock sequence is incremented modulo 2¹⁴.
    /// 3. Once 2¹³ sequence values have been issued for the current tick (the
    ///    capacity of the 13 sequence bits a DCOM UUID retains), the last
    ///    timestamp advances by one tick, borrowing from the future rather
    ///    than repeating a sequence value.
    ///
    /// The returned timestamp is therefore non-decreasing across calls and may
    /// run slightly ahead of the provided clock while the clock stands still.
    ///
    /// # Usage
    ///
    /// ```
    /// use ps_uuid::STATE;
    /// use std::time::SystemTime;
    ///
    /// let (timestamp, clock_seq) = STATE.lock().next(SystemTime::now());
    /// ```
    pub fn next(&mut self, timestamp: SystemTime) -> (SystemTime, u16) {
        if timestamp > self.last_ts + TICK {
            // The clock advanced past the current tick: open a new window.
            self.last_ts = timestamp;
            self.stalled = 0;
        } else {
            // Same tick, or a backward-moving clock: advance the sequence.
            self.seq = (self.seq.wrapping_add(1)) & 0x3FFF;
            self.stalled += 1;

            if self.stalled >= TICK_CAPACITY {
                // The sequence values for this tick are exhausted: borrow
                // the next tick instead of repeating one.
                self.last_ts += TICK;
                self.stalled = 0;
            }
        }

        (self.last_ts, self.seq)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::{NodeId, State, STATE};

    #[test]
    fn monotonic_and_unique() {
        let mut guard = STATE.lock_arc();

        let mut pairs = HashSet::with_capacity(100_000);
        let mut previous = UNIX_EPOCH;

        for _ in 0..100_000 {
            let (timestamp, seq) = guard.next(SystemTime::now());

            assert!(timestamp >= previous, "Timestamps must never decrease.");
            assert!(
                pairs.insert((timestamp, seq)),
                "No (timestamp, clock sequence) pair may repeat."
            );

            previous = timestamp;
        }

        drop(guard);
    }

    /// A frozen clock must still yield unique pairs: the sequence advances
    /// within a tick, and the next tick is borrowed once the 13 bits a DCOM
    /// UUID retains are exhausted.
    #[test]
    fn frozen_clock_uniqueness() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
        };

        let frozen = UNIX_EPOCH + Duration::from_secs(1_000_000_000);

        let mut pairs = HashSet::with_capacity(20_000);
        let mut ticks = HashSet::new();
        let mut previous = UNIX_EPOCH;

        for _ in 0..20_000 {
            let (timestamp, seq) = state.next(frozen);

            assert!(timestamp >= previous, "Timestamps must never decrease.");
            assert!(
                pairs.insert((timestamp, seq & 0x1FFF)),
                "No (tick, 13-bit clock sequence) pair may repeat."
            );

            ticks.insert(timestamp);
            previous = timestamp;
        }

        // 20 000 calls at one instant span exactly ⌈20 000 / 8192⌉ = 3 ticks.
        assert_eq!(ticks.len(), 3);
        assert!(previous <= frozen + Duration::from_nanos(200));
    }

    /// A backward-moving clock must neither regress the returned timestamp
    /// nor repeat a (tick, clock sequence) pair.
    #[test]
    fn backward_clock_never_regresses() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
        };

        let now = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
        let past = now - Duration::from_secs(3600);

        let mut pairs = HashSet::with_capacity(200);
        let mut previous = UNIX_EPOCH;

        for _ in 0..100 {
            for timestamp in [now, past] {
                let (issued, seq) = state.next(timestamp);

                assert!(issued >= previous, "Timestamps must never decrease.");
                assert!(
                    pairs.insert((issued, seq & 0x1FFF)),
                    "No (tick, 13-bit clock sequence) pair may repeat."
                );

                previous = issued;
            }
        }
    }
}
