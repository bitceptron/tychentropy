use std::fmt::Display;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum UtilsError {
    TotalBitSpaceIsLessThanDataSignificantBitLength {
        data_significant_bit_len: u32,
        bit_space_len: u32,
    },
}

impl Display for UtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UtilsError::TotalBitSpaceIsLessThanDataSignificantBitLength { data_significant_bit_len, bit_space_len } => write!(f, "Length of significant bits of data is {}, while the space provided is {} bits long. Cannot truncate original data into a lesser space.", data_significant_bit_len, bit_space_len),
        }
    }
}

impl From<UtilsError> for Error {
    fn from(value: UtilsError) -> Self {
        Error::UtilsError(value)
    }
}