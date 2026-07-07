use std::time::SystemTime;

use crate::{methods::TICK, State};

impl State {
    /// This method returns the next version-2 [`UUID`](crate::UUID)'s
    /// timestamp and clock sequence, advancing a dedicated counter so that the
    /// six clock-sequence bits a version-2 UUID retains change on every call.
    ///
    /// A version-2 UUID discards the low 32 timestamp bits (they carry the
    /// local ID) and byte 9 (it carries the domain), so within one 2³²-tick
    /// (about 429 s) window only six clock-sequence bits distinguish UUIDs
    /// sharing a domain, local ID, and node. Those six bits come from a
    /// counter stepped only here, independent of the shared clock sequence
    /// that [`State::next`] advances, so version-1, version-6, and DCOM
    /// traffic cannot realign them. A window therefore yields all 64 distinct
    /// values before wrapping.
    ///
    /// The returned timestamp tracks the shared last timestamp and is
    /// non-decreasing across calls; it may run slightly ahead of the provided
    /// clock while the clock stands still.
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
        if timestamp > self.last_ts + TICK {
            // The clock advanced past the current tick: adopt it and reset the
            // shared tick budget that `next` maintains.
            self.last_ts = timestamp;
            self.stalled = 0;
        }

        self.seq_v2 = self.seq_v2.wrapping_add(1) & 0x3F;

        (self.last_ts, u16::from(self.seq_v2) << 8)
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
            seq_v2: 0,
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
            seq_v2: 0,
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

    /// Interleaved `next` traffic must not change the version-2 sequence.
    ///
    /// The six bits a version-2 UUID retains come from a dedicated counter, so
    /// version-1, version-6, and DCOM calls through `next` cannot perturb them:
    /// the surviving values a run of `next_v2` calls produces are identical
    /// whether or not `next` calls fall in between.
    #[test]
    fn interleaved_next_does_not_change_v2_sequence() {
        let frozen = UNIX_EPOCH + Duration::from_secs(1_000_000_000);

        let mut clean = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
            seq_v2: 0,
        };

        let clean_bits: Vec<u16> = (0..64)
            .map(|_| surviving_bits(clean.next_v2(frozen).1))
            .collect();

        let mut noisy = State {
            last_ts: UNIX_EPOCH,
            node_id: NodeId::random(),
            seq: 0,
            stalled: 0,
            seq_v2: 0,
        };

        let mut noisy_bits = Vec::with_capacity(64);

        for i in 0..64 {
            for _ in 0..(i * 7 + 3) {
                noisy.next(frozen);
            }

            noisy_bits.push(surviving_bits(noisy.next_v2(frozen).1));
        }

        assert_eq!(
            clean_bits, noisy_bits,
            "Interleaved next() traffic must not change the version-2 sequence."
        );
        assert_eq!(
            clean_bits.iter().collect::<HashSet<_>>().len(),
            64,
            "A window must yield all 64 distinct surviving values."
        );
    }
}
