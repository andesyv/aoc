use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;

type Pos = (i32, i32);

fn parse_moves(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            iter::repeat(match &l[..1] {
                "R" => (1, 0),
                "U" => (0, 1),
                "L" => (-1, 0),
                "D" => (0, -1),
                _ => unimplemented!(),
            })
            .take(l[2..].parse().unwrap())
        })
        .flatten()
        .collect()
}

fn is_neighbour(a: Pos, b: Pos) -> bool {
    (a.0 - b.0).abs() < 2 && (a.1 - b.1).abs() < 2
}

fn get_next_non_stationary_pos(head: &Pos, tail: &Pos) -> Pos {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dy == 0 {
        (tail.0 + dx.signum(), tail.1)
    } else if dx == 0 {
        (tail.0, tail.1 + dy.signum())
    } else {
        (tail.0 + dx.signum(), tail.1 + dy.signum())
    }
}

// Gosh this could've been such a pretty function in Haskell...
fn get_next_pos(rope: &Vec<Pos>, dir: Pos) -> Vec<Pos> {
    let head = rope.first().unwrap();
    let mut new_rope = rope.clone();
    *new_rope.first_mut().unwrap() = (head.0 + dir.0, head.1 + dir.1);
    for i in 0..new_rope.len() - 1 {
        if let (Some(local_head), Some(local_tail)) = (new_rope.get(i), new_rope.get(i + 1)) {
            *new_rope.get_mut(i + 1).unwrap() = if is_neighbour(*local_head, *local_tail) {
                *local_tail
            } else {
                get_next_non_stationary_pos(local_head, local_tail)
            }
        }
    }
    new_rope
}

fn debug_pos(rope: &Vec<Pos>) {
    const GRID_SIZE: i32 = 10;
    for j in -GRID_SIZE..=GRID_SIZE {
        let mut line = String::new();
        for i in -GRID_SIZE..=GRID_SIZE {
            if let Some(rope_pos) = rope.iter().position(|p| *p == (i, j)) {
                line += &rope_pos.to_string();
            } else if i == 0 && j == 0 {
                line += "s";
            } else {
                line += ".";
            }
        }
        println!("{}", line);
    }
    println!("\n");
}

fn traverse_tail(input: &str, rope_len: usize) -> Vec<Pos> {
    let mut rope_pos = vec![(0, 0); rope_len];
    parse_moves(input)
        .into_iter()
        .map(|dir| {
            rope_pos = get_next_pos(&rope_pos, dir);
            // debug_pos(&rope_pos);
            *rope_pos.last().unwrap()
        })
        .collect()
}

fn get_unique_tail_position_count(input: &str, rope_len: usize) -> i32 {
    traverse_tail(input, rope_len)
        .into_iter()
        .collect::<HashSet<Pos>>()
        .len()
        .try_into()
        .unwrap()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/9.txt");
    println!(
        "Unique positions visited by rope tail: {}",
        get_unique_tail_position_count(INPUT, 2)
    );

    println!(
        "Unique positions visited by longer rope tail: {}",
        get_unique_tail_position_count(INPUT, 10)
    );
}

const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

const EXAMPLE_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

#[test]
fn test_parse_moves() {
    assert_eq!(parse_moves("L 1\nD 1"), vec![(-1, 0), (0, -1)]);
    assert_eq!(
        parse_moves("R 2\nU 3"),
        vec![(1, 0), (1, 0), (0, 1), (0, 1), (0, 1)]
    );
}

#[test]
fn test_is_neighbour() {
    let mut grid = HashMap::new();
    for i in -2..=2 {
        for j in -2..=2 {
            grid.insert((i, j), false);
        }
    }
    for i in -1..=1 {
        for j in -1..=1 {
            grid.insert((i, j), true);
        }
    }

    for (pos, answer) in grid {
        assert_eq!(is_neighbour(pos, (0, 0)), answer);
    }
}

#[test]
fn example_1() {
    assert_eq!(get_unique_tail_position_count(EXAMPLE_INPUT, 2), 13);
}

#[test]
fn example_2() {
    assert_eq!(get_unique_tail_position_count(EXAMPLE_INPUT, 10), 1);
    assert_eq!(get_unique_tail_position_count(EXAMPLE_INPUT_2, 10), 36);
}
