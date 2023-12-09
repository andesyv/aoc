use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

fn get_recurring_pattern_of_start_pos(dirs: &str, nodes: &HashMap<String, (String, String)>, pos: &str) -> u64 {
  nodes.iter().map(|(keys, values)|keys.as_str().hash)
  for step in 0.. {
    
  }

  let mut steps: u64 = 0;


  let mut dir_it = dirs.chars().cycle();
  while positions
      .iter()
      .any(|pos| pos.chars().last().unwrap() != 'Z')
  {
      let dir = dir_it.next().unwrap();
      for pos in positions.iter_mut() {
          let (left, right) = nodes.get(pos).unwrap();
          *pos = (if dir == 'L' { left } else { right }).clone();
      }
      steps += 1;
      if steps % 1000000 == 0 {
        println!("Steps: {}", steps);
      }
  }
}

fn get_required_steps_to_get_to_goal_2(input: &str) -> u64 {
    let (dirs, nodes) = parse(input).unwrap();)
    // let mut pos = "AAA";
    let mut positions: Vec<String> = nodes
        .keys()
        .filter(|name| name.chars().last().unwrap() == 'A')
        .cloned()
        .collect();

    let mut steps = 0;
    let mut dir_it = dirs.chars().cycle();
    while positions
        .iter()
        .any(|pos| pos.chars().last().unwrap() != 'Z')
    {
        let dir = dir_it.next().unwrap();
        for pos in positions.iter_mut() {
            let (left, right) = nodes.get(pos).unwrap();
            *pos = (if dir == 'L' { left } else { right }).clone();
        }
        steps += 1;
        if steps % 1000000 == 0 {
          println!("Steps: {}", steps);
        }
    }

    steps
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
