use std::fmt::Display;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum StatisticsError {
    NotEnoughSamplesForChiSquaredTest {
        num_given_samples: u64,
        num_required_samples: u64,
    },
    RangeBoundsDoNotFullyCoverAllObservations,
    RangeBoundsAreIncorrect,

    BlockFrequencyTestError,
    LongestRunOfOnesTestError,
    NonOverlappingTemplateTestError,
    RandomExcursionsTestError(String),
    RandomExcursionsVariantTestError(String),
}

impl Display for StatisticsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatisticsError::NotEnoughSamplesForChiSquaredTest { num_given_samples, num_required_samples } => write!(f, "Number of samples are {}, which is less than the minimum number of samples per possible outcome required to perform the chi-squared test, that is 5 times the number of possible outcomes or {} in this case.", num_given_samples, num_required_samples),
            StatisticsError::RangeBoundsDoNotFullyCoverAllObservations => write!(f, "There are observations in data that are not present in the bases."),
            StatisticsError::RangeBoundsAreIncorrect => write!(f, "Provided range min and max do not form a correct inclusive range of more than one member."),
            StatisticsError::BlockFrequencyTestError => write!(f, "Size of block must be lower than number of bits"),
            StatisticsError::LongestRunOfOnesTestError => write!(f, "At least 128 bits are required."),
            StatisticsError::NonOverlappingTemplateTestError => write!(f, "In this implementation for non-overlapping template test, 2 <= m <= 16"),
            StatisticsError::RandomExcursionsTestError(msg) => write!(f, "{}", msg),
            StatisticsError::RandomExcursionsVariantTestError(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<StatisticsError> for Error {
    fn from(value: StatisticsError) -> Self {
        Error::StatisticsError(value)
    }
}
