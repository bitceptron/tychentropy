use std::panic;

use nistrs::{
    cusum::cumulative_sums_test,
    fft::fft_test,
    freq::frequency_test,
    linear::linear_complexity_test,
    prelude::{
        approximate_entropy_test, block_frequency_test, longest_run_of_ones_test,
        non_overlapping_template_test, overlapping_template_test, random_excursions_test,
        random_excursions_variant_test, universal_test,
    },
    runs::runs_test,
    serial::serial_test,
    BitsData, TestResultT,
};

use crate::utils::domain_utils::vec_u8_to_bit_string;

use super::statistical_tests_results::StatisticalTestsResults;

pub struct UniformRandomDistStatisticalTest;

impl UniformRandomDistStatisticalTest {
    pub fn approximate_entropy_test(data: &[u8], block_length: usize) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| approximate_entropy_test(&bits_data, block_length));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn approximate_entropy_test_m2(data: &[u8]) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| approximate_entropy_test(&bits_data, 2));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn approximate_entropy_test_m3(data: &[u8]) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| approximate_entropy_test(&bits_data, 3));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn block_frequency_test(data: &[u8], block_size: usize) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| block_frequency_test(&bits_data, block_size).unwrap());
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn cumulative_sums_test(data: &[u8]) -> Option<[TestResultT; 2]> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| cumulative_sums_test(&bits_data));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn fft_test(data: &[u8]) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| fft_test(&bits_data));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn frequency_test(data: &[u8]) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| frequency_test(&bits_data));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn linear_complexity_test(data: &[u8], block_size: usize) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| linear_complexity_test(&bits_data, block_size));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn longest_run_of_ones_test(data: &[u8]) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| longest_run_of_ones_test(&bits_data).unwrap());
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn non_overlapping_template_test(
        data: &[u8],
        template_length_2_to_16: u8,
    ) -> Option<Vec<TestResultT>> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| {
            non_overlapping_template_test(&bits_data, template_length_2_to_16 as usize).unwrap()
        });
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn overlapping_template_test(data: &[u8], template_length: usize) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| overlapping_template_test(&bits_data, template_length));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn random_excursions_test(data: &[u8]) -> Option<[TestResultT; 8]> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| random_excursions_test(&bits_data).unwrap());
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn random_excursions_variant_test(data: &[u8]) -> Option<[TestResultT; 18]> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| random_excursions_variant_test(&bits_data).unwrap());
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn runs_test(data: &[u8]) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| runs_test(&bits_data));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn serial_test(data: &[u8], pattern_size: usize) -> Option<[TestResultT; 2]> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| serial_test(&bits_data, pattern_size));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn universal_test(data: &[u8]) -> Option<TestResultT> {
        let bits_data = BitsData::from_text(vec_u8_to_bit_string(data));
        let result = panic::catch_unwind(|| universal_test(&bits_data));
        match result {
            Ok(test_result) => Some(test_result),
            Err(_) => None,
        }
    }

    pub fn perform_selected_tests(data: &[u8]) -> StatisticalTestsResults {
        let num_bits = vec_u8_to_bit_string(data).len();
        let approximate_entropy_test_m2_result =
            UniformRandomDistStatisticalTest::approximate_entropy_test_m2(data);
        let approximate_entropy_test_m3_result =
            UniformRandomDistStatisticalTest::approximate_entropy_test_m3(data);
        let block_frequency_test_result =
            UniformRandomDistStatisticalTest::block_frequency_test(data, 20.min(num_bits));
        let cumulative_sums_test_result =
            UniformRandomDistStatisticalTest::cumulative_sums_test(data);
        let frequency_test_result = UniformRandomDistStatisticalTest::frequency_test(data);
        let longest_run_of_ones_test_result =
            UniformRandomDistStatisticalTest::longest_run_of_ones_test(data);
        let runs_test_result = UniformRandomDistStatisticalTest::runs_test(data);
        let serial_test_pattern_size_6_result =
            UniformRandomDistStatisticalTest::serial_test(data, 6);

        StatisticalTestsResults::new(
            approximate_entropy_test_m2_result,
            approximate_entropy_test_m3_result,
            block_frequency_test_result,
            cumulative_sums_test_result,
            frequency_test_result,
            longest_run_of_ones_test_result,
            runs_test_result,
            serial_test_pattern_size_6_result,
        )
    }
}

#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn perform_all_tests_works_01() {
        let mut random = [0u8; 32];
        let mut rng = thread_rng();
        rng.fill(&mut random);
        let result = UniformRandomDistStatisticalTest::perform_selected_tests(&random);
        assert!(result.get_approximate_entropy_test_m2_result().is_some());
        assert!(result.get_approximate_entropy_test_m3_result().is_some());
        assert!(result.get_block_frequency_test_result().is_some());
        assert!(result.get_cumulative_sums_test_result().is_some());
        assert!(result.get_frequency_test_result().is_some());
        assert!(result.get_longest_run_of_ones_test_result().is_some());
        assert!(result.get_runs_test_result().is_some());
        assert!(result.get_serial_test_pattern_size_6_result().is_some());
    }

    #[test]
    fn perform_all_tests_works_02() {
        let not_random = [1u8; 8];
        let _result = UniformRandomDistStatisticalTest::perform_selected_tests(&not_random);
    }

    #[test]
    fn perform_all_tests_works_03() {
        let not_random = [0u8; 8];
        let _result = UniformRandomDistStatisticalTest::perform_selected_tests(&not_random);
    }

    #[test]
    fn perform_all_tests_works_04() {
        let not_random = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let _result = UniformRandomDistStatisticalTest::perform_selected_tests(&not_random);
    }
}
