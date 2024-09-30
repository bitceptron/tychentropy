use tychentropy::{domain::process::input::TychentropyNewInput, utils::test_utils::generate_pre_determined_series_of_data_till_entropy_is_full, Tychentropy};

    fn main() {
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
    }