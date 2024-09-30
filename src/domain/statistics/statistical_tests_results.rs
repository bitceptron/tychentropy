
use getset::Getters;
use nistrs::TestResultT;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Zeroize, ZeroizeOnDrop, Getters, Default, Clone)]
#[get = "pub with_prefix"]
pub struct StatisticalTestsResults {
    approximate_entropy_test_m2_result: Option<TestResultT>,
    approximate_entropy_test_m3_result: Option<TestResultT>,
    block_frequency_test_result: Option<TestResultT>,
    cumulative_sums_test_result: Option<[TestResultT; 2]>,
    frequency_test_result: Option<TestResultT>,
    longest_run_of_ones_test_result: Option<TestResultT>,
    runs_test_result: Option<TestResultT>,
    serial_test_pattern_size_6_result: Option<[TestResultT; 2]>,
}

impl StatisticalTestsResults {
    pub fn new(
        approximate_entropy_test_m2_result: Option<TestResultT>,
        approximate_entropy_test_m3_result: Option<TestResultT>,
        block_frequency_test_result: Option<TestResultT>,
        cumulative_sums_test_result: Option<[TestResultT; 2]>,
        frequency_test_result: Option<TestResultT>,
        longest_run_of_ones_test_result: Option<TestResultT>,
        runs_test_result: Option<TestResultT>,
        serial_test_pattern_size_6_result: Option<[TestResultT; 2]>,
    ) -> Self {
        StatisticalTestsResults {
            approximate_entropy_test_m2_result,
            approximate_entropy_test_m3_result,
            block_frequency_test_result,
            cumulative_sums_test_result,
            frequency_test_result,
            longest_run_of_ones_test_result,
            runs_test_result,
            serial_test_pattern_size_6_result,
        }
    }
}

#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop, PartialEq, Default)]
pub enum StatsTestState {
    #[default]
    NotAvailable,
    Passed,
    Failed,
}

#[derive(Debug, Zeroize, ZeroizeOnDrop, Getters, Default)]
#[get = "pub with_prefix"]
pub struct StatisticalTestsOverview {
    approximate_entropy_test_m2_result: StatsTestState,
    approximate_entropy_test_m3_result: StatsTestState,
    block_frequency_test_result: StatsTestState,
    cumulative_sums_test_result: StatsTestState,
    frequency_test_result: StatsTestState,
    longest_run_of_ones_test_result: StatsTestState,
    runs_test_result: StatsTestState,
    serial_test_pattern_size_6_result: StatsTestState,
}

impl From<StatisticalTestsResults> for StatisticalTestsOverview {
    fn from(value: StatisticalTestsResults) -> Self {
        let approximate_entropy_test_m2_result = match value.approximate_entropy_test_m2_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let approximate_entropy_test_m3_result = match value.approximate_entropy_test_m3_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let block_frequency_test_result = match value.block_frequency_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let cumulative_sums_test_result = match value.cumulative_sums_test_result {
            Some(result) => {
                if result.iter().all(|res| res.0) {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let frequency_test_result = match value.frequency_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let longest_run_of_ones_test_result = match value.longest_run_of_ones_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let runs_test_result = match value.runs_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let serial_test_pattern_size_6_result = match value.serial_test_pattern_size_6_result {
            Some(result) => {
                if result.iter().all(|res| res.0) {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        StatisticalTestsOverview {
            approximate_entropy_test_m2_result,
            approximate_entropy_test_m3_result,
            block_frequency_test_result,
            cumulative_sums_test_result,
            frequency_test_result,
            longest_run_of_ones_test_result,
            runs_test_result,
            serial_test_pattern_size_6_result,
        }
    }
}

impl From<&StatisticalTestsResults> for StatisticalTestsOverview {
    fn from(value: &StatisticalTestsResults) -> Self {
        let approximate_entropy_test_m2_result = match value.approximate_entropy_test_m2_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let approximate_entropy_test_m3_result = match value.approximate_entropy_test_m3_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let block_frequency_test_result = match value.block_frequency_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let cumulative_sums_test_result = match value.cumulative_sums_test_result {
            Some(result) => {
                if result.iter().all(|res| res.0) {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let frequency_test_result = match value.frequency_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let longest_run_of_ones_test_result = match value.longest_run_of_ones_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };

        let runs_test_result = match value.runs_test_result {
            Some(result) => {
                if result.0 {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        let serial_test_pattern_size_6_result = match value.serial_test_pattern_size_6_result {
            Some(result) => {
                if result.iter().all(|res| res.0) {
                    StatsTestState::Passed
                } else {
                    StatsTestState::Failed
                }
            }
            None => StatsTestState::NotAvailable,
        };
        StatisticalTestsOverview {
            approximate_entropy_test_m2_result,
            approximate_entropy_test_m3_result,
            block_frequency_test_result,
            cumulative_sums_test_result,
            frequency_test_result,
            longest_run_of_ones_test_result,
            runs_test_result,
            serial_test_pattern_size_6_result,
        }
    }
}
