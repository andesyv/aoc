use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper,
    Scissors,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Move::Rock, Move::Scissors) => std::cmp::Ordering::Greater,
            (Move::Scissors, Move::Rock) => std::cmp::Ordering::Less,
            (l, r) => (*l as u8).cmp(&(*r as u8)),
        }
    }
}

fn parse_rounds(input: &str) -> Vec<(Move, Move)> {
    use Move::*;
    input
        .lines()
        .filter_map(|l| {
            l.chars()
                .filter_map(|c| match c {
                    'A' | 'X' => Some(Rock),
                    'B' | 'Y' => Some(Paper),
                    'C' | 'Z' => Some(Scissors),
                    _ => None,
                })
                .next_tuple()
        })
        .collect()
}

fn parse_rounds_2(input: &str) -> Vec<(Move, u64)> {
    use Move::*;
    parse_rounds(input)
        .into_iter()
        .map(|(l, r)| {
            (
                l,
                match r {
                    Rock => 0,
                    Paper => 3,
                    Scissors => 6,
                },
            )
        })
        .collect()
}

fn get_victory_score(opponent: Move, you: Move) -> u64 {
    if opponent < you {
        6
    } else if opponent == you {
        3
    } else {
        0
    }
}

fn determine_move_for_outcome(opponent: Move, outcome: u64) -> Move {
    for m in [Move::Rock, Move::Paper, Move::Scissors] {
        if get_victory_score(opponent, m) == outcome {
            return m;
        }
    }
    panic!("At the very least one move should reach the given outcome!");
}

fn get_round_points(opponent: Move, you: Move) -> u64 {
    you as u64 + get_victory_score(opponent, you)
}

fn get_score_for_plan(input: &str) -> u64 {
    parse_rounds(input)
        .into_iter()
        .map(|(l, r)| get_round_points(l, r))
        .sum()
}

fn get_score_for_plan_2(input: &str) -> u64 {
    parse_rounds_2(input)
        .into_iter()
        .map(|(opponent, outcome)| {
            get_round_points(opponent, determine_move_for_outcome(opponent, outcome))
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/2.txt");
    println!(
        "The plan would give you a total score of {}",
        get_score_for_plan(INPUT)
    );

    println!(
        "With these new instructions, the plan would instead give you a total score of {}",
        get_score_for_plan_2(INPUT)
    );
}

const EXAMPLE_INPUT: &str = "A Y
B X
C Z";

#[test]
fn parse_rounds_test() {
    use Move::*;
    let expected = vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)];
    assert_eq!(parse_rounds(EXAMPLE_INPUT), expected);
}

#[test]
fn move_cmp_test() {
    use Move::*;
    assert!(Rock < Paper);
    assert!(Paper < Scissors);
    assert!(Scissors < Rock);
    assert!(Paper > Rock);
    assert!(Scissors > Paper);
    assert!(Rock > Scissors);
}

#[test]
fn example_1() {
    assert_eq!(get_score_for_plan(EXAMPLE_INPUT), 15);
}

#[test]
fn example_2() {
    assert_eq!(get_score_for_plan_2(EXAMPLE_INPUT), 12);
}
