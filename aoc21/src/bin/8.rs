use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("../inputs/8.txt");
    //     const INPUT: &str = "
    // be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let displays: Vec<(&str, &str)> = INPUT
        .lines()
        .filter_map(|s| {
            if let [a, b, ..] = s.split(" | ").collect::<Vec<&str>>()[..] {
                return Some((a, b));
            }
            None
        })
        .collect();

    let smol_len = |s: &str| [2, 4, 3, 7].into_iter().any(|n| n == s.len());
    let easy_number_count = displays
        .iter()
        .map(|(_, b)| b.split(' ').collect::<Vec<&str>>())
        .flatten()
        .filter(|s| smol_len(s))
        .count();
    println!("1, 4, 7 or 8 appear {}", easy_number_count);

    println!("Union 'abc' 'ac' = {:?}", union_pattern("abc", "ac"));
    println!("Diff 'abc' 'ac' = {:?}", diff_pattern("abc", "ac"));
}

/** Bit Id's:
 *  000
 * 1   2
 * 1   2
 *  333
 * 4   5
 * 4   5
 *  666
 */
fn flags_to_numberdisplay(flag: u32) -> u32 {
    match flag {
        0b01110111 => 0,
        0b00100100 => 1,
        0b01011101 => 2,
        0b01101101 => 3,
        0b00101110 => 4,
        0b01101011 => 5,
        0b01111011 => 6,
        0b00100101 => 7,
        0b01111111 => 8,
        0b01101111 => 9,
        _ => 0,
    }
}

fn union_pattern(l: &str, r: &str) -> Option<char> {
    l.chars().filter(|lc| r.chars().any(|rc| rc == *lc)).next()
}

fn diff_pattern(l: &str, r: &str) -> Option<char> {
    let a = l.chars().filter(|lc| r.chars().all(|rc| rc != *lc)).next();
    if let None = a {
        r.chars().filter(|rc| l.chars().all(|lc| lc != *rc)).next()
    } else {
        a
    }
}

fn uniques(ls: &str) -> HashSet<char> {
    let set: HashSet<char>;
    for el in ls.chars() {
        set.insert(el);
    }
    set
}

fn union_set(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.union(&b).map(|v| *v).collect()
}

fn diff_set(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.difference(&b).map(|v| *v).collect()
}

/**
 * Patterns are ordered by length: 2, 3, 4, 5, 5, 5, 6, 6, 6, 7
 * This means we already know they represent the numbers: 1, 7, 4, {2, 3, 5}, {0, 6, 9}, 8
 * , meaning we only have to find 2, 3, 5 and 0, 6, 9
 */
fn solve(patterns: &[&str]) -> [Option<char>; 10] {
    let mut sequence: [Option<char>; 10] = [None; 10];
    // diff(7, 1) = the first sequence
    sequence[0] = diff_set(&uniques(patterns[1]), &uniques(patterns[0]))
        .into_iter()
        .next();
    // union(diff(3, 8), 4) = second sequence
    for p in &patterns[3..6] {
        // if union(num, 1) == 2, we found 3
        if union_set(&uniques(p), &uniques(patterns[0])).iter().count() == 2 {
            let lefts = union_set(&uniques(p), &uniques(patterns[7]));

            sequence[1] = diff_set(&uniques(patterns[2]), &lefts).into_iter().next();
            if let Some(a) = sequence[1] {
                sequence[4] = diff_set(&lefts, &HashSet::from([a])).into_iter().next();

                // Union of 9 and 6 == 5, which means the last one is 0
                
            }
        }
    }

    sequence
}
