use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

type Pos = (u32, u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Configuration = (Pos, Direction);

impl TryFrom<i16> for Direction {
    type Error = &'static str;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value_looped = value.rem_euclid(4);
        match value_looped {
            0 => Ok(Direction::North),
            1 => Ok(Direction::East),
            2 => Ok(Direction::South),
            3 => Ok(Direction::West),
            _ => Err("Invalid direction"),
        }
    }
}

impl From<Direction> for i16 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

fn rotate_cw(dir: Direction) -> Direction {
    Direction::try_from(i16::from(dir) + 1).unwrap()
}

fn rotate_ccw(dir: Direction) -> Direction {
    Direction::try_from(i16::from(dir) - 1).unwrap()
}

struct Map<'a> {
    tokens: &'a [u8],
    width: u32,
    height: u32,
    start: Pos,
    goal: Pos,
}

impl<'a> Map<'a> {
    fn new(input: &'a str) -> Map<'a> {
        let trimmed = input.trim();
        let width = trimmed.find('\n').unwrap() as u32;
        let height = trimmed.lines().count() as u32;
        let mut map = Map {
            tokens: trimmed.as_bytes(),
            width,
            height,
            start: (0, 0),
            goal: (0, 0),
        };

        map.start = map.get_pos_of_char('S' as u8).unwrap();
        map.goal = map.get_pos_of_char('E' as u8).unwrap();
        map
    }

    fn get_char_at_pos(&self, pos: Pos) -> Option<u8> {
        let (x, y) = pos;
        if self.width <= x {
            return None;
        }

        let i = ((self.width + 1) * y + x) as usize;
        if self.tokens.len() <= i {
            return None;
        }

        self.tokens.get(i).copied()
    }

    fn get_pos_of_char(&self, c: u8) -> Option<Pos> {
        let i = self.tokens.iter().position(|&x| x == c)?;
        let x = i % (self.width + 1) as usize;
        let y = i / (self.width + 1) as usize;
        Some((x as u32, y as u32))
    }

    fn is_wall(&self, pos: Pos) -> bool {
        self.get_char_at_pos(pos) == Some('#' as u8)
    }

    fn get_next_pos(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        let x = match dir {
            Direction::East => pos.0 + 1,
            Direction::West => pos.0.wrapping_sub(1),
            _ => pos.0,
        };

        let y = match dir {
            Direction::South => pos.1 + 1,
            Direction::North => pos.1.wrapping_sub(1),
            _ => pos.1,
        };

        if x < self.width && y < self.height {
            Some((x, y))
        } else {
            None
        }
    }

    /**
     * It turns out that the problem with this logic, is that the cost for traversing from a cell
     * is greedily taken as the shortest path to the goal, without considering the steps that was
     * required to get to the cell.
     * Consider this example:
     * ```text
     * #########
     * #......E#
     * ###.#.###
     * #...#...#
     * #.#.#.###
     * #.......#
     * #.#######
     * #########
     * ```
     * Here, this greedy logic will determine that this is the best path:
     * ```text
     * #########
     * #..>>>>E#
     * ###^#.###
     * #..^#...#
     * #.#^#.###
     * #>>^....#
     * #.#######
     * #########
     * ```
     * However, if the start position was at the bottom left, getting to that "optimal path"
     * requires an additional turn, at which point it would be cheaper to take the other straighter
     * route instead:
     * ```text
     * More expensive:
     * #########
     * #..>>>>E#
     * ###^#.###
     * #..^#...#
     * #.#^#.###
     * #^>^....#
     * #S#######
     * #########
     *
     * Cheaper:
     * #########
     * #..>>>>E#
     * ###^#.###
     * #>>^#...#
     * #^#.#.###
     * #^......#
     * #S#######
     * #########
     * ```
     */
    // fn memoized_traversal(&self, conf: Configuration, prev_configuration: Configuration, memoized_results: &mut HashMap<(Configuration, Configuration), Option<usize>>) -> Option<usize> {
    //     let (pos, dir) = conf;
    //     if pos == self.goal {
    //         // println!("Found the goal! Conf: {:?}", conf);
    //         return Some(0);
    //     }
    //
    //     // Can't traverse a wall
    //     if self.is_wall(pos) {
    //         return None;
    //     }
    //
    //     // Skip re-traversing already calculated configurations
    //     if let Some(score) = memoized_results.get(&(conf, prev_configuration)) {
    //         return *score;
    //     }
    //
    //     // println!("Traversal: Current conf = {:?}, previous conf = {:?}", conf, prev_configuration);
    //
    //     // Preemptively append the current configuration to prevent infinite loops down the line
    //     memoized_results.insert((conf, prev_configuration), None);
    //
    //     let mut best_score = None;
    //
    //     // First try moving, as this will always incur the cheapest cost.
    //     if let Some(next_pos) = self.get_next_pos(pos, dir) {
    //         if let Some(score) = self.memoized_traversal((next_pos, dir), conf, memoized_results) {
    //             let score = score + 1;
    //             if score < best_score.unwrap_or(usize::MAX) {
    //                 best_score = Some(score);
    //             }
    //         }
    //     }
    //
    //     // Then try rotating left and right
    //     if let Some(score) = self.memoized_traversal((pos, rotate_cw(dir)), conf, memoized_results) {
    //         let score = score + 1000;
    //         if score < best_score.unwrap_or(usize::MAX) {
    //             best_score = Some(score);
    //         }
    //     }
    //
    //     if let Some(score) = self.memoized_traversal((pos, rotate_ccw(dir)), conf, memoized_results) {
    //         let score = score + 1000;
    //         if score < best_score.unwrap_or(usize::MAX) {
    //             best_score = Some(score);
    //         }
    //     }
    //
    //     // Now replace the value with the actual calculated value
    //     memoized_results.insert((conf, prev_configuration), best_score);
    //
    //     best_score
    // }

    fn print(&self, memoized_results: &HashMap<(Configuration, Configuration), Option<usize>>) {
        let mut best_dirs = HashMap::new();
        for ((conf, _), score) in memoized_results {
            let (pos, dir) = *conf;
            let score = score.unwrap_or(usize::MAX);
            let current_best_score = best_dirs
                .get(&pos)
                .map(|(_, best_score)| *best_score)
                .unwrap_or(usize::MAX);
            if score < current_best_score {
                best_dirs.insert(pos, (dir, score));
            }
        }

        println!("Map:");
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if self.is_wall(pos) {
                    print!("#");
                } else if self.start == pos {
                    print!("S");
                } else if self.goal == pos {
                    print!("E");
                } else if let Some((dir, _)) = best_dirs.get(&pos) {
                    print!(
                        "{}",
                        match *dir {
                            Direction::East => ">",
                            Direction::South => "v",
                            Direction::West => "<",
                            Direction::North => "^",
                        }
                    );
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn print_good_spots(&self, good_spots: &HashMap<Configuration, usize>) {
        let good_spot_positions: HashSet<_> = good_spots.keys().map(|(pos, _)| *pos).collect();

        println!("Good spots:");
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if self.is_wall(pos) {
                    print!("#");
                } else if self.start == pos {
                    print!("S");
                } else if self.goal == pos {
                    print!("E");
                } else if good_spot_positions.contains(&pos) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // fn traverse_2_electric_boogaloo(&self, conf: Configuration, current_cost: usize, previous_results: &mut HashMap<Configuration, usize>) -> Option<usize> {
    //     if current_cost > 20000 {
    //         panic!("Probably too high?");
    //     }
    //     let (pos, dir) = conf;
    //     if pos == self.goal {
    //         // println!("Found the goal! Conf: {:?}", conf);
    //         return Some(current_cost);
    //     }
    //
    //     // Can't traverse a wall
    //     if self.is_wall(pos) {
    //         return None;
    //     }
    //
    //     // Skip re-traversing already calculated configurations
    //     if let Some(previous_cost) = previous_results.get(&conf) {
    //         if *previous_cost < current_cost {
    //             return None;
    //         }
    //     }
    //
    //     previous_results.insert(conf, current_cost);
    //
    //     let mut best_score = None;
    //
    //     // First try moving, as this will always incur the cheapest cost.
    //     if let Some(next_pos) = self.get_next_pos(pos, dir) {
    //         if let Some(score) = self.traverse_2_electric_boogaloo((next_pos, dir), current_cost + 1, previous_results) {
    //             if score < best_score.unwrap_or(usize::MAX) {
    //                 best_score = Some(score);
    //             }
    //         }
    //     }
    //
    //     // Then try rotating left and right
    //     if let Some(score) = self.traverse_2_electric_boogaloo((pos, rotate_cw(dir)), current_cost + 1000, previous_results) {
    //         if score < best_score.unwrap_or(usize::MAX) {
    //             best_score = Some(score);
    //         }
    //     }
    //
    //     if let Some(score) = self.traverse_2_electric_boogaloo((pos, rotate_ccw(dir)), current_cost + 1000, previous_results) {
    //         if score < best_score.unwrap_or(usize::MAX) {
    //             best_score = Some(score);
    //         }
    //     }
    //
    //     best_score
    // }

    fn traverse_iteratively(&self, start_conf: Configuration) -> HashMap<Configuration, usize> {
        let mut next = VecDeque::from([(start_conf, 0)]);
        let mut visited = HashMap::new();
        // let mut best_cost = None;
        while let Some(((pos, dir), cost)) = next.pop_front() {
            if self.is_wall(pos) {
                continue;
            }

            if let Some(previous_cost) = visited.get(&(pos, dir)) {
                if *previous_cost <= cost {
                    continue;
                }
            }

            visited.insert((pos, dir), cost);

            if pos == self.goal {
                // if cost < best_cost.unwrap_or(usize::MAX) {
                //     best_cost = Some(cost);
                // }

                continue;
            }

            if let Some(next_pos) = self.get_next_pos(pos, dir) {
                next.push_back(((next_pos, dir), cost + 1));
            }
            next.push_back(((pos, rotate_cw(dir)), cost + 1000));
            next.push_back(((pos, rotate_ccw(dir)), cost + 1000));
        }
        visited
        // best_cost
    }

    fn find_end_config(&self, results: &HashMap<Configuration, usize>) -> Option<(Configuration, usize)> {
        let best_key = results
            .keys()
            .filter(|(pos, _)| *pos == self.goal)
            .min_by_key(|key|*results.get(key).unwrap());
        best_key.map(|key| (*key, *results.get(key).unwrap()))
    }

    fn config_is_reachable(&self, current: (Configuration, usize), previous: (Configuration, usize)) -> bool {
        let ((prev_pos, prev_dir), prev_cost) = previous;
        let ((curr_pos, curr_dir), curr_cost) = current;

        if curr_pos == prev_pos && prev_cost + 1000 == curr_cost {
            if curr_dir == rotate_cw(prev_dir) || curr_dir == rotate_ccw(prev_dir) {
                return true;
            }
        }

        if prev_dir == curr_dir && prev_cost + 1 == curr_cost {
            if let Some(pos) = self.get_next_pos(prev_pos, prev_dir) {
                if pos == curr_pos {
                    return true;
                }
            }
        }

        false
    }

    // As we've already traversed all possible combinations to find the best path, we can traverse backwards
    // to find "a" best path using the traversal results. For any configuration starting with the end one,
    // if any configuration is reachable from the current and it's cost is lower, it's part of a "best path".
    fn filter_best_paths(&self, mut results: HashMap<Configuration, usize>) -> HashMap<Configuration, usize> {
        let end_config = self.find_end_config(&results);
        if end_config.is_none() {
            return HashMap::new();
        }

        let mut to_process = VecDeque::from([end_config.unwrap()]);
        let mut to_remove = Vec::new();
        let mut safe_max_cost = end_config.unwrap().1;
        let mut filtered_results = HashMap::new();
        while let Some((curr_conf, curr_cost)) = to_process.pop_front() {
            for (prev_conf, prev_cost) in &results {
                if *prev_cost >= curr_cost {
                    if *prev_cost >= safe_max_cost {
                        to_remove.push(*prev_conf);
                    }
                    continue;
                }

                if !self.config_is_reachable((curr_conf, curr_cost), (*prev_conf, *prev_cost)) {
                    continue;
                }

                to_process.push_back((*prev_conf, *prev_cost));
            }

            safe_max_cost = to_process.iter().map(|(_, cost)| *cost).max().unwrap_or(safe_max_cost);

            for conf in to_remove.drain(..) {
                results.remove(&conf);
            }

            filtered_results.insert(curr_conf, curr_cost);
        }

        filtered_results
    }
}

fn find_min_score_to_reach_goal(input: &str) -> Result<usize, &'static str> {
    let map = Map::new(input);
    let results = map.traverse_iteratively((map.start, Direction::East));
    let best_score = map.find_end_config(&results).map(|(_, score)| score);
    if let Some(score) = best_score {
        Ok(score)
    } else {
        Err("Could not find a path to the goal")
    }
}

fn find_count_of_good_spots(input: &str) -> Result<usize, &'static str> {
    let map = Map::new(input);
    let results = map.traverse_iteratively((map.start, Direction::East));
    let best_paths = map.filter_best_paths(results);
    map.print_good_spots(&best_paths);
    let good_spot_count = best_paths.keys().map(|(pos, _)|*pos).unique().count();

    if good_spot_count > 0 {
        Ok(good_spot_count)
    } else {
        Err("Could not find a path to the goal")
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/16.txt");
    println!(
        "Smallest cost to reach goal: {}",
        find_min_score_to_reach_goal(INPUT).unwrap()
    );

    println!(
        "Count of good spots: {}",
        find_count_of_good_spots(INPUT).unwrap()
    );
}

const EXAMPLE_INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const EXAMPLE_INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

const EXAMPLE_INPUT_3: &str ="############
#.#######E##
#.#........#
#.#.###.####
#.#...#...##
#.###.#.#.##
#.#........#
#.#.###.#.##
#.#S###...##
############";

const EXAMPLE_INPUT_4: &str ="##############
#######E#.####
..#...#.....##
#.#.#.#.###.##
..#.#...#...##
#.#.#####.####
#.#.........##
#.#.###.###.##
#.#...#...#..#
#.###.#.#.#.##
#.#..........#
#.#.###.#.####
#.#S#...#...##
##############";

#[test]
fn parse_test() {
    let sut = Map::new(EXAMPLE_INPUT_1);
    assert_eq!(sut.width, 15);
    assert_eq!(sut.height, 15);
    assert_eq!(sut.start, (1, 13));
    assert_eq!(sut.goal, (13, 1));
    assert!(sut.is_wall((0, 0)));
    assert!(sut.is_wall((3, 0)));
    assert!(sut.is_wall((0, 3)));
}

#[test]
fn min_score_to_reach_goal_example_1() {
    let result = find_min_score_to_reach_goal(EXAMPLE_INPUT_1).unwrap();
    assert_eq!(result, 7036);
}

#[test]
fn min_score_to_reach_goal_example_2() {
    let result = find_min_score_to_reach_goal(EXAMPLE_INPUT_2).unwrap();
    assert_eq!(result, 11048);
}

#[test]
fn find_good_spot_count_example_1() {
    let result = find_count_of_good_spots(EXAMPLE_INPUT_1).unwrap();
    assert_eq!(result, 45);
}

#[test]
fn find_good_spot_count_example_2() {
    let result = find_count_of_good_spots(EXAMPLE_INPUT_2).unwrap();
    assert_eq!(result, 64);
}

#[test]
fn find_good_spot_count_example_3() {
    let result = find_count_of_good_spots(EXAMPLE_INPUT_3).unwrap();
    assert_eq!(result, 14);
}

#[test]
fn find_good_spot_count_example_4() {
    let result = find_count_of_good_spots(EXAMPLE_INPUT_4).unwrap();
    assert_eq!(result, 41);
}
