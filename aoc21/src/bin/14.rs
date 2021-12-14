use itertools::Itertools;
use std::collections::HashMap; // For next_tuple()
use std::time::Instant;

fn main() {
    const INPUT: &str = include_str!("../inputs/14.txt");
    let mut polymer: Vec<char> = INPUT.lines().next().unwrap().chars().collect();

    let rules: HashMap<(char, char), char> = INPUT
        .lines()
        .skip(2)
        .filter_map(|l| {
            let (l, r) = l.split_once(" -> ")?;
            let pat = l.chars().next_tuple()?;
            Some((pat, r.chars().next().unwrap()))
        })
        .collect();

    let now = Instant::now();
    for _ in 0..40 {
        polymer = polymerize(&polymer, &rules);
    }
    println!("Polymerization took {} seconds to execute", now.elapsed().as_secs());

    let mut frequency: HashMap<char, u64> = HashMap::new();
    for c in polymer {
        *frequency.entry(c).or_default() += 1;
    }

    if let (Some(max), Some(min)) = (frequency.iter().map(|t|t.1).max(), frequency.iter().map(|t|t.1).min()) {
        println!("Most common - least common = {}", max - min);
    }
}

#[test]
fn test1() {
    const INPUT: &str = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";
    let polymer: Vec<char> = INPUT.lines().next().unwrap().chars().collect();

    let rules: HashMap<(char, char), char> = INPUT
        .lines()
        .skip(2)
        .filter_map(|l| {
            let (l, r) = l.split_once(" -> ")?;
            let pat = l.chars().next_tuple()?;
            Some((pat, r.chars().next().unwrap()))
        })
        .collect();

    let polymer = polymerize_n(&polymer, &rules, 10);

    let mut frequency: HashMap<char, u32> = HashMap::new();
    for c in polymer.chars() {
        *frequency.entry(c).or_default() += 1;
    }
    if let (Some(max), Some(min)) = (frequency.iter().map(|t|t.1).max(), frequency.iter().map(|t|t.1).min()) {
        assert_eq!(max - min, 1588);
    } else {
        unreachable!();
    }
}

fn polymerize(polymer: &Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    if polymer.is_empty() {
        panic!("Waii. ;_;");
    } else {
        // polymer
        //     .windows(2)
        //     .filter_map(|window| {
        //         if let &[a, b, ..] = window {
        //             if let Some(inserted) = rules.get(&(a, b)) {
        //                 Some(vec![a, *inserted])
        //             } else {
        //                 Some(vec![a])
        //             }
        //         } else {
        //             None
        //         }
        //     })
        //     .flatten()
        //     .chain(std::iter::once(*polymer.last().unwrap()))
        //     .collect()
        let mut out: Vec<char> = Vec::with_capacity(polymer.len() * 2);
        let mut it = polymer.windows(2);
        while let Some(&[a,b]) = it.next() {
            out.push(a);
            if let Some(inserted) = rules.get(&(a, b)) {
                out.push(*inserted);
            }
        }
        out.push(*polymer.last().unwrap());
        out
    }
}

fn polymerize_n(polymer: &Vec<char>, rules: &HashMap<(char, char), char>, n: u32) -> String {
    if n == 0 {
        return polymer.into_iter().collect::<String>();
    } else {
        polymerize_n(&polymerize(&polymer, &rules), &rules, n-1)
    }
}