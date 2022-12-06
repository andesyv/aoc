use itertools::Itertools;

fn find_start_marker<const N: usize>(input: &str) -> Option<usize> {
    input.chars().collect_vec()[..]
        .windows(N)
        .enumerate()
        .find(|(i, pattern)| pattern.into_iter().unique().count() == N)
        .map(|(i, _)| i + N)
}

const find_start_of_packet: fn(&str) -> Option<usize> = find_start_marker::<4>;
const find_start_of_message: fn(&str) -> Option<usize> = find_start_marker::<14>;

fn main() {
    const INPUT: &str = include_str!("../inputs/6.txt");
    println!(
        "The start-of-packet marker is at position {}",
        find_start_of_packet(INPUT).unwrap()
    );

    println!(
        "The start-of-message marker is at position {}",
        find_start_of_message(INPUT).unwrap()
    );
}

const EXAMPLE_INPUTS: [&str; 5] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
];

#[test]
fn example_find_start_of_packets() {
    let results = [7, 5, 6, 10, 11];
    for i in 0..5 {
        assert_eq!(find_start_of_packet(EXAMPLE_INPUTS[i]).unwrap(), results[i]);
    }
}

#[test]
fn example_find_start_of_messages() {
    let results = [19, 23, 23, 29, 26];
    for i in 0..5 {
        assert_eq!(
            find_start_of_message(EXAMPLE_INPUTS[i]).unwrap(),
            results[i]
        );
    }
}
