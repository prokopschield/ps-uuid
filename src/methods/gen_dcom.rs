use std::time::SystemTime;

use crate::{UuidConstructionError, STATE, UUID};

impl UUID {
    /// Generates a Microsoft (DCOM) variant UUID using the current system time.
    ///
    /// The node ID is supplied by the caller. The timestamp and clock sequence
    /// are drawn from the shared [`STATE`]: repeated calls within the same
    /// 100-nanosecond tick advance the clock sequence, and once the 13
    /// sequence bits the DCOM variant retains are exhausted (8192 UUIDs in one
    /// tick) the generator borrows the next tick, so results stay distinct
    /// even under a frozen or coarse system clock. Borrowed timestamps may run
    /// slightly ahead of the real clock. The result uses this crate's
    /// `FILETIME` encoding; see [`UUID::new_dcom`] for the layout and its
    /// caveats.
    ///
    /// # Errors
    /// - [`UuidConstructionError::TimestampBeforeEpoch`] is returned while the
    ///   issued timestamp precedes 1601-01-01, the start of the `FILETIME`
    ///   epoch: a fresh state starts at 1582-10-15, so the error persists
    ///   until a reading at or after 1601-01-01 is adopted.
    ///
    /// Because the state is shared with the version-1 family, adoption of
    /// clock readings is capped at the 60-bit RFC 4122 range (through
    /// 5236-03-31) even though a `FILETIME` reaches the year 60056; a reading
    /// beyond the cap is never adopted, and generation continues from the last
    /// issued tick, so `TimestampOverflow` cannot occur from clock input.
    pub fn gen_dcom(node_id: [u8; 6]) -> Result<Self, UuidConstructionError> {
        let (timestamp, clock_seq) = STATE.lock().next(SystemTime::now());

        Self::new_dcom(timestamp, clock_seq, node_id)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
    use std::{
        collections::HashSet,
        sync::{Arc, Mutex},
        thread,
    };

    use super::*;
    use crate::Variant;

    #[test]
    fn gen_dcom_succeeds_and_sets_variant() {
        let uuid = UUID::gen_dcom([0x01, 0x23, 0x45, 0x67, 0x89, 0xAB])
            .expect("gen_dcom should succeed with the current time");

        assert_eq!(uuid.get_variant(), Variant::DCOM);
    }

    #[test]
    fn gen_dcom_preserves_node_id() {
        let node_id = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let uuid = UUID::gen_dcom(node_id).expect("gen_dcom should succeed with the current time");

        assert_eq!(&uuid.as_bytes()[10..16], &node_id);
    }

    #[test]
    fn gen_dcom_is_unique_across_calls() {
        let node_id = [1, 2, 3, 4, 5, 6];

        let first = UUID::gen_dcom(node_id).expect("gen_dcom should succeed");
        let second = UUID::gen_dcom(node_id).expect("gen_dcom should succeed");

        assert_ne!(
            first, second,
            "consecutive UUIDs must differ via timestamp or clock sequence"
        );
    }

    /// With a reasonable sample size we should observe no duplicates.
    #[test]
    fn gen_dcom_is_unique() {
        const N: usize = 10_000;

        let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

        let mut set = HashSet::with_capacity(N);

        for _ in 0..N {
            let id = UUID::gen_dcom(node_id).expect("generation must succeed");

            assert!(
                set.insert(id),
                "duplicate UUID generated – monotonicity/clock-seq buggy?"
            );
        }
    }

    /// Ensure the generator remains collision-free when hammered from several
    /// threads at once.
    #[test]
    fn gen_dcom_thread_safety_and_uniqueness() {
        const THREADS: usize = 8;
        const PER_THREAD: usize = 2_000;

        let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

        let global: Arc<Mutex<HashSet<UUID>>> =
            Arc::new(Mutex::new(HashSet::with_capacity(THREADS * PER_THREAD)));

        let mut handles = Vec::with_capacity(THREADS);

        for _ in 0..THREADS {
            let global = Arc::clone(&global);

            handles.push(thread::spawn(move || {
                for _ in 0..PER_THREAD {
                    let id = UUID::gen_dcom(node_id).expect("generation must succeed");

                    let mut guard = global.lock().expect("state mutex should not be poisoned");

                    assert!(guard.insert(id), "duplicate across threads");

                    drop(guard);
                }
            }));
        }

        for h in handles {
            h.join().expect("thread panicked");
        }
    }
}
