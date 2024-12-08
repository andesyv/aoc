use std::collections::{HashMap, HashSet};

fn get_map_width_height(input: &str) -> (i64, i64) {
    let mut width = 0;
    for line in input.lines() {
        width = line.len();
        break;
    }

    let height = input.trim().lines().count();
    (i64::try_from(width).unwrap(), i64::try_from(height).unwrap())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Pos {
        Pos { x, y }
    }

    fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x.wrapping_add(other.x),
            y: self.y.wrapping_add(other.y),
        }
    }

    fn sub(self: &Pos, other: &Pos) -> Pos {
        Pos {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y),
        }
    }

    fn is_within_map(&self, width: i64, height: i64) -> bool {
        0 <= self.x && 0 <= self.y && self.x < width && self.y < height
    }
}

fn get_antennas(input: &str) -> HashMap<char, Vec<Pos>> {
    let mut antennas = HashMap::new();

    for (line, y) in input.trim().lines().zip(0i64..) {
        for (c, x) in line.chars().zip(0i64..) {
            if c == '.' { continue; }
            if !antennas.contains_key(&c) {
                antennas.insert(c, Vec::new());
            }
            antennas.get_mut(&c).unwrap().push(Pos::new(x, y));
        }
    }

    antennas
}

fn get_antinodes_for_antenna_set(antennas: &[Pos], width: i64, height: i64) -> Vec<Pos> {
    let mut antinodes = Vec::new();

    for a in antennas {
        for b in antennas {
            if b == a { continue; }

            let diff = b.sub(a);
            let a_antinode = a.sub(&diff);
            if a_antinode.is_within_map(width, height) {
                antinodes.push(a_antinode);
            }

            let b_antinode = b.add(&diff);
            if b_antinode.is_within_map(width, height) {
                antinodes.push(b_antinode);
            }
        }
    }

    antinodes
}

fn get_count_of_unique_antinodes_for_antennas(input: &str) -> u32 {
    let (width, height) = get_map_width_height(input);
    let antenna_set = get_antennas(input);

    let mut antinodes = HashSet::new();
    for antennas in antenna_set.values() {
        for antinode in get_antinodes_for_antenna_set(&antennas[..], width, height) {
            antinodes.insert(antinode);
        }
    }

    u32::try_from(antinodes.len()).unwrap()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/8.txt");

    println!("Unique antennas: {}", get_count_of_unique_antinodes_for_antennas(INPUT));
}

const EXAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[test]
fn unique_antinodes_example() {
    assert_eq!(get_count_of_unique_antinodes_for_antennas(EXAMPLE_INPUT), 14);
}