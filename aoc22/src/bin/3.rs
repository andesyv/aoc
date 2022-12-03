use std::collections::HashSet;

fn parse_rucksacks_into_compartments(input: &str) -> Vec<(HashSet<char>, HashSet<char>)> {
    input
        .lines()
        .map(|l| {
            let split_index = l.len() / 2;
            (
                l.chars().take(split_index).collect(),
                l.chars().skip(split_index).collect(),
            )
        })
        .collect()
}

fn parse_rucksacks_into_singular_set(input: &str) -> Vec<HashSet<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn map_priority(c: char) -> u64 {
    let c_digit = u8::try_from(c).unwrap();
    match c {
        'a'..='z' => c_digit - u8::try_from('a').unwrap() + 1,
        'A'..='Z' => c_digit - u8::try_from('A').unwrap() + 27,
        _ => unreachable!(),
    }
    .into()
}

fn find_common_item_type((l, r): &(HashSet<char>, HashSet<char>)) -> Vec<char> {
    let mut priorities = Vec::new();
    for c in r {
        if l.contains(c) {
            priorities.push(*c);
        }
    }
    if priorities.is_empty() {
        panic!("There should be at least 1 shared item in every rucksack!");
    }
    priorities
}

fn sum_of_priorities_for_all_rucksacks(input: &str) -> u64 {
    parse_rucksacks_into_compartments(input)
        .into_iter()
        .map(|p| {
            find_common_item_type(&p)
                .into_iter()
                .map(|item| map_priority(item))
                .sum::<u64>()
        })
        .sum()
}

fn find_common_badge(rucksacks: &[HashSet<char>]) -> Option<char> {
    if let [a, b, c] = rucksacks {
        for item in a {
            if b.contains(item) && c.contains(item) {
                return Some(*item);
            }
        }
    }
    None
}

fn sum_of_common_badges(input: &str) -> u64 {
    parse_rucksacks_into_singular_set(input)
        .chunks(3)
        .map(|group| map_priority(find_common_badge(group).unwrap()))
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/3.txt");
    println!(
        "The sum of priorities for the common item types is {}",
        sum_of_priorities_for_all_rucksacks(INPUT)
    );

    println!(
        "The sum of the badges' priorities for all groups {}",
        sum_of_common_badges(INPUT)
    );
}

const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn parse_test() {
    let expected: Vec<(HashSet<char>, HashSet<char>)> = vec![
        ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
        ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
        ("PmmdzqPrV", "vPwwTWBwg"),
        ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
        ("ttgJtRGJ", "QctTZtZT"),
        ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
    ]
    .into_iter()
    .map(|(l, r)| (l.chars().collect(), r.chars().collect()))
    .collect();

    assert_eq!(parse_rucksacks_into_compartments(EXAMPLE_INPUT), expected);
}

#[test]
fn example_1() {
    assert_eq!(sum_of_priorities_for_all_rucksacks(EXAMPLE_INPUT), 157);
}

#[test]
fn example_2() {
    assert_eq!(sum_of_common_badges(EXAMPLE_INPUT), 70);
}
