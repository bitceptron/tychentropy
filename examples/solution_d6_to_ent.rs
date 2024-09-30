use bitvec::{order::Lsb0, vec::BitVec};
use core::f64;
use rand::Rng;

/// Here we want to devise a reliable solution to get a uniformly distributed set of random
/// bits from a preset number of 6-sided dice rolls.

fn main() {
    let mut rng = rand::thread_rng();

    let target_entropy_bits = 160u32;
    let number_of_dice_sides = 6u64;
    let easy_entropy_in_each_roll_bits = number_of_dice_sides.ilog2();
    let num_target_usable_rolls =
        (target_entropy_bits as f64 / easy_entropy_in_each_roll_bits as f64).ceil() as u64;
    let cut_off_for_usable_roll = 2u64.pow(easy_entropy_in_each_roll_bits);

    let mut number_of_rolls = 0u64;
    let mut actual_rolls_container = vec![];
    let mut usable_rolls_container = vec![];

    while num_target_usable_rolls != usable_rolls_container.len() as u64 {
        let roll = rng.gen_range(0..number_of_dice_sides);
        actual_rolls_container.push(roll);
        number_of_rolls += 1;
        if roll < cut_off_for_usable_roll {
            usable_rolls_container.push(roll)
        }
    }

    println!("\nDice sides:                     {}", number_of_dice_sides);
    println!("Target entropy bits:             {}", target_entropy_bits);
    println!(
        "Easy entropy in each roll:       {}",
        easy_entropy_in_each_roll_bits
    );
    println!(
        "Cutoff for usable rolls:         {}",
        cut_off_for_usable_roll
    );
    println!(
        "Number of target usable rolls:   {}",
        num_target_usable_rolls
    );
    println!("Number of actual rolls:          {}", number_of_rolls);

    println!("\nActual rolls:\n{:?}", actual_rolls_container);
    println!("\nUsable rolls:\n{:?}", usable_rolls_container);

    let mut bitstring_vec = vec![];
    for roll in usable_rolls_container {
        let mut bitstring = format!("{:b}", roll);
        if bitstring.len() != easy_entropy_in_each_roll_bits as usize {
            let num_trailing_zeros = easy_entropy_in_each_roll_bits as usize - bitstring.len();
            let mut trailing_zeros = "0".repeat(num_trailing_zeros);
            trailing_zeros.push_str(&bitstring);
            bitstring = trailing_zeros;
        }
        bitstring_vec.push(bitstring)
    }

    let bitstring_joined = bitstring_vec.join("");
    println!("\nEntropy bits:\n{}", bitstring_joined);

    let bit_vector = bitstring_joined
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let byte_vector = bit_vector
        .chunks(8)
        .map(|chu| {
            chu.iter()
                .enumerate()
                .fold(0, |acc, (i, c)| acc + c * 2u8.pow(i as u32))
        })
        .collect::<Vec<u8>>();
    println!("\nEntropy byte vector:\n{:?}", byte_vector);

    let bitvec: BitVec<u8, Lsb0> = BitVec::from_vec(byte_vector);
    println!("\nBitVec of byte vector:\n{:?}", bitvec);
}
