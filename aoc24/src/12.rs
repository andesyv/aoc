use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);

fn collect_connected_sets(input: &str) -> Vec<HashSet<Pos>> {
    let mut grouped_sets: HashMap::<u8, Vec<Pos>> = HashMap::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            if !grouped_sets.contains_key(c) {
                grouped_sets.insert(*c, vec![(x, y)]);
            } else {
                grouped_sets.get_mut(c).unwrap().push((x, y));
            }
        }
    }

    let mut connected_sets = Vec::with_capacity(grouped_sets.len());
    for set in grouped_sets.values() {
        let mut clusters = Vec::new();
        for pos in set {

        }
    }
}

fn main() {
    println!("Hello AOC!");
}

const SMALL_EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

const MEDIUM_EXAMPLE: &str ="OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const BIG_EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
