use std::time::SystemTime;

use crate::{methods::TICK, State};

use super::next::TICK_CAPACITY;

/// The clock-sequence step for version-2 UUIDs. A version-2 UUID replaces
/// `clock_seq_low` with the local domain, and the variant claims the top two
/// bits of `clock_seq_hi`, so only bits 8..14 of the clock sequence survive.
/// Stepping by 2⁸ changes those bits on every call.
const STEP: u16 = 1 << 8;

impl State {
    /// This method returns the next version-2 [`UUID`](crate::UUID)'s
    /// timestamp and clock sequence, advancing the clock sequence by 2⁸ so
    /// that the six bits a version-2 UUID retains change on every call.
    ///
    /// Unlike [`State::next`], the clock sequence advances even when the
    /// clock has moved to a new tick: a version-2 UUID discards the low 32
    /// bits of the timestamp, so a fresh tick alone does not distinguish it
    /// from its predecessor. The tick budget is shared with [`State::next`]:
    /// each call consumes 2⁸ sequence values, and the next tick is borrowed
    /// once the current tick's capacity is exhausted.
    ///
    /// The returned timestamp is non-decreasing across calls and may run
    /// slightly ahead of the provided clock while the clock stands still.
    ///
    /// # Usage
    ///
    /// ```
    /// use ps_uuid::STATE;
    /// use std::time::SystemTime;
    ///
    /// let (timestamp, clock_seq) = STATE.lock().next_v2(SystemTime::now());
    /// ```
    pub fn next_v2(&mut self, timestamp: SystemTime) -> (SystemTime, u16) {
        self.seq = self.seq.wrapping_add(STEP) & 0x3FFF;

        if timestamp > self.last_ts + TICK {
            // The clock advanced past the current tick: open a new window.
            self.last_ts = timestamp;
            self.stalled = 0;
        } else {
            // Same tick, or a backward-moving clock: account for the values
            // this call consumed.
            self.stalled += STEP;

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
        time::{Duration, UNIX_EPOCH},
    };

    use crate::{NodeId, State};

    /// The six clock-sequence bits that survive in a version-2 UUID.
    const fn surviving_bits(seq: u16) -> u16 {
        (seq >> 8) & 0x3F
    }

    #[test]
    fn frozen_clock_yields_distinct_surviving_bits() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
        };

        let frozen = UNIX_EPOCH + Duration::from_secs(1_000_000_000);

        let mut bits = HashSet::new();
        let mut previous = UNIX_EPOCH;

        for _ in 0..64 {
            let (timestamp, seq) = state.next_v2(frozen);

            assert!(timestamp >= previous, "Timestamps must never decrease.");
            assert!(
                bits.insert(surviving_bits(seq)),
                "The surviving clock-sequence bits must not repeat within 64 calls."
            );

            previous = timestamp;
        }
    }

    #[test]
    fn surviving_bits_wrap_after_64_calls() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
        };

        let frozen = UNIX_EPOCH + Duration::from_secs(1_000_000_000);

        let (_, first) = state.next_v2(frozen);

        for _ in 0..63 {
            state.next_v2(frozen);
        }

        let (_, wrapped) = state.next_v2(frozen);

        assert_eq!(
            surviving_bits(wrapped),
            surviving_bits(first),
            "The 65th call must wrap back to the first surviving value."
        );
    }

    /// Mixed `next` and `next_v2` traffic must never repeat a
    /// (tick, 13-bit clock sequence) pair.
    #[test]
    fn mixed_with_next_never_repeats_a_pair() {
        let mut state = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
        };

        let frozen = UNIX_EPOCH + Duration::from_secs(1_000_000_000);

        let mut pairs = HashSet::new();
        let mut previous = UNIX_EPOCH;

        for round in 0..10_000 {
            let (timestamp, seq) = if round % 3 == 0 {
                state.next_v2(frozen)
            } else {
                state.next(frozen)
            };

            assert!(timestamp >= previous, "Timestamps must never decrease.");
            assert!(
                pairs.insert((timestamp, seq & 0x1FFF)),
                "No (tick, 13-bit clock sequence) pair may repeat."
            );

            previous = timestamp;
        }
    }
}
