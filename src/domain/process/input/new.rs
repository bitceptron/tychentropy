use getset::Getters;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::domain::process::error::ProcessError;

#[derive(Debug, Zeroize, ZeroizeOnDrop, Getters, Clone)]
#[get = "pub with_prefix"]
pub struct TychentropyNewInput {
    range_len: u64,
    target_entropy_bytes: u64,
}

impl Default for TychentropyNewInput {
    fn default() -> Self {
        Self {
            range_len: 6,
            target_entropy_bytes: 32,
        }
    }
}

impl TychentropyNewInput {
    pub fn new(range_len: u64, target_entropy_bytes: u64) -> Result<Self, ProcessError> {
        if range_len < 2 {
            return Err(ProcessError::RangeCountIsLessThanTwo);
        }
        if target_entropy_bytes < 1 {
            return Err(ProcessError::TargetEntropyBytesAreLessThanOne);
        }
        Ok(TychentropyNewInput {
            range_len,
            target_entropy_bytes,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works_in_normal_conditions_01() {
        let test = TychentropyNewInput::new(2, 32).unwrap();
        assert_eq!(*test.get_target_entropy_bytes(), 32);
        assert_eq!(*test.get_range_len(), 2);
    }

    #[test]
    fn new_emits_error_when_range_is_less_than_2_01() {
        let test = TychentropyNewInput::new(1, 32);
        assert!(test.is_err());
        assert_eq!(test.err().unwrap(), ProcessError::RangeCountIsLessThanTwo);
    }

    #[test]
    fn new_emits_error_when_target_bytes_are_less_than_1_01() {
        let test = TychentropyNewInput::new(2, 0);
        assert!(test.is_err());
        assert_eq!(
            test.err().unwrap(),
            ProcessError::TargetEntropyBytesAreLessThanOne
        );
    }

    #[test]
    fn default_works_01() {
        let result = TychentropyNewInput::default();
        assert_eq!(result.range_len, 6);
        assert_eq!(result.target_entropy_bytes, 32);
    }
}
