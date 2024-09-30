use std::fmt::Display;

use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum ProcessError {
    EntropyAlreadyCreated,
    NoSequenceAppendixFound,
    EntropyBitsAreNotReady {
        required_entropy_bits: u64,
        current_entropy_bits: u64,
    },
    ProvidedEntropyLengthDoesNotMatchTarget {
        provided_len: u64,
        target_len: u64,
    },
    ProvidedRngEntropyLengthDoesNotMatchTarget {
        provided_len: u64,
        target_len: u64,
    },
    MaxInclusiveLessThanTwo,
    NaturalDatumOutOfRange {
        value: u64,
        range: u64,
    },
    EntropyBitVecLenDoesNotMatchTarget {
        provided_bit_len: u64,
        target_bit_len: u64,
    },
    RangeCountIsLessThanTwo,
    TargetEntropyBytesAreLessThanOne,
    RngEntropyBytesHaveNotBeenGeneratedYet,
    EntropyGeneratingAppendageIsNoneWhileItsDerivativesAreSome,
    EntropyGeneratingAppendageIsSomeWhileItsDerivativesAreNone,
    EntropyGeneratingAppendageIsNotRightComparedToSequenceAppendix {
        sequence_appendage: u64,
        entropy_generating_sequence_appendage: u64,
    },
    MixedEntropyAlreadyCreated,
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::EntropyAlreadyCreated => {
                write!(f, "Entropy has been generated. No need to add more data.")
            }
            ProcessError::NoSequenceAppendixFound => write!(f, "No sequence appendix was found."),
            ProcessError::EntropyBitsAreNotReady {
                required_entropy_bits,
                current_entropy_bits,
            } => write!(
                f,
                "{} bits of entropy is required, while only {} bits have been provided so far.",
                required_entropy_bits, current_entropy_bits
            ),
            ProcessError::ProvidedEntropyLengthDoesNotMatchTarget {
                provided_len,
                target_len,
            } => write!(
                f,
                "Provide entropy is {} bytes long, while the target is {} bytes.",
                provided_len, target_len
            ),
            ProcessError::ProvidedRngEntropyLengthDoesNotMatchTarget { provided_len, target_len } => write!(
                f,
                "Provide rng entropy is {} bytes long, while the target is {} bytes.",
                provided_len, target_len
            ),

            ProcessError::NaturalDatumOutOfRange { value: datum, range: max_inclusive } => write!(f, "Datum must be between 1 and {} (inclusive). The number entered was {}, which is out of range.", max_inclusive, datum),
            ProcessError::MaxInclusiveLessThanTwo => write!(f, "Max inclusive limit must be more than 2."),
            ProcessError::EntropyBitVecLenDoesNotMatchTarget { provided_bit_len: provided_len, target_bit_len: target_len } => write!(f, "Bit vector provided contains {} bits, while the target entropy requires {} bits.", provided_len, target_len),
            ProcessError::RangeCountIsLessThanTwo => write!(
                f,
                "Range must have at least two members to generate entropy."
            ),
            ProcessError::TargetEntropyBytesAreLessThanOne => {
                write!(f, "At least 1 byte is required to generate entropy.")
            }
            ProcessError::RngEntropyBytesHaveNotBeenGeneratedYet => write!(f, "Rng generated entropy has not been created yet. To mix, you have to generate the rng entropy first."),
            ProcessError::EntropyGeneratingAppendageIsNoneWhileItsDerivativesAreSome => write!(f, "Data is not used to create entropy. But there are Options with Some() value that suggest otherwise."),
            ProcessError::EntropyGeneratingAppendageIsSomeWhileItsDerivativesAreNone => write!(f, "Data is used to create entropy. But there are Options with None value that suggest otherwise."),
            ProcessError::EntropyGeneratingAppendageIsNotRightComparedToSequenceAppendix { sequence_appendage: sequence_appendix, entropy_generating_sequence_appendage: entropy_generating_sequence_appendix } => write!(f, "Sequence appendix is {}. Entropy generating appendix must be {}, but is {}.", sequence_appendix, sequence_appendix - 1, entropy_generating_sequence_appendix),
            ProcessError::MixedEntropyAlreadyCreated => write!(f, "Mixed entropy has been generated. No need to remix."),
        }
    }
}

impl From<ProcessError> for Error {
    fn from(value: ProcessError) -> Self {
        Error::ProcessError(value)
    }
}