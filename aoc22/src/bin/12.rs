use std::{collections::HashSet, hash::Hash, iter::Successors, rc::Rc};

use itertools::Itertools;
use pathfinding::prelude::bfs;

type Grid = Vec<Vec<u32>>;
type Pos = (i32, i32);

#[derive(Debug, Clone)]
struct Path {
    steps: Vec<Pos>,
    traversed_positions: HashSet<Pos>,
}

fn find_char_pos(char_grid: &Vec<Vec<char>>, target: char) -> Option<Pos> {
    char_grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .position(|c| *c == target)
                .and_then(|j| Some((i.try_into().ok()?, j.try_into().ok()?)))
        })
        .next()
}

fn find_all_positions<T: std::cmp::PartialEq + std::marker::Copy>(
    grid: &Vec<Vec<T>>,
    target: T,
) -> Vec<Pos> {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, value)| {
                if *value == target {
                    Some((i.try_into().ok()?, j.try_into().ok()?))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

fn parse(input: &str) -> Option<(Grid, Pos, Pos)> {
    let char_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = find_char_pos(&char_grid, 'S')?;
    let end = find_char_pos(&char_grid, 'E')?;

    Some((
        char_grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| {
                        match match c {
                            'S' => 'a',
                            'E' => 'z',
                            _ => c,
                        } {
                            x @ 'a'..='z' => x as u32 - 'a' as u32,
                            _ => unimplemented!(),
                        }
                    })
                    .collect()
            })
            .collect(),
        start,
        end,
    ))
}

fn get_height(grid: &Grid, (x, y): Pos) -> Option<u32> {
    grid.get(usize::try_from(x).ok()?)?
        .get(usize::try_from(y).ok()?)
        .map(|x| *x)
}

fn get_neighbours((x, y): Pos) -> [Pos; 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

#[derive(Clone, Eq)]
struct Node {
    grid: Rc<Grid>,
    pos: Pos,
}

impl Node {
    fn next(&self) -> Vec<Node> {
        let mut potential_paths = Vec::new();
        let current_height = get_height(&self.grid, self.pos).unwrap();
        for next_pos in get_neighbours(self.pos) {
            if let Some(next_height) = get_height(&self.grid, next_pos) {
                if next_height <= current_height || next_height == current_height + 1 {
                    potential_paths.push(Node {
                        grid: Rc::clone(&self.grid),
                        pos: next_pos,
                    });
                }
            }
        }
        potential_paths
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

fn find_shortest_path(grid: Rc<Grid>, start: Pos, goal: Pos) -> Option<Vec<Pos>> {
    let goal = Node {
        grid: Rc::clone(&grid),
        pos: goal,
    };
    let start = Node {
        grid: Rc::clone(&grid),
        pos: start,
    };
    Some(
        bfs(&start, |node| node.next(), |node| *node == goal)?
            .into_iter()
            .map(|node| node.pos)
            .collect(),
    )
}

fn find_shortest_hiking_paths(grid: Rc<Grid>, goal: Pos) -> Vec<Pos> {
    let starting_positions = find_all_positions(&grid, 0);
    starting_positions
        .into_iter()
        .filter_map(|p| find_shortest_path(grid.clone(), p, goal))
        .min_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/12.txt");
    let (grid, start, goal) = parse(INPUT).unwrap();
    let grid = Rc::new(grid);
    let shortest_path = find_shortest_path(grid.clone(), start, goal).unwrap();
    println!(
        "The shortest path to the goal takes {} steps",
        shortest_path.len() - 1
    );

    println!(
        "The shortest hiking path is instead {} steps",
        find_shortest_hiking_paths(grid, goal).len() - 1
    );
}

const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[test]
fn parse_test() {
    let expected_grid = vec![
        vec![0, 0, 1, 16, 15, 14, 13, 12],
        vec![0, 1, 2, 17, 24, 23, 23, 11],
        vec![0, 2, 2, 18, 25, 25, 23, 10],
        vec![0, 2, 2, 19, 20, 21, 22, 9],
        vec![0, 1, 3, 4, 5, 6, 7, 8],
    ];
    let (grid, start, end) = parse(EXAMPLE_INPUT).unwrap();
    assert_eq!(grid, expected_grid);
    assert_eq!(start, (0, 0));
    assert_eq!(end, (2, 5));

    assert_eq!(get_height(&grid, start), Some(0));
    assert_eq!(get_height(&grid, end), Some('z' as u32 - 'a' as u32));
}

#[test]
fn example_1() {
    let (grid, start, end) = parse(EXAMPLE_INPUT).unwrap();
    let grid = Rc::new(grid);
    assert_eq!(find_shortest_path(grid, start, end).unwrap().len(), 32);
}

#[test]
fn example_2() {
    let (grid, _, end) = parse(EXAMPLE_INPUT).unwrap();
    let grid = Rc::new(grid);
    assert_eq!(find_shortest_hiking_paths(grid, end).len(), 30);
}
