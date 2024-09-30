use std::fmt::Display;

use crate::{
    domain::{
        process::error::ProcessError,
        statistics::error::StatisticsError,
    },
    utils::error::UtilsError,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    StatisticsError(StatisticsError),
    UtilsError(UtilsError),
    ProcessError(ProcessError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StatisticsError(statistics_error) => write!(f, "{}", statistics_error),
            Error::UtilsError(utils_error) => write!(f, "{}", utils_error),
            Error::ProcessError(process_error) => write!(f, "{}", process_error),
        }
    }
}
