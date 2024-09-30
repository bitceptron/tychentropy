use std::{ops::BitXor, panic};

use bip39::Mnemonic;
use getset::Getters;
use rand::RngCore;
use ring::rand::{SecureRandom, SystemRandom};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::utils::domain_utils::u64_to_binary;

use super::{
    mnemonics::MnemonicLength,
    process::{
        error::ProcessError,
        input::{new::TychentropyNewInput, NaturalDatum},
    },
    statistics::{
        statistical_tests_results::StatisticalTestsResults,
        uniform_random_test::UniformRandomDistStatisticalTest,
    },
};

#[derive(Debug, Zeroize, ZeroizeOnDrop, Getters, Clone)]
#[get = "pub with_prefix"]
pub struct Tychentropy {
    /// Range length of possible numbers that are to be drawn. Think of this as how many sides your dice has. It must have at least 2 to produce any entropy.
    range: u64,
    full_bits_in_each_datum: u32,
    target_entropy_bytes: u64,
    target_entropy_bits: u64,
    sequence: Vec<u64>,
    entropy_generating_sequence: Vec<u64>,
    generated_entropy_bits: u64,
    is_entropy_ready: bool,
    entropy_bit_string: String,
    entropy_bit_vector: Vec<u8>,
    entropy_bytes_vector: Vec<u8>,
    rng_entropy_bytes_vector: Vec<u8>,
    mixed_entropy_and_rng_bytes_vector: Vec<u8>,
    final_entropy_bytes_vector: Vec<u8>,
    statistical_test_results: StatisticalTestsResults,
    mnemonic_length: Option<MnemonicLength>,
    mnemonic: Option<Mnemonic>,
}

impl Default for Tychentropy {
    fn default() -> Self {
        Self::new(TychentropyNewInput::new(6, 32).unwrap())
    }
}

impl Tychentropy {
    pub fn new(input: TychentropyNewInput) -> Self {
        let range_len = *input.get_range_len();
        let full_bits_in_each_datum = range_len.ilog2();
        let target_entropy_bytes = *input.get_target_entropy_bytes();
        let target_entropy_bits = target_entropy_bytes * 8;
        let sequence = vec![];
        let entropy_generating_sequence = vec![];
        let generated_entropy_bits = 0;
        let is_entropy_ready = false;
        let entropy_bit_string = String::new();
        let entropy_bit_vector = vec![];
        let entropy_bytes_vector = vec![];
        let rng_entropy_bytes_vector = vec![];
        let mixed_entropy_and_rng_bytes_vector = vec![];
        let final_entropy_bytes_vector = vec![];
        let statistical_test_results = StatisticalTestsResults::default();
        let mnemonic_length = match *input.get_target_entropy_bytes() {
            16 => Some(MnemonicLength::Twelve),
            20 => Some(MnemonicLength::Fifteen),
            24 => Some(MnemonicLength::Eighteen),
            28 => Some(MnemonicLength::TwentyOne),
            32 => Some(MnemonicLength::TwentyFour),
            _ => None,
        };
        let mnemonic = None;
        Tychentropy {
            range: range_len,
            full_bits_in_each_datum,
            target_entropy_bytes,
            target_entropy_bits,
            sequence,
            entropy_generating_sequence,
            generated_entropy_bits,
            is_entropy_ready,
            entropy_bit_string,
            entropy_bit_vector,
            entropy_bytes_vector,
            rng_entropy_bytes_vector,
            mixed_entropy_and_rng_bytes_vector,
            final_entropy_bytes_vector,
            statistical_test_results,
            mnemonic_length,
            mnemonic,
        }
    }

    pub fn add_natural_datum(&mut self, datum: NaturalDatum) -> Result<(), ProcessError> {
        let datum_value = *datum.get_value();
        if self.is_entropy_ready {
            Err(ProcessError::EntropyAlreadyCreated)
        } else if datum_value > self.range {
            Err(ProcessError::NaturalDatumOutOfRange {
                value: datum_value,
                range: self.range,
            })
        } else {
            self.sequence.push(datum_value);
            let zero_indexed_datum_value = datum_value - 1;
            let cutoff_value = 2u64.pow(self.full_bits_in_each_datum);
            if zero_indexed_datum_value < cutoff_value {
                self.entropy_generating_sequence
                    .push(zero_indexed_datum_value);
                let zero_indexed_bit_string =
                    u64_to_binary(zero_indexed_datum_value, self.full_bits_in_each_datum)
                        .map_err(|err| err)
                        .unwrap();
                self.entropy_bit_string
                    .push_str(&zero_indexed_bit_string.clone());
                let zero_indexed_bit_vec = zero_indexed_bit_string
                    .chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();
                self.entropy_bit_vector.extend(zero_indexed_bit_vec);
                self.generated_entropy_bits += self.full_bits_in_each_datum as u64;
                if self.generated_entropy_bits >= self.target_entropy_bits {
                    self.is_entropy_ready = true;
                    self.entropy_bit_string
                        .truncate(self.target_entropy_bits as usize);
                    self.entropy_bit_vector
                        .truncate(self.target_entropy_bits as usize);

                    let entropy_bytes_vector = self
                        .entropy_bit_vector
                        .chunks(8)
                        .map(|byte_sized_chunk| {
                            byte_sized_chunk
                                .iter()
                                .enumerate()
                                .fold(0, |acc, (index, bit)| acc + bit * 2u8.pow(index as u32))
                        })
                        .collect::<Vec<u8>>();
                    self.entropy_bytes_vector = entropy_bytes_vector.clone();
                    self.final_entropy_bytes_vector = entropy_bytes_vector;
                    self.perform_selected_statistical_tests();
                    self.generate_mnemonic();
                }
            }
            Ok(())
        }
    }

    pub fn mix_with_rng(&mut self) -> Result<(), ProcessError> {
        if !self.is_entropy_ready {
            Err(ProcessError::EntropyBitsAreNotReady {
                required_entropy_bits: self.target_entropy_bits,
                current_entropy_bits: self.generated_entropy_bits,
            })
        } else {
            let mut rand_rng = rand::thread_rng();
            let mut rand_rng_bytes = vec![0u8; self.target_entropy_bytes as usize];
            rand_rng.fill_bytes(&mut rand_rng_bytes);

            let mut ring_rng_bytes = vec![0u8; self.target_entropy_bytes as usize];
            let sys_random = SystemRandom::new();
            sys_random.fill(&mut ring_rng_bytes).unwrap();

            let mixed = rand_rng_bytes
                .iter()
                .zip(ring_rng_bytes.iter())
                .map(|(rand_byte, ring_byte)| rand_byte.bitxor(ring_byte))
                .collect();
            self.rng_entropy_bytes_vector = mixed;
            self.final_entropy_bytes_vector = self
                .entropy_bytes_vector
                .iter()
                .zip(self.rng_entropy_bytes_vector.iter())
                .map(|(data, rng)| data.bitxor(rng))
                .collect();
            self.perform_selected_statistical_tests();
            self.generate_mnemonic();
            Ok(())
        }
    }

    pub fn put_data_from_another_tychentropy(&mut self, other: Tychentropy) {
        self.range = other.range;
        self.full_bits_in_each_datum = other.full_bits_in_each_datum;
        self.target_entropy_bits = other.target_entropy_bits;
        self.target_entropy_bytes = other.target_entropy_bytes;
        self.sequence = other.sequence.clone();
        self.entropy_generating_sequence = other.entropy_generating_sequence.clone();
        self.generated_entropy_bits = other.generated_entropy_bits;
        self.is_entropy_ready = other.is_entropy_ready;
        self.entropy_bit_string = other.entropy_bit_string.clone();
        self.entropy_bit_vector = other.entropy_bit_vector.clone();
        self.entropy_bytes_vector = other.entropy_bytes_vector.clone();
        self.rng_entropy_bytes_vector = other.rng_entropy_bytes_vector.clone();
        self.mixed_entropy_and_rng_bytes_vector = other.mixed_entropy_and_rng_bytes_vector.clone();
        self.final_entropy_bytes_vector = other.final_entropy_bytes_vector.clone();
        self.mnemonic_length = other.mnemonic_length.clone();
        self.mnemonic = other.mnemonic.clone();
        self.statistical_test_results = other.statistical_test_results.clone();
    }

    pub fn reset_data(&mut self) {
        let input = TychentropyNewInput::new(self.range, self.target_entropy_bytes).unwrap();
        let new_tych_entropy = Tychentropy::new(input);
        self.put_data_from_another_tychentropy(new_tych_entropy);
    }

    pub fn generate_mnemonic(&mut self) {
        if self.is_entropy_ready && self.mnemonic_length.is_some() {
            self.mnemonic = Some(Mnemonic::from_entropy(&self.final_entropy_bytes_vector).unwrap())
        }
    }

    pub fn perform_selected_statistical_tests(&mut self) {
        let data = self.final_entropy_bytes_vector.clone();
        if !data.is_empty() {
            let results = panic::catch_unwind(|| {
                UniformRandomDistStatisticalTest::perform_selected_tests(&data)
            });
            match results {
                Ok(res) => self.statistical_test_results = res,
                Err(_) => {}
            }
        };
    }

    pub fn recover_original_entropy_bytes_after_mix(&mut self) {
        self.final_entropy_bytes_vector = self.entropy_bytes_vector.clone();
        self.mixed_entropy_and_rng_bytes_vector = vec![];
        self.rng_entropy_bytes_vector = vec![];
        self.perform_selected_statistical_tests();
        self.generate_mnemonic();
    }
}

#[cfg(test)]
mod test {
    use crate::utils::test_utils::{
        generate_pre_determined_series_of_data_till_entropy_is_full,
        generate_random_data_sequence_till_entropy_is_full,
    };

    use super::*;

    #[test]
    fn default_works_01() {
        let default = Tychentropy::default();
        assert_eq!(*default.get_range(), 6);
        assert_eq!(*default.get_target_entropy_bytes(), 32);
        assert_eq!(*default.get_target_entropy_bits(), 256);
        assert_eq!(*default.get_full_bits_in_each_datum(), 2);
    }

    #[test]
    fn new_works_in_normal_condition_01() {
        let range = 3;
        let num_bytes_of_target_entropy = 5;
        let input = TychentropyNewInput::new(range, num_bytes_of_target_entropy).unwrap();
        let tychentropy = Tychentropy::new(input);
        assert_eq!(tychentropy.range, 3);
        assert_eq!(tychentropy.full_bits_in_each_datum, 1);
        assert_eq!(tychentropy.target_entropy_bytes, 5);
        assert_eq!(tychentropy.target_entropy_bits, 40);
        assert_eq!(tychentropy.sequence, vec![]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![]);
        assert_eq!(tychentropy.generated_entropy_bits, 0);
        assert_eq!(tychentropy.is_entropy_ready, false);
        assert_eq!(tychentropy.entropy_bit_string, "");
        assert_eq!(tychentropy.entropy_bit_vector, vec![]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector, vec![]);
    }

    #[test]
    fn add_datum_works_in_normal_condition_01() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let datum = NaturalDatum::new(range, 5).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![5]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![]);
        assert_eq!(tychentropy.generated_entropy_bits, 0);
        assert_eq!(tychentropy.is_entropy_ready, false);
        assert_eq!(tychentropy.entropy_bit_string, "");
        assert_eq!(tychentropy.entropy_bit_vector, vec![]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector, vec![]);
    }

    #[test]
    fn add_datum_works_in_normal_condition_02() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let datum = NaturalDatum::new(range, 4).unwrap();
        let _ = tychentropy.add_natural_datum(datum);

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![4]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![3]);
        assert_eq!(tychentropy.generated_entropy_bits, 2);
        assert_eq!(tychentropy.is_entropy_ready, false);
        assert_eq!(tychentropy.entropy_bit_string, "11");
        assert_eq!(tychentropy.entropy_bit_vector, vec![1, 1]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector, vec![]);
    }

    #[test]
    fn add_datum_works_in_normal_condition_03() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let datum = NaturalDatum::new(range, 4).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 3).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![4, 3]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![3, 2]);
        assert_eq!(tychentropy.generated_entropy_bits, 4);
        assert_eq!(tychentropy.is_entropy_ready, false);
        assert_eq!(tychentropy.entropy_bit_string, "1110");
        assert_eq!(tychentropy.entropy_bit_vector, vec![1, 1, 1, 0]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector, vec![]);
    }

    #[test]
    fn add_datum_works_in_normal_condition_04() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let datum = NaturalDatum::new(range, 4).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 5).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 3).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 6).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 2).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![4, 5, 3, 6, 2]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![3, 2, 1]);
        assert_eq!(tychentropy.generated_entropy_bits, 6);
        assert_eq!(tychentropy.is_entropy_ready, false);
        assert_eq!(tychentropy.entropy_bit_string, "111001");
        assert_eq!(tychentropy.entropy_bit_vector, vec![1, 1, 1, 0, 0, 1]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector, vec![]);
    }

    #[test]
    fn add_datum_works_in_normal_condition_05() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let datum = NaturalDatum::new(range, 4).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 5).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 3).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 6).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 2).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        let datum = NaturalDatum::new(range, 1).unwrap();
        tychentropy.add_natural_datum(datum).unwrap();

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![4, 5, 3, 6, 2, 1]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![3, 2, 1, 0]);
        assert_eq!(tychentropy.generated_entropy_bits, 8);
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string, "11100100");
        assert_eq!(tychentropy.entropy_bit_vector, vec![1, 1, 1, 0, 0, 1, 0, 0]);
        assert_eq!(tychentropy.entropy_bytes_vector.len(), 1);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 1);
    }

    #[test]
    fn add_datum_works_in_normal_condition_06() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);

        generate_random_data_sequence_till_entropy_is_full(range, &mut tychentropy);

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.entropy_generating_sequence.len(), 4);
        assert_eq!(tychentropy.generated_entropy_bits, 8);
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string.len(), 8);
        assert_eq!(tychentropy.entropy_bit_vector.len(), 8);
        assert_eq!(tychentropy.entropy_bytes_vector.len(), 1);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 1);
    }

    #[test]
    fn add_datum_works_in_normal_condition_07() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let sequence = vec![1, 4, 6, 2, 3, 4, 1, 4, 6, 4, 2];

        generate_pre_determined_series_of_data_till_entropy_is_full(
            range,
            sequence,
            &mut tychentropy,
        );

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![1, 4, 6, 2, 3]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![0, 3, 1, 2]);
        assert_eq!(tychentropy.generated_entropy_bits, 8);
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string, "00110110");
        assert_eq!(tychentropy.entropy_bit_vector, vec![0, 0, 1, 1, 0, 1, 1, 0]);
        assert_eq!(tychentropy.entropy_bytes_vector.len(), 1);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 1);
    }

    #[test]
    fn add_datum_works_in_normal_condition_08() {
        let range = 2048;
        let target_entropy_bytes = 1024;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);

        generate_random_data_sequence_till_entropy_is_full(range, &mut tychentropy);

        assert_eq!(tychentropy.range, 2048);
        assert_eq!(tychentropy.full_bits_in_each_datum, 11);
        assert_eq!(tychentropy.target_entropy_bytes, 1024);
        assert_eq!(tychentropy.target_entropy_bits, 8192);
        assert_eq!(tychentropy.generated_entropy_bits, 8195); // every datum has 11 bits and that's not a multiple of 8. Hence a bit of over doing here.
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string.len(), 8192);
        assert_eq!(tychentropy.entropy_bit_vector.len(), 8192);
        assert_eq!(tychentropy.entropy_bytes_vector.len(), 1024);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 1024);
    }

    #[test]
    fn add_datum_works_in_normal_condition_09() {
        let range = 632;
        let target_entropy_bytes = 893;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);

        generate_random_data_sequence_till_entropy_is_full(range, &mut tychentropy);

        assert_eq!(tychentropy.range, 632);
        assert_eq!(tychentropy.full_bits_in_each_datum, 9);
        assert_eq!(tychentropy.target_entropy_bytes, 893);
        assert_eq!(tychentropy.target_entropy_bits, 7144);
        assert_eq!(tychentropy.generated_entropy_bits, 7146); // every datum has 11 bits and that's not a multiple of 8. Hence a bit of over doing here.
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string.len(), 7144);
        assert_eq!(tychentropy.entropy_bit_vector.len(), 7144);
        assert_eq!(tychentropy.entropy_bytes_vector.len(), 893);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 893);
    }

    #[test]
    fn add_datum_emits_error_for_out_of_range_datum_01() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let datum = NaturalDatum::new(8, 7).unwrap();
        let result = tychentropy.add_natural_datum(datum);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            ProcessError::NaturalDatumOutOfRange { value: 7, range: 6 }
        )
    }

    #[test]
    fn create_entropy_bytes_works_in_normal_condition_01() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let sequence = vec![1, 4, 6, 2, 3, 4, 1, 4, 6, 4, 2];

        generate_pre_determined_series_of_data_till_entropy_is_full(
            range,
            sequence,
            &mut tychentropy,
        );

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![1, 4, 6, 2, 3]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![0, 3, 1, 2]);
        assert_eq!(tychentropy.generated_entropy_bits, 8);
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string, "00110110");
        assert_eq!(tychentropy.entropy_bit_vector, vec![0, 0, 1, 1, 0, 1, 1, 0]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![108]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector, vec![108]);
    }

    #[test]
    fn mix_rng_and_data_entropy_works_in_normal_condition_01() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let sequence = vec![1, 4, 6, 2, 3, 4, 1, 4, 6, 4, 2];

        generate_pre_determined_series_of_data_till_entropy_is_full(
            range,
            sequence,
            &mut tychentropy,
        );

        tychentropy.mix_with_rng().unwrap();

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![1, 4, 6, 2, 3]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![0, 3, 1, 2]);
        assert_eq!(tychentropy.generated_entropy_bits, 8);
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string, "00110110");
        assert_eq!(tychentropy.entropy_bit_vector, vec![0, 0, 1, 1, 0, 1, 1, 0]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![108]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector.len(), 1);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 1);
        assert_ne!(
            tychentropy.entropy_bytes_vector,
            tychentropy.rng_entropy_bytes_vector
        );
        assert_ne!(
            tychentropy.entropy_bytes_vector,
            tychentropy.final_entropy_bytes_vector
        );
        assert_ne!(
            tychentropy.rng_entropy_bytes_vector,
            tychentropy.final_entropy_bytes_vector
        );
    }

    #[test]
    fn mix_rng_and_data_entropy_works_in_normal_condition_02() {
        let range = 1024;
        let target_entropy_bytes = 32;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);

        generate_random_data_sequence_till_entropy_is_full(range, &mut tychentropy);

        tychentropy.mix_with_rng().unwrap();

        assert_eq!(tychentropy.entropy_bytes_vector.len(), 32);
        assert_eq!(tychentropy.rng_entropy_bytes_vector.len(), 32);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 32);
        assert_ne!(
            tychentropy.entropy_bytes_vector,
            tychentropy.rng_entropy_bytes_vector
        );
        assert_ne!(
            tychentropy.entropy_bytes_vector,
            tychentropy.final_entropy_bytes_vector
        );
        assert_ne!(
            tychentropy.rng_entropy_bytes_vector,
            tychentropy.final_entropy_bytes_vector
        );
    }

    #[test]
    fn mix_rng_and_data_entropy_emits_error_when_entropy_is_not_ready_01() {
        let range = 1024;
        let target_entropy_bytes = 32;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);

        let result = tychentropy.mix_with_rng();

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            ProcessError::EntropyBitsAreNotReady {
                required_entropy_bits: 256,
                current_entropy_bits: 0
            }
        )
    }

    #[test]
    fn put_data_from_another_tychentropy_works_01() {
        let range = 1024;
        let target_entropy_bytes = 32;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);

        let range = 32;
        let target_entropy_bytes = 42;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let tychentropy_rep = Tychentropy::new(input);

        tychentropy.put_data_from_another_tychentropy(tychentropy_rep);
        assert_eq!(tychentropy.range, 32);
        assert_eq!(tychentropy.full_bits_in_each_datum, 5);
        assert_eq!(tychentropy.target_entropy_bytes, 42);
        assert_eq!(tychentropy.target_entropy_bits, 336);
    }

    #[test]
    fn put_data_from_another_tychentropy_works_02() {
        let range = 8;
        let target_entropy_bytes = 3;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let sequence = vec![1, 1, 1, 1, 1, 1, 1, 1];

        generate_pre_determined_series_of_data_till_entropy_is_full(
            range,
            sequence,
            &mut tychentropy,
        );

        tychentropy.mix_with_rng().unwrap();

        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy_rep = Tychentropy::new(input);
        let sequence = vec![1, 4, 6, 2, 3, 4, 1, 4, 6, 4, 2];

        generate_pre_determined_series_of_data_till_entropy_is_full(
            range,
            sequence,
            &mut tychentropy_rep,
        );

        tychentropy_rep.mix_with_rng().unwrap();

        tychentropy.put_data_from_another_tychentropy(tychentropy_rep);

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![1, 4, 6, 2, 3]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![0, 3, 1, 2]);
        assert_eq!(tychentropy.generated_entropy_bits, 8);
        assert_eq!(tychentropy.is_entropy_ready, true);
        assert_eq!(tychentropy.entropy_bit_string, "00110110");
        assert_eq!(tychentropy.entropy_bit_vector, vec![0, 0, 1, 1, 0, 1, 1, 0]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![108]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector.len(), 1);
        assert_eq!(tychentropy.final_entropy_bytes_vector.len(), 1);
        assert_ne!(
            tychentropy.entropy_bytes_vector,
            tychentropy.rng_entropy_bytes_vector
        );
        assert_ne!(
            tychentropy.entropy_bytes_vector,
            tychentropy.final_entropy_bytes_vector
        );
        assert_ne!(
            tychentropy.rng_entropy_bytes_vector,
            tychentropy.final_entropy_bytes_vector
        );
    }

    #[test]
    fn reset_works_01() {
        let range = 6;
        let target_entropy_bytes = 1;
        let input = TychentropyNewInput::new(range, target_entropy_bytes).unwrap();
        let mut tychentropy = Tychentropy::new(input);
        let sequence = vec![1, 4, 6, 2, 3, 4, 1, 4, 6, 4, 2];

        generate_pre_determined_series_of_data_till_entropy_is_full(
            range,
            sequence,
            &mut tychentropy,
        );

        tychentropy.mix_with_rng().unwrap();

        tychentropy.reset_data();

        assert_eq!(tychentropy.range, 6);
        assert_eq!(tychentropy.full_bits_in_each_datum, 2);
        assert_eq!(tychentropy.target_entropy_bytes, 1);
        assert_eq!(tychentropy.target_entropy_bits, 8);
        assert_eq!(tychentropy.sequence, vec![]);
        assert_eq!(tychentropy.entropy_generating_sequence, vec![]);
        assert_eq!(tychentropy.generated_entropy_bits, 0);
        assert_eq!(tychentropy.is_entropy_ready, false);
        assert_eq!(tychentropy.entropy_bit_string, "");
        assert_eq!(tychentropy.entropy_bit_vector, vec![]);
        assert_eq!(tychentropy.entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.rng_entropy_bytes_vector, vec![]);
        assert_eq!(tychentropy.final_entropy_bytes_vector, vec![]);
    }
}
