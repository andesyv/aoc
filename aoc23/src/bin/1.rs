const EXAMPLE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const EXAMPLE_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

fn trim_letters(line: &str) -> &str {
    line.trim_matches(|c: char| !c.is_digit(10))
}

fn number_from_front_and_back(s: &str) -> u32 {
    let number: String = [s.chars().next(), s.chars().last()]
        .into_iter()
        .filter_map(|o| o)
        .collect();
    u32::from_str_radix(number.as_str(), 10).ok().unwrap_or(0)
}

enum FindPosition {
    First,
    Last,
}

fn find_numberish(line: &str, position: FindPosition) -> Option<u32> {
    const VALID_NUMBER_PATTERNS: [&str; 19] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9", "0",
    ];
    let matches = VALID_NUMBER_PATTERNS
        .iter()
        .flat_map(|pattern| line.match_indices(pattern));
    (match position {
        FindPosition::First => matches.min_by_key(|(i, _)| *i),
        FindPosition::Last => matches.max_by_key(|(i, _)| *i),
    })
    .and_then(|(_, word)| {
        Some(match &word[..] {
            "one" | "1" => 1,
            "two" | "2" => 2,
            "three" | "3" => 3,
            "four" | "4" => 4,
            "five" | "5" => 5,
            "six" | "6" => 6,
            "seven" | "7" => 7,
            "eight" | "8" => 8,
            "nine" | "9" => 9,
            _ => 0,
        })
    })
}

fn number_from_front_and_back_2(s: &str) -> u32 {
    let number: String = [
        find_numberish(s, FindPosition::First),
        find_numberish(s, FindPosition::Last),
    ]
    .into_iter()
    .filter_map(|o| o.and_then(|n| char::from_digit(n, 10)))
    .collect();
    u32::from_str_radix(number.as_str(), 10).ok().unwrap_or(0)
}

fn sum_of_ends(input: &str, number_words: bool) -> u32 {
    input
        .lines()
        .map(|line| {
            if number_words {
                number_from_front_and_back_2(line)
            } else {
                number_from_front_and_back(trim_letters(line))
            }
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/1.txt");
    println!("Sum of calibration values: {}", sum_of_ends(INPUT, true));
}

#[test]
fn example_1() {
    assert_eq!(sum_of_ends(EXAMPLE_INPUT, false), 142);
}

#[test]
fn example_2() {
    assert_eq!(sum_of_ends(EXAMPLE_INPUT_2, true), 281);
}
