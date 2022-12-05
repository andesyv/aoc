use std::collections::HashMap;

use itertools::Itertools;

type Command = (i32, i32, i32);

fn parse(input: &str) -> Option<(Vec<Vec<char>>, Vec<Command>)> {
    input
        .split_once("\n\n")
        .map(|(first, second)| (parse_stacks(first), parse_commands(second)))
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut indexed_crates: HashMap<usize, Vec<char>> = HashMap::new();
    for l in input.lines() {
        for (i, c) in l.chars().enumerate() {
            if let 'A'..='Z' = c {
                if indexed_crates.contains_key(&i) {
                    if let Some(q) = indexed_crates.get_mut(&i) {
                        q.push(c);
                    }
                } else {
                    indexed_crates.insert(i, vec![c]);
                }
            }
        }
    }

    indexed_crates
        .into_iter()
        .sorted()
        .map(|(_, q)| q.into_iter().rev().collect())
        .collect()
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|l| {
            l.split(' ')
                .filter_map(|segment| segment.parse().ok())
                .collect_tuple()
        })
        .collect()
}

fn simulate(mut crates: Vec<Vec<char>>, commands: Vec<Command>) -> String {
    for (mut count, a, b) in commands {
      while 0 < count {
        perform_move(&mut crates, (1, a, b));
        count -= 1;
      }
    }

    crates.iter().map(|q|q.last().unwrap()).collect()
}

fn simulate_2(mut crates: Vec<Vec<char>>, commands: Vec<Command>) -> String {
  for command in commands {
      perform_move(&mut crates, command);
  }

  crates.iter().map(|q|q.last().unwrap()).collect()
}

fn perform_move(crates: &mut Vec<Vec<char>>, (mut count, a, b): Command) {
    let i_a: usize = (a-1).try_into().unwrap();
    let i_b: usize = (b-1).try_into().unwrap();
    let mut tmp = Vec::new();
    while 0 < count {
      if let Some(c) = crates.get_mut(i_a).unwrap().pop() {
          tmp.push(c);
      }
      count -= 1;
    }
    tmp.reverse();
    crates.get_mut(i_b).unwrap().append(&mut tmp);
}

fn main() {
    const INPUT: &str = include_str!("../inputs/5.txt");
    let (crates, commands) = parse(INPUT).unwrap();
    println!("The resulting crate configuration is: {}", simulate(crates, commands));

    let (crates, commands) = parse(INPUT).unwrap();
    println!("Instead using the CrateMover 9001, the crate configuration ends up as: {}", simulate_2(crates, commands));
}

const EXAMPLE_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn parse_test() {
    let expected = (
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
    );

    assert_eq!(parse(EXAMPLE_INPUT).unwrap(), expected);
}

#[test]
fn example_1() {
  let (crates, commands) = parse(EXAMPLE_INPUT).unwrap();
  assert_eq!(simulate(crates, commands), "CMZ");
}

fn example_2() {
  let (crates, commands) = parse(EXAMPLE_INPUT).unwrap();
  assert_eq!(simulate_2(crates, commands), "MCD");
}