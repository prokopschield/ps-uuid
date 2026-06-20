use std::time::SystemTime;

use crate::{UuidConstructionError, STATE, UUID};

impl UUID {
    /// Generates a Microsoft (DCOM) variant UUID using the current system time.
    ///
    /// The node ID is supplied by the caller. The timestamp and clock sequence
    /// are drawn from the shared [`STATE`](crate::STATE), so repeated calls
    /// within the same 100-nanosecond tick advance the clock sequence to keep
    /// the results distinct. The DCOM variant leaves 13 bits for the clock
    /// sequence, so distinctness holds for up to 8192 calls within a single
    /// tick before the sequence repeats.
    ///
    /// # Errors
    /// - [`UuidConstructionError`] is returned if the current system time is out of range.
    pub fn gen_dcom(node_id: [u8; 6]) -> Result<Self, UuidConstructionError> {
        let (timestamp, clock_seq) = STATE.lock().next(SystemTime::now());

        Self::new_dcom(timestamp, clock_seq, node_id)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]
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
}
