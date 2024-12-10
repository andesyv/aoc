// Classic graph task. S a possible solution would be to use the "petgraph" crate and calculate
// a djikstra / min spanning tree for a directed graph. But it's more fun to do it myself. So for
// the time being we'll implement djikstra manually.

use std::collections::HashSet;

struct Map<'a> {
    chars: &'a [u8],
    width: usize,
    height: usize,
}

type Pos = (usize, usize);

impl <'a> Map<'a> {
    fn new(chars: &'a str) -> Option<Map<'a>> {
        let trimmed = chars.trim();
        let width = trimmed.lines().next()?.len();
        let height = trimmed.lines().count();
        Some(Map {
            chars: trimmed.as_bytes(),
            width,
            height,
        })
    }

    fn get_char_as_num(&self, pos: Pos) -> Option<u8> {
        if self.width <= pos.0 || self.height <= pos.1 {
            return None;
        }

        let i = pos.1 * (self.width + 1) + pos.0;
        self.chars.get(i).map(|&c| c - '0' as u8)
    }

    fn find_all_of_char(&self, char: u8) -> Vec<Pos> {
        let mut positions = Vec::new();

        for (y, line) in self.chars.split(|c| c == &('\n' as u8)).enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c == &char {
                    positions.push((x, y));
                }
            }
        }

        positions
    }

    fn find_trailheads(&self) -> Vec<Pos> {
        self.find_all_of_char('0' as u8)
    }

    fn find_tops(&self) -> Vec<Pos> {
        self.find_all_of_char('9' as u8)
    }
}

fn get_surrounding_positions(current: Pos) -> Vec<Pos> {
    vec![
        (current.0 + 1, current.1), // [1, 0]
        (current.0, current.1 + 1), // [0, 1]
        (current.0.wrapping_sub(1), current.1), // [-1, 0]
        (current.0, current.1.wrapping_sub(1)), // [0, -1]
    ]
}

fn find_number_of_tops_reachable_from_trailhead(map: &Map, trailhead: Pos, mut tops_left: HashSet<Pos>) -> u32 {
    let mut count = 0;

    // Simple djikstra with stack
    let mut discovered_positions = HashSet::new();
    let mut next_positions = vec![(trailhead, 0)];
    while let Some((current_pos, current_cost)) = next_positions.pop() {
        if tops_left.remove(&current_pos) {
            count += 1;

            // We can short-circuit when we've found the last top
            if tops_left.is_empty() {
                return count;
            }
        }


        for new_pos in get_surrounding_positions(current_pos) {
            if discovered_positions.contains(&new_pos) { continue; }

            if let Some(cost) = map.get_char_as_num(new_pos) {
                // each reachable node has an exact increase of 1
                if cost.wrapping_sub(current_cost) == 1 {
                    next_positions.push((new_pos, current_cost + 1));
                }
            }
        }

        // Note to self: discovered_positions does nothing in this algorithm, because we can only
        // ever reach nodes that are 1 higher. So it's impossible to loop either way.
        discovered_positions.insert(current_pos);
    }

    count
}

fn find_score_of_all_trailheads(input: &str) -> Option<u32> {
    let map = Map::new(input)?;
    let trailheads = map.find_trailheads();
    let tops = map.find_tops();

    let mut total_score = 0;
    for trailhead in trailheads {
        total_score += find_number_of_tops_reachable_from_trailhead(&map, trailhead, tops.iter().cloned().collect());
    }

    Some(total_score)
}

// Now this one is easy to solve using recursion, which would've not been very obvious
// if we stuck with the graph finding library mentioned above :/
fn find_distinct_hiking_trails(map: &Map, current_pos: Pos, current_height: u8) -> u32 {
    // Base case
    if current_height == 9 {
        return 1;
    }

    let mut sum = 0;

    for new_pos in get_surrounding_positions(current_pos) {
        if let Some(height) = map.get_char_as_num(new_pos) {
            // each reachable node has an exact increase of 1
            if height.wrapping_sub(current_height) == 1 {
                sum += find_distinct_hiking_trails(&map, new_pos, height);
            }
        }
    }

    sum
}

fn find_number_of_distinct_hiking_trails_for_all_trailheads(input: &str) -> Option<u32> {
    let map = Map::new(input)?;
    let trailheads = map.find_trailheads();
    // let tops = map.find_tops();

    let mut total_score = 0;
    for trailhead in trailheads {
        total_score += find_distinct_hiking_trails(&map, trailhead, 0);
    }

    Some(total_score)
}

fn main() {
    const INPUT: &str = include_str!("../inputs/10.txt");

    println!("Sum of the score of all trailheads: {}", find_score_of_all_trailheads(INPUT).unwrap());

    println!("Sum of the number of distinct trails for all trailheads: {}", find_number_of_distinct_hiking_trails_for_all_trailheads(INPUT).unwrap());
}

const EXAMPLE_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[test]
fn map_parse_test() {
    const SMALL_MAP: &str = "0123
1234
8765
9876";

    let map = Map::new(SMALL_MAP).unwrap();
    assert_eq!(map.width, 4);
    assert_eq!(map.height, 4);

    assert_eq!(map.get_char_as_num((0, 0)).unwrap(), 0);
    assert_eq!(map.get_char_as_num((3, 0)).unwrap(), 3);
    assert_eq!(map.get_char_as_num((0, 1)).unwrap(), 1);
    assert_eq!(map.get_char_as_num((1, 3)).unwrap(), 8);
    assert_eq!(map.get_char_as_num((3, 3)).unwrap(), 6);

    assert_eq!(map.get_char_as_num((4, 0)), None);
    assert_eq!(map.get_char_as_num((0, 4)), None);
    assert_eq!(map.get_char_as_num((4, 4)), None);
}

#[test]
fn find_score_of_all_trailheads_test() {
    let sut = find_score_of_all_trailheads(EXAMPLE_INPUT).unwrap();
    assert_eq!(sut, 36);
}

#[test]
fn sum_of_all_distinct_trails_test() {
    let sut = find_number_of_distinct_hiking_trails_for_all_trailheads(EXAMPLE_INPUT).unwrap();
    assert_eq!(sut, 81);
}
