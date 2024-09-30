use rand::Rng;

use crate::domain::{process::input::natural_datum::NaturalDatum, tychentropy::Tychentropy};

pub fn generate_random_datum(sides: u64) -> NaturalDatum {
    let mut rng = rand::thread_rng();
    NaturalDatum::new(sides, rng.gen_range(1..sides + 1)).unwrap()
}

pub fn generate_pre_determined_datum(sides: u64, roll: u64) -> NaturalDatum {
    NaturalDatum::new(sides, roll).unwrap()
}

pub fn generate_random_data_sequence_till_entropy_is_full(
    range: u64,
    tychentropy: &mut Tychentropy,
) {
    loop {
        let datum = generate_random_datum(range);
        tychentropy.add_natural_datum(datum).unwrap();
        if *tychentropy.get_is_entropy_ready() {
            break;
        }
    }
}

pub fn generate_pre_determined_series_of_data_till_entropy_is_full(
    range: u64,
    sequence: Vec<u64>,
    tychentropy: &mut Tychentropy,
) {
    let mut i = 0;
    while i < sequence.len() {
        let datum = generate_pre_determined_datum(range, sequence[i]);
        tychentropy.add_natural_datum(datum).unwrap();
        if *tychentropy.get_is_entropy_ready() {
            break;
        }
        i += 1;
    }
}
