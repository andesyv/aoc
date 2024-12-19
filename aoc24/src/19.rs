use std::collections::{HashMap, HashSet};
use std::time::Instant;
// use radix_trie::Trie;

// A simple wrapper around the "Trie" type to use it as a regular HashSet
// Update: After testing, it turns out that a simple HashSet is faster than the radix_trie.
// Likely because the `radix_trie` does a bunch of unnecessary allocations under the hood.
// (as we're using a string slice there really should not be a need to do any allocations)
// Complexity wise the radix tree should be faster, so there's probably some optimization
// opportunities in the crate.
// struct StringSet<'a> {
//     trie: Trie<&'a str, ()>,
// }
//
// impl<'a> StringSet<'a> {
//     fn new() -> Self {
//         Self { trie: Trie::new() }
//     }
//
//     fn contains(&self, s: &'a str) -> bool {
//         self.trie.get(s).is_some()
//     }
//
//     fn insert(&mut self, s: &'a str) -> bool {
//         self.trie.insert(s, ()).is_some()
//     }
// }
//
// impl<'a> FromIterator<&'a str> for StringSet<'a> {
//     fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
//         let mut set = Self::new();
//         for s in iter {
//             set.insert(s);
//         }
//         set
//     }
// }

fn parse(input: &str) -> Option<(Vec<&str>, Vec<&str>)> {
    let mut lines = input.lines();
    let first_line = lines.next()?;
    let patterns = first_line.trim().split(", ").filter_map(|segment|{
        let trimmed = segment.trim();
        if !trimmed.is_empty() {
            Some(trimmed)
        } else {
            None
        }
    }).collect();

    let mut strings = Vec::new();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        strings.push(trimmed);
    }

    Some((patterns, strings))
}

fn is_combination_possible<'a>(combination: &'a str, patterns: &HashSet<&str>, memoized_results: &mut HashMap<&'a str, bool>) -> bool {
    if combination.is_empty() {
        return true;
    }

    if let Some(precalculated_result) = memoized_results.get(combination) {
        return *precalculated_result;
    }

    // println!("Checking combination: {}", combination);

    for i in (1..combination.len() + 1).rev() {
        if patterns.contains(&combination[0..i]) {
            if is_combination_possible(&combination[i..], patterns, memoized_results) {
                memoized_results.insert(combination, true);
                return true;
            }
        }
    }

    memoized_results.insert(combination, false);
    false
}

fn count_possible_combinations(patterns: &[&str], strings: &[&str]) -> usize {
    // Collecting all strings into a hash tree for fast lookups. In theory, using strings in hash
    // trees can lead to hash collisions. However, in practice I think this should be sufficiently
    // safe for this task. Additionally, using a trie would likely be faster. (potential to use
    // the `radix_trie` crate here)

    let pattern_lookup: HashSet<_> = patterns.into_iter().map(|p|*p).collect();
    let mut pattern_combination_lookup_cache = HashMap::new();

    let mut count = 0;
    for string in strings {
        if is_combination_possible(*string, &pattern_lookup, &mut pattern_combination_lookup_cache) {
            count += 1;
        }
    }

    count
}

fn get_possible_combinations_from_input(input: &str) -> usize {
    let start = Instant::now();
    let (patterns, strings) = parse(input).unwrap();
    let result = count_possible_combinations(&patterns, &strings);
    println!("Possible combinations took {}ms to calculate", start.elapsed().as_millis());
    result
}

fn main() {
    const INPUT: &str = include_str!("../inputs/19.txt");
    println!("Possible combinations: {}", get_possible_combinations_from_input(INPUT));
}

const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"; // 6 / 8 are possible

#[test]
fn count_possible_combinations_from_example() {
    assert_eq!(get_possible_combinations_from_input(EXAMPLE_INPUT), 6);
}
