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
    ///    [`State`]'s last timestamp and is representable as a 60-bit
    ///    RFC 4122 tick count, it is stored as the new last timestamp and
    ///    returned with the clock sequence unchanged.
    /// 2. Otherwise (a same-tick call, a backward-moving clock, or a reading
    ///    too far in the future to represent), the last timestamp is kept and
    ///    the clock sequence is incremented modulo 2¹⁴.
    /// 3. Once 2¹³ sequence values have been issued for the current tick (the
    ///    capacity of the 13 sequence bits a DCOM UUID retains), the last
    ///    timestamp advances by one tick, borrowing from the future rather
    ///    than repeating a sequence value.
    ///
    /// The returned timestamp is therefore non-decreasing across calls and may
    /// run slightly ahead of the provided clock while the clock stands still.
    /// An unrepresentable reading is never adopted: the same-tick path applies
    /// instead, so generation continues from the last issued tick and resumes
    /// normal adoption once the clock recovers.
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
        if timestamp > self.last_ts + TICK && Self::is_adoptable(timestamp) {
            // The clock advanced past the current tick and the reading is
            // representable: open a new window.
            self.last_ts = timestamp;
            self.stalled = 0;
        } else {
            // Same tick, a backward-moving clock, or an unrepresentable
            // reading: advance the sequence.
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

    use crate::{Gregorian, NodeId, State, STATE, UUID};

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
            seq_v2: 0,
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

    /// A reading beyond the representable 60-bit tick range must not be
    /// adopted: the sequence advances on the last good tick instead, so a
    /// transient bogus clock cannot poison the state.
    #[test]
    fn far_future_reading_is_not_adopted() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
            seq_v2: 0,
        };

        let bogus = Gregorian::epoch() + Duration::from_secs(200_000_000_000);

        let (timestamp, seq) = state.next(bogus);

        assert_eq!(timestamp, UNIX_EPOCH);
        assert_eq!(seq, 1);
        assert!(UUID::system_time_to_ticks(timestamp).is_ok());
    }

    /// After a far-future glitch, a sane reading resumes normal adoption.
    #[test]
    fn recovers_after_far_future_glitch() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
            seq_v2: 0,
        };

        let bogus = Gregorian::epoch() + Duration::from_secs(200_000_000_000);

        for _ in 0..10 {
            let (timestamp, _) = state.next(bogus);

            assert!(UUID::system_time_to_ticks(timestamp).is_ok());
        }

        let sane = UNIX_EPOCH + Duration::from_secs(1_000_000_000);

        let (timestamp, _) = state.next(sane);

        assert_eq!(timestamp, sane);
    }

    /// The maximum representable tick (2⁶⁰ − 1 ticks after 1582-10-15) is
    /// adopted; one 100 ns tick past it is not.
    #[test]
    fn adoption_boundary_at_the_maximum_tick() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
            seq_v2: 0,
        };

        let last_within = Gregorian::epoch() + Duration::new(115_292_150_460, 684_697_500);
        let first_beyond = last_within + Duration::from_nanos(100);

        let (timestamp, _) = state.next(last_within);

        assert_eq!(timestamp, last_within);

        let (timestamp, _) = state.next(first_beyond);

        assert_eq!(timestamp, last_within);
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
            seq_v2: 0,
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
