use itertools::Itertools;
use std::collections::HashMap;

trait Die {
    fn roll(&mut self) -> u32;
}

struct DeterministicDie {
    counter: u32,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        DeterministicDie { counter: 1 }
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> u32 {
        let ret = (self.counter - 1) % 100 + 1;
        self.counter += 1;
        ret
    }
}

struct DiracDie {
    // let mut rng = rand::thread_rng();
}

// impl Die for DiracDie {
//     fn roll(&mut self) -> u32 {
//         self.rng.gen() % 3 + 1
//     }
// }

impl DiracDie {
    fn get_distinct_permutations() -> Vec<([u32; 3], u32)> {
        let mut sums = HashMap::new();
        let mut outs = Vec::new();
        let permutations = DiracDie::get_permutations();
        for roll in permutations {
            let sum = roll[0] + roll[1] + roll[2];
            if !sums.contains_key(&sum) {
                sums.insert(sum, 1);
                outs.push(roll);
            } else {
                *sums.get_mut(&sum).unwrap() += 1;
            }
        }

        outs.iter()
            .map(|roll| (*roll, *sums.get(&(roll[0] + roll[1] + roll[2])).unwrap()))
            .collect()
    }

    fn get_permutations() -> Vec<[u32; 3]> {
        let mut outs = Vec::new();
        for i in 1..4 {
            for j in 1..4 {
                for k in 1..4 {
                    outs.push([i, j, k]);
                }
            }
        }
        outs
    }
}

fn main() {
    // const INPUT: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
    const INPUT: &str = include_str!("../inputs/21.txt");
    let ((_, pos1), (_, pos2)): ((u32, u32), (u32, u32)) = INPUT
        .lines()
        .map(|s| {
            s.matches(char::is_numeric)
                .filter_map(|p| p.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let mut die = DeterministicDie::new();
    let final_scores = play([pos1, pos2], &mut die);
    let (winner, score) = determine_winner(&final_scores).unwrap();
    println!("The winner is {}, which won with {} points.", winner, score);
    let looser_score = if final_scores[0] == score {
        final_scores[1]
    } else {
        final_scores[0]
    };
    println!(
        "Which means the looser times the dice roll ended up as: {} * {} = {}",
        looser_score,
        die.counter - 1,
        looser_score * (die.counter - 1)
    );

    let dirac_die_combinations = DiracDie::get_distinct_permutations();
    let win_counts = count_win_permutations([pos1, pos2], [0, 0], &dirac_die_combinations, 0, 21);
    println!(
        "Player 1 won {} times, and player 2 won {} times",
        win_counts[0], win_counts[1]
    );
}

fn make_move(rolls: [u32; 3], pos: u32) -> u32 {
    let movement: u32 = rolls.iter().sum();
    (pos - 1 + movement) % 10 + 1
}

fn determine_winner(scores: &[u32; 2]) -> Option<(&'static str, u32)> {
    if 1000 <= scores[0] {
        Some(("Player 1", scores[0]))
    } else if 1000 <= scores[1] {
        Some(("Player 2", scores[1]))
    } else {
        None
    }
}

fn play(mut pos: [u32; 2], die: &mut impl Die) -> [u32; 2] {
    let mut scores = [0; 2];
    let mut current_player = 0;
    loop {
        pos[current_player] = make_move([die.roll(), die.roll(), die.roll()], pos[current_player]);
        scores[current_player] += pos[current_player];
        if 1000 <= scores[current_player] {
            return scores;
        }
        current_player = if current_player == 0 { 1 } else { 0 };
    }
}

fn count_win_permutations(
    pos: [u32; 2],
    scores: [u32; 2],
    possible_moves: &Vec<([u32; 3], u32)>,
    current_player: usize,
    winning_score: u32,
) -> [u128; 2] {
    let mut wins = [0, 0];
    for (roll, permutations) in possible_moves {
        let mut new_pos = pos;
        let mut new_scores = scores;
        let permutations = u128::from(*permutations);
        new_pos[current_player] = make_move(*roll, pos[current_player]);
        new_scores[current_player] += new_pos[current_player];
        if winning_score <= new_scores[current_player] {
            wins[current_player] += permutations;
        } else {
            let new_wins = count_win_permutations(
                new_pos,
                new_scores,
                possible_moves,
                if current_player == 0 { 1 } else { 0 },
                winning_score,
            );
            wins[0] += permutations * new_wins[0];
            wins[1] += permutations * new_wins[1];
        }
    }
    wins
}

#[test]
fn test_move() {
    assert_eq!(make_move([1, 2, 1], 6), 10);
    assert_eq!(make_move([1, 2, 2], 6), 1);
}

#[test]
fn test_deterministic_die() {
    let mut die = DeterministicDie { counter: 100 };
    assert_eq!(die.roll(), 100);
    assert_eq!(die.roll(), 1);
}

#[test]
fn test_deterministic_game() {
    const INPUT: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
    let ((_, pos1), (_, pos2)): ((u32, u32), (u32, u32)) = INPUT
        .lines()
        .map(|s| {
            s.matches(char::is_numeric)
                .filter_map(|p| p.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let mut die = DeterministicDie::new();
    let final_scores = play([pos1, pos2], &mut die);
    let looser_score = final_scores.iter().min().unwrap();
    assert_eq!(looser_score * (die.counter - 1), 739785);
}

#[test]
fn test_dirac_die_permutations() {
    assert_eq!(DiracDie::get_permutations().iter().count(), 3*3*3);
}

#[test]
fn test_dirac_die_outcomes() {
    const INPUT: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
    let ((_, pos1), (_, pos2)): ((u32, u32), (u32, u32)) = INPUT
        .lines()
        .map(|s| {
            s.matches(char::is_numeric)
                .filter_map(|p| p.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
        
    let dirac_die_combinations = DiracDie::get_distinct_permutations();
    let win_counts = count_win_permutations([pos1, pos2], [0, 0], &dirac_die_combinations, 0, 21);

    assert_eq!(win_counts, [444356092776315, 341960390180808]);
}