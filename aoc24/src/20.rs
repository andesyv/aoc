use std::collections::{HashMap, HashSet};

type Pos = (i64, i64);

fn get_neighbours(pos: Pos) -> [Pos; 4] {
    let (x, y) = pos;
    [
        (x + 1, y),
        (x.wrapping_sub(1), y),
        (x, y + 1),
        (x, y.wrapping_sub(1)),
    ]
}

fn get_intermediate_position(a: Pos, b: Pos) -> Pos {
    let (a_x, a_y) = a;
    let (b_x, b_y) = b;
    (a_x + (b_x - a_x) / 2, a_y + (b_y - a_y) / 2)
}

struct Map<'a> {
    tokens: &'a str,
    width: i64,
    height: i64,
    start: Pos,
    end: Pos,
}

impl<'a> Map<'a> {
    fn new(input: &'a str) -> Self {
        let tokens = input.trim();
        let width = tokens.lines().next().unwrap().len() as i64;
        let height = tokens.lines().count() as i64;
        let mut map = Self {
            tokens,
            width,
            height,
            start: (0, 0),
            end: (0, 0),
        };
        map.start = map.get_pos_of_char('S').unwrap();
        map.end = map.get_pos_of_char('E').unwrap();
        map
    }

    fn get_char_at_pos(&self, pos: Pos) -> Option<char> {
        let (x, y) = pos;
        if x >= self.width || y >= self.height {
            return None;
        }
        self.tokens.chars().nth((y * (self.width + 1) + x) as usize)
    }

    fn is_wall(&self, pos: Pos) -> bool {
        self.get_char_at_pos(pos) == Some('#')
    }

    fn get_pos_of_char(&self, needle: char) -> Option<Pos> {
        for (y, line) in self.tokens.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == needle {
                    return Some((x as i64, y as i64));
                }
            }
        }
        None
    }

    fn traverse_track_normally(&self) -> HashMap<Pos, usize> {
        let mut results = HashMap::new();
        let mut current = self.start;
        let mut previous = current;
        for cost in 0.. {
            results.insert(current, cost);

            if current == self.end {
                break;
            }

            for neighbour in get_neighbours(current) {
                if neighbour != previous && !self.is_wall(neighbour) {
                    previous = current;
                    current = neighbour;
                    break;
                }
            }
        }
        results
    }

    fn get_cheat_positions(&self, pos: Pos) -> Vec<Pos> {
        let mut cheat_positions = Vec::new();
        for (i, neighbour) in get_neighbours(pos).into_iter().enumerate() {
            if !self.is_wall(neighbour) {
                continue;
            }

            let next_neighbour = get_neighbours(neighbour)[i];
            if self.is_wall(next_neighbour) {
                continue;
            }

            cheat_positions.push(next_neighbour);
        }
        cheat_positions
    }

    fn get_cheats_and_timesaves(
        &self,
        normal_track_cost: &HashMap<Pos, usize>,
    ) -> Vec<(Pos, Pos, usize)> {
        let mut cheats = Vec::new();

        for (pos, current_cost) in normal_track_cost {
            for cheat_position in self.get_cheat_positions(*pos) {
                if let Some(cheat_position_cost) = normal_track_cost.get(&cheat_position) {
                    // The cheat traverses 2 spaces as well, so it costs 2 picoseconds too
                    if *cheat_position_cost > *current_cost + 2 {
                        cheats.push((
                            get_intermediate_position(*pos, cheat_position),
                            cheat_position,
                            cheat_position_cost - current_cost - 2,
                        ));
                    }
                }
            }
        }

        cheats
    }

    fn get_count_of_cheats_that_would_save_n_picoseconds(
        &self,
        n: usize,
        normal_track_cost: &HashMap<Pos, usize>,
    ) -> usize {
        let cheats = self.get_cheats_and_timesaves(normal_track_cost);
        cheats.iter().filter(|(_, _, save)| *save >= n).count()
    }

    fn print_map_and_cheats(&self, cheats: &[(Pos, Pos, usize)]) {
        let cheats_start_positions: HashSet<_> =
            cheats.iter().map(|(start, _, _)| *start).collect();
        let cheats_end_positions: HashSet<_> = cheats.iter().map(|(_, end, _)| *end).collect();

        println!("Map:");
        for (y, line) in self.tokens.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if cheats_start_positions.contains(&(x as i64, y as i64)) {
                    print!("1");
                } else if cheats_end_positions.contains(&(x as i64, y as i64)) {
                    print!("2");
                } else {
                    print!("{}", c);
                }
            }
            println!()
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/20.txt");
    let map = Map::new(INPUT);
    let normal_track_results = map.traverse_track_normally();
    println!("Track traversed");

    println!("Count of cheats that saves at least a 100 picoseconds: {}", map.get_count_of_cheats_that_would_save_n_picoseconds(100, &normal_track_results));
}

const EXAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn normal_track_traversal_test() {
    let map = Map::new(EXAMPLE_INPUT);
    let results = map.traverse_track_normally();
    assert_eq!(results.len(), 85); // We include the start point as well (which is 0 picoseconds)
    assert_eq!(*results.get(&map.end).unwrap(), 84);
}

#[test]
fn cheat_time_saving_test() {
    let map = Map::new(EXAMPLE_INPUT);

    let normal_track_records = map.traverse_track_normally();
    let cheats = map.get_cheats_and_timesaves(&normal_track_records);
    // map.print_map_and_cheats(cheats.as_slice());
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 2).count(),
        14
    );
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 4).count(),
        14
    );
    assert_eq!(cheats.iter().filter(|(_, _, saved)| *saved == 6).count(), 2);
    assert_eq!(cheats.iter().filter(|(_, _, saved)| *saved == 8).count(), 4);
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 10).count(),
        2
    );
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 12).count(),
        3
    );
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 20).count(),
        1
    );
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 36).count(),
        1
    );
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 38).count(),
        1
    );
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 40).count(),
        1
    );
    assert_eq!(
        cheats.iter().filter(|(_, _, saved)| *saved == 64).count(),
        1
    );

    assert_eq!(
        map.get_count_of_cheats_that_would_save_n_picoseconds(40, &normal_track_records),
        2
    );
}
