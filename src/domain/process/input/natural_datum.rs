use getset::Getters;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::domain::process::error::ProcessError;

#[derive(Debug, Zeroize, ZeroizeOnDrop, Getters, Clone)]
#[get = "pub with_prefix"]
pub struct NaturalDatum {
    value: u64,
}

impl NaturalDatum {
    pub fn new(range: u64, value: u64) -> Result<Self, ProcessError> {
        if range < 2 {
            Err(ProcessError::MaxInclusiveLessThanTwo)
        } else if !(1..=range).contains(&value) {
            Err(ProcessError::NaturalDatumOutOfRange { value, range })
        } else {
            Ok(NaturalDatum { value })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_works_normal_in_min_value_01() {
        let test = NaturalDatum::new(6, 1).unwrap();
        assert_eq!(*test.get_value(), 1);
    }

    #[test]
    fn new_works_normal_in_mid_value_01() {
        let test = NaturalDatum::new(6, 3).unwrap();
        assert_eq!(*test.get_value(), 3);
    }

    #[test]
    fn new_works_normal_in_max_value_01() {
        let test = NaturalDatum::new(6, 6).unwrap();
        assert_eq!(*test.get_value(), 6);
    }

    #[test]
    fn new_emits_error_in_zero_as_datum_01() {
        let test = NaturalDatum::new(6, 0);
        assert!(test.is_err());
        assert_eq!(
            test.err().unwrap(),
            ProcessError::NaturalDatumOutOfRange { value: 0, range: 6 }
        )
    }

    #[test]
    fn new_emits_error_in_out_of_range_datum_01() {
        let test = NaturalDatum::new(6, 7);
        assert!(test.is_err());
        assert_eq!(
            test.err().unwrap(),
            ProcessError::NaturalDatumOutOfRange { value: 7, range: 6 }
        )
    }

    #[test]
    fn new_emits_error_in_max_inclusive_less_than_2_01() {
        let test = NaturalDatum::new(0, 2);
        assert!(test.is_err());
        assert_eq!(test.err().unwrap(), ProcessError::MaxInclusiveLessThanTwo)
    }

    #[test]
    fn new_emits_error_in_max_inclusive_less_than_2_02() {
        let test = NaturalDatum::new(1, 2);
        assert!(test.is_err());
        assert_eq!(test.err().unwrap(), ProcessError::MaxInclusiveLessThanTwo)
    }
}
