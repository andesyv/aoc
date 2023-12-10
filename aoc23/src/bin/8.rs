use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use num::integer::lcm;
use regex::Regex;

const EXAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const EXAMPLE_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const EXAMPLE_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

fn parse(input: &str) -> Option<(&str, HashMap<String, (String, String)>)> {
    if let Some((dirs, nodes)) = input.split_once("\n\n") {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

        Some((
            dirs,
            re.captures_iter(nodes)
                .map(|cap| (cap[1].to_string(), (cap[2].to_string(), cap[3].to_string())))
                .collect(),
        ))
    } else {
        None
    }
}

// fn iterate_step<I>(
//     pos: &str,
//     mut directions: I,
//     nodes: &HashMap<String, (String, String)>,
//     step: u64,
// ) -> u64
// where
//     I: Iterator<Item = char>,
// {
//     if pos == "ZZZ" {
//         step
//     } else {
//         let (left, right) = nodes.get(pos).unwrap();
//         let dir = directions.next().unwrap();
//         iterate_step(
//             if dir == 'L' { left } else { right },
//             directions,
//             nodes,
//             step,
//         )
//     }
// }

fn get_required_steps_to_get_to_goal(input: &str) -> u64 {
    let (dirs, nodes) = parse(input).unwrap();
    let mut pos = "AAA";
    let mut steps = 0;
    let mut dir_it = dirs.chars().cycle();
    while pos != "ZZZ" {
        let (left, right) = nodes.get(pos).unwrap();
        let dir = dir_it.next().unwrap();
        pos = if dir == 'L' { left } else { right };
        steps += 1;
    }
    steps
}

fn calc_hash<T: Hash + ?Sized>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// It seems the task was nice enough to choose a set of direction steps such that the recurring pattern never changes (which is not a given I think?)
// We thus only have to find the first pattern
fn get_recurring_pattern_step_size_of_start_pos(
    dirs: &str,
    nodes: &HashMap<String, (String, String)>,
    start_pos: &str,
) -> u64 {
    let hashed_nodes: HashMap<u64, (u64, u64)> = nodes
        .iter()
        .map(|(key, (left, right))| (calc_hash(key), (calc_hash(left), calc_hash(right))))
        .collect();
    let mut pos = calc_hash(start_pos);
    let valid_end_positions: HashSet<_> = nodes
        .keys()
        .filter(|key| key.chars().last().unwrap() == 'Z')
        .map(calc_hash)
        .collect();
    let dir_list: Vec<_> = dirs.chars().collect();

    // let mut recurring_patterns: Vec<u64> = Vec::new();

    // let mut pattern_counter = 0;

    for step in 0u64.. {
        if valid_end_positions.contains(&pos) {
            return step;
            // let previous_pattern_steps: u64 = recurring_patterns.iter().sum();
            // let step_diff = step - previous_pattern_steps;
            // recurring_patterns.push(step_diff);
            // pattern_counter += 1;
            // if pattern_counter == 30 {
            //     return recurring_patterns;
            // }
            // if previous_pattern_steps == step_diff {
            //     return recurring_patterns;
            // }
        }
        let dir = dir_list[(step % dir_list.len() as u64) as usize];
        let (left, right) = hashed_nodes.get(&pos).unwrap();
        pos = if dir == 'L' { *left } else { *right };
    }
    // recurring_patterns
    0
}

fn get_required_steps_to_get_to_goal_2(input: &str) -> u128 {
    let (dirs, nodes) = parse(input).unwrap();
    // let mut pos = "AAA";
    let positions: Vec<String> = nodes
        .keys()
        .filter(|name| name.chars().last().unwrap() == 'A')
        .cloned()
        .collect();

    positions
        .iter()
        .map(|start_pos| {
            u128::from(get_recurring_pattern_step_size_of_start_pos(
                dirs, &nodes, &start_pos,
            ))
        })
        .reduce(|acc, x| lcm(acc, x))
        .unwrap_or(0)

    // let mut steps = 0;
    // let mut dir_it = dirs.chars().cycle();
    // while positions
    //     .iter()
    //     .any(|pos| pos.chars().last().unwrap() != 'Z')
    // {
    //     let dir = dir_it.next().unwrap();
    //     for pos in positions.iter_mut() {
    //         let (left, right) = nodes.get(pos).unwrap();
    //         *pos = (if dir == 'L' { left } else { right }).clone();
    //     }
    //     steps += 1;
    //     if steps % 1000000 == 0 {
    //         println!("Steps: {}", steps);
    //     }
    // }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/8.txt");

    println!(
        "Steps to get to goal: {}",
        get_required_steps_to_get_to_goal(INPUT)
    );

    println!(
        "Steps to get to goal (part 2): {}",
        get_required_steps_to_get_to_goal_2(INPUT)
    );
}

#[test]
fn example_1() {
    assert_eq!(get_required_steps_to_get_to_goal(EXAMPLE_INPUT), 2);
}

#[test]
fn example_2() {
    assert_eq!(get_required_steps_to_get_to_goal(EXAMPLE_INPUT_2), 6);
}

#[test]
fn example_3() {
    assert_eq!(get_required_steps_to_get_to_goal_2(EXAMPLE_INPUT_3), 6);
}
