use crate::{UuidConstructionError, STATE, UUID};
use std::time::SystemTime;

impl UUID {
    /// Generate an RFC 4122 version-6 (time-ordered) UUID.
    ///
    /// The current system time together with the process-wide `NodeId`
    /// and clock sequence held in the global `STATE` are used.
    ///
    /// # Errors
    /// - `TimestampBeforeEpoch` if the current time predates 1582-10-15.
    /// - `TimestampOverflow`    if the current time exceeds 5236-03-31.
    pub fn gen_v6() -> Result<Self, UuidConstructionError> {
        let mut guard = STATE.lock();

        let (timestamp, clock_seq) = guard.next(SystemTime::now());
        let node_id = guard.node_id;

        drop(guard);

        Self::new_v6(timestamp, clock_seq, *node_id)
    }
}

#[allow(clippy::expect_used, clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashSet,
        sync::{Arc, Mutex},
        thread,
    };

    /// Extract the RFC 4122 version (upper 4 bits of byte 6).
    const fn check_version(bytes: &[u8; 16]) -> u8 {
        bytes[6] >> 4
    }

    /// Extract the RFC 4122 variant (upper 2 bits of byte 8).
    const fn check_variant(bytes: &[u8; 16]) -> u8 {
        bytes[8] >> 6
    }

    #[test]
    fn gen_v6_produces_valid_rfc4122_id() {
        let uuid = UUID::gen_v6().expect("generation must succeed");
        let bytes = uuid.as_bytes();

        assert_eq!(
            check_version(bytes),
            0b0110,
            "high-order nibble of byte 6 must equal version 6"
        );
        assert_eq!(
            check_variant(bytes),
            0b10,
            "high-order two bits of byte 8 must equal the RFC 4122 variant"
        );
    }

    /// With a reasonable sample size we should observe no duplicates.
    #[test]
    fn gen_v6_is_unique() {
        const N: usize = 10_000;

        let mut set = HashSet::with_capacity(N);

        for _ in 0..N {
            let id = UUID::gen_v6().expect("generation must succeed").to_string();
            assert!(
                set.insert(id),
                "duplicate UUID generated – monotonicity/clock-seq buggy?"
            );
        }
    }

    /// Ensure the generator is `Send + Sync` and collision-free when
    /// hammered from several threads at once.
    #[test]
    fn gen_v6_thread_safety_and_uniqueness() {
        const THREADS: usize = 8;
        const PER_THREAD: usize = 2_000;

        let global: Arc<Mutex<HashSet<UUID>>> =
            Arc::new(Mutex::new(HashSet::with_capacity(THREADS * PER_THREAD)));

        let mut handles = Vec::with_capacity(THREADS);
        for _ in 0..THREADS {
            let global = Arc::clone(&global);
            handles.push(thread::spawn(move || {
                for _ in 0..PER_THREAD {
                    let id = UUID::gen_v6().expect("generation should succeed");
                    let mut guard = global.lock().unwrap();
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
