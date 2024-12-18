use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

type Pos = (u32, u32);

fn manhattan_distance(a: Pos, b: Pos) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

// Taken directly from Rust's BinaryHeap example :)
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct PosHeapEntry {
    weight: u32,
    cost: u32,
    pos: Pos,
}

impl Ord for PosHeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flipping comparison order turns the heap from a "max" heap to a "min" heap
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.cost.cmp(&other.cost))
    }
}

impl PartialOrd for PosHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    width: u32,
    height: u32,
    corrupted_spaces: HashSet<Pos>,
}

impl Grid {
    fn new_for_example() -> Self {
        Self {
            width: 7,
            height: 7,
            corrupted_spaces: HashSet::new(),
        }
    }

    fn new() -> Self {
        Self {
            width: 71,
            height: 71,
            corrupted_spaces: HashSet::new(),
        }
    }

    fn simulate_bytes(&mut self, bytes: &[Pos]) {
        for byte in bytes {
            self.corrupted_spaces.insert(*byte);
        }
    }

    fn is_within_grid(&self, pos: &Pos) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    fn get_surrounding_positions(&self, current: Pos) -> Vec<Pos> {
        let candidates = [
            (current.0 + 1, current.1),             // [1, 0]
            (current.0, current.1 + 1),             // [0, 1]
            (current.0.wrapping_sub(1), current.1), // [-1, 0]
            (current.0, current.1.wrapping_sub(1)), // [0, -1]
        ];

        candidates
            .into_iter()
            .filter(|pos| self.is_within_grid(pos) && !self.corrupted_spaces.contains(pos))
            .collect()
    }

    fn find_cost_of_escape_route(&self) -> u32 {
        let goal = (self.width - 1, self.height - 1);

        let mut processed = HashSet::new();
        let mut to_process = BinaryHeap::from([PosHeapEntry {
            weight: 0,
            cost: 0,
            pos: (0, 0),
        }]);
        while let Some(PosHeapEntry { cost, pos, .. }) = to_process.pop() {
            if pos == goal {
                return cost;
            }

            for next_position in self.get_surrounding_positions(pos) {
                if processed.contains(&next_position) {
                    continue;
                }

                // Using the "manhattan distance" as a weight to the classic Dijkstra algorithm to
                // turn it into an A* one instead.
                to_process.push(PosHeapEntry {
                    weight: cost + 1 + manhattan_distance(next_position, goal),
                    cost: cost + 1,
                    pos: next_position,
                });
            }
            processed.insert(pos);
        }

        u32::MAX
    }
}

fn parse(input: &str) -> Vec<Pos> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            if let Some((x, y)) = line.split_once(',') {
                let x = x.parse().ok()?;
                let y = y.parse().ok()?;
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/18.txt");

    let mut grid = Grid::new();
    grid.simulate_bytes(&parse(INPUT)[..1024]);

    println!(
        "Cost of escape route after simulating 1024 bytes: {}",
        grid.find_cost_of_escape_route()
    );
}

const EXAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn cost_of_escape_route_for_example() {
    let mut grid = Grid::new_for_example();
    grid.simulate_bytes(&parse(EXAMPLE_INPUT)[..12]);
    assert_eq!(grid.find_cost_of_escape_route(), 22);
}
