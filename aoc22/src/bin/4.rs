use itertools::Itertools;

fn parse(input: &str) -> Vec<(std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>)> {
    input
        .lines()
        .filter_map(|l| {
            l.split(',')
                .filter_map(|p| {
                    p.split_once('-')
                        .and_then(|(l, r)| Some(l.parse().ok()?..=r.parse().ok()?))
                })
                .collect_tuple()
        })
        .collect()
}

fn pairs_contained(
    (l, r): &(std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>),
) -> bool {
    r.clone().all(|i| l.contains(&i)) || l.clone().all(|i| r.contains(&i))
}

fn pairs_overlap((l, r): &(std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>)) -> bool {
    r.clone().any(|i| l.contains(&i)) || l.clone().any(|i| r.contains(&i))
}

fn count_contained_pairs(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|p| if pairs_contained(&p) { 1 } else { 0 })
        .sum()
}

fn count_pairs_overlapping(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|p| if pairs_overlap(&p) { 1 } else { 0 })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/4.txt");
    println!(
        "{} assignment pairs fully contains the other",
        count_contained_pairs(INPUT)
    );

    println!(
      "{} overlapping pairs",
      count_pairs_overlapping(INPUT)
  );
}

const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn parse_test() {
    let expected: Vec<(std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>)> = vec![
        (2..=4, 6..=8),
        (2..=3, 4..=5),
        (5..=7, 7..=9),
        (2..=8, 3..=7),
        (6..=6, 4..=6),
        (2..=6, 4..=8),
    ];
    assert_eq!(parse(EXAMPLE_INPUT), expected);
}

#[test]
fn example_1() {
    assert_eq!(count_contained_pairs(EXAMPLE_INPUT), 2);
}

#[test]
fn example_2() {
    assert_eq!(count_pairs_overlapping(EXAMPLE_INPUT), 4);
}
