use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn gen_next_secret_number(mut next_number: u64) -> u64 {
    next_number = (next_number ^ (next_number * 64)) % 16777216;
    next_number = (next_number ^ (next_number / 32)) % 16777216;
    next_number = (next_number ^ (next_number * 2048)) % 16777216;
    next_number
}

fn get_nth_secret_number(mut initial_number: u64, n: usize) -> u64 {
    for _ in 0..n {
        initial_number = gen_next_secret_number(initial_number);
    }
    initial_number
}

fn sum_of_2000th_secret_numbers_from_input(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let num = line.parse().unwrap();
            get_nth_secret_number(num, 2000)
        })
        .sum()
}

type ShortSequence = (i16, i16, i16, i16);

fn get_ordered_buying_sequences(mut secret_number: u64) -> HashMap<ShortSequence, i16> {
    let mut buying_sequences = HashMap::with_capacity(2000 - 4);
    let mut diffs = VecDeque::new();
    let mut last_buying_price = (secret_number % 10) as i16;
    for _ in 0..2000 {
        let new_secret_number = gen_next_secret_number(secret_number);
        let buying_price = (secret_number % 10) as i16;
        let diff = buying_price - last_buying_price;
        diffs.push_back(diff);
        if diffs.len() > 4 {
            diffs.pop_front();
        }

        if let Some(diff_sequence) = diffs.iter().copied().collect_tuple::<ShortSequence>() {
            if !buying_sequences.contains_key(&diff_sequence) {
                buying_sequences.insert(diff_sequence, buying_price);
            }
        }

        last_buying_price = buying_price;
        secret_number = new_secret_number;
    }

    buying_sequences
}

fn get_all_ordered_buying_sequences(input: &str) -> Vec<HashMap<ShortSequence, i16>> {
    input
        .trim()
        .lines()
        .map(|line| get_ordered_buying_sequences(line.parse().unwrap()))
        .collect()
}

fn find_the_most_bananas_possible_to_get(input: &str) -> u32 {
    let buying_sequences = get_all_ordered_buying_sequences(input);
    let all_sequences: HashSet<_> = buying_sequences
        .iter()
        .map(|set| set.iter().map(|(seq, _)| *seq))
        .flatten()
        .collect();

    println!("Preprocessing done");

    let mut possible_max = 0;
    for sequence in all_sequences {
        let mut current = 0;
        for buyer_sequences in &buying_sequences {
            if let Some(buyer_price) = buyer_sequences.get(&sequence) {
                current += *buyer_price as u32;
            }
        }

        if current > possible_max {
            // println!("New best sequence: {:?}", sequence);
            possible_max = current;
        }
    }
    possible_max
}

fn main() {
    const INPUT: &str = include_str!("../inputs/22.txt");
    println!(
        "Sum of 2000th secret number: {}",
        sum_of_2000th_secret_numbers_from_input(INPUT)
    );

    println!("Best possible bananas: {}", find_the_most_bananas_possible_to_get(INPUT));
}

const EXAMPLE_INPUT_1: &str = "1
10
100
2024";

const EXAMPLE_INPUT_2: &str = "1
2
3
2024";

#[test]
fn gen_next_secret_test() {
    let sut = gen_next_secret_number(123);
    assert_eq!(sut, 15887950);

    let sut = gen_next_secret_number(sut);
    assert_eq!(sut, 16495136);

    let sut = gen_next_secret_number(sut);
    assert_eq!(sut, 527345);
}

#[test]
fn get_sum_of_2000th_secrets_test() {
    let sut = sum_of_2000th_secret_numbers_from_input(EXAMPLE_INPUT_1);
    assert_eq!(sut, 37327623);
}

#[test]
fn get_possible_bananas_for_example() {
    let result = find_the_most_bananas_possible_to_get(&EXAMPLE_INPUT_2);
    assert_eq!(result, 23);
}
