use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

#[derive(PartialEq)]
enum CaveType {
    Start,
    End,
    Big,
    Small,
}

struct Cave<'a> {
    name: &'a str,
    cave_type: CaveType,
    neighbours: Vec<Weak<RefCell<Cave<'a>>>>,
}

trait Graph {
    fn traverse_all(&self, prev: &Vec<&str>, visit_small_caves_twice: bool) -> Vec<String>;
}

impl Graph for Cave<'_> {
    fn traverse_all(&self, prev: &Vec<&str>, visit_small_caves_twice: bool) -> Vec<String> {
        let current_path = [&prev[..], &[self.name][..]].concat();
        if CaveType::End == self.cave_type {
            return vec![current_path.join(",")];
        }
        let mut mutators = Vec::new();
        let firsts: Vec<String> = self
            .neighbours
            .iter()
            .filter_map(|next| {
                let next_cell = next.upgrade().unwrap();
                let next_cave = next_cell.borrow();
                if match next_cave.cave_type {
                    CaveType::Start => false,
                    CaveType::Small => {
                        if prev.contains(&next_cave.name) {
                            mutators.push(Rc::downgrade(&next_cell));
                            false
                        } else {
                            true
                        }
                    }
                    _ => true,
                } {
                    Some(next_cave.traverse_all(&current_path, visit_small_caves_twice))
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        // If we haven't already visited a small cave twice,
        // add the mutated paths to the list of possible paths
        if visit_small_caves_twice && can_mutate_small_caves(&current_path) {
            [
                &firsts[..],
                &mutators
                    .into_iter()
                    .map(|next| {
                        let next_cell = next.upgrade().unwrap();
                        let next_cave = next_cell.borrow();
                        next_cave.traverse_all(&current_path, visit_small_caves_twice)
                    })
                    .flatten()
                    .collect::<Vec<String>>()[..],
            ]
            .concat()
        } else {
            firsts
        }
    }
}

fn find_type(name: &str) -> CaveType {
    match name {
        "start" => CaveType::Start,
        "end" => CaveType::End,
        a if 0 < a.len() && a.chars().nth(0).unwrap().is_uppercase() => CaveType::Big,
        _ => CaveType::Small,
    }
}

// Returns true unless it finds a duplicate small cave
fn can_mutate_small_caves(prevs: &Vec<&str>) -> bool {
    let mut uniques: HashSet<&str> = HashSet::new();
    for key in prevs {
        if !uniques.insert(key) && find_type(key) == CaveType::Small {
            return false;
        }
    }
    true
}

fn build_graph(input: &str) -> HashMap<&str, Rc<RefCell<Cave>>> {
    // Build graph:
    let mut graph: HashMap<&str, Rc<RefCell<Cave>>> = HashMap::new();
    for edge in input.lines() {
        if let Some((a_key, b_key)) = edge.split_once('-') {
            if !graph.contains_key(a_key) {
                graph.insert(
                    a_key,
                    Rc::new(RefCell::new(Cave {
                        name: a_key,
                        cave_type: find_type(a_key),
                        neighbours: Vec::new(),
                    })),
                );
            }
            if !graph.contains_key(b_key) {
                graph.insert(
                    b_key,
                    Rc::new(RefCell::new(Cave {
                        name: b_key,
                        cave_type: find_type(b_key),
                        neighbours: Vec::new(),
                    })),
                );
            }

            let a_entry = graph.get(a_key).unwrap();
            let b_entry = graph.get(b_key).unwrap();
            if let (Ok(mut a_cave), Ok(mut b_cave)) =
                (a_entry.try_borrow_mut(), b_entry.try_borrow_mut())
            {
                a_cave.neighbours.push(Rc::downgrade(b_entry));
                b_cave.neighbours.push(Rc::downgrade(a_entry));
            } else {
                panic!("Failed to mutably borrow RefCell!");
            }
        }
    }
    graph
}

fn solve1(input: &str) -> usize {
    let graph = build_graph(input);
    // Traverse graph:
    let start = graph.get("start").unwrap();
    let all_paths = start.borrow().traverse_all(&Vec::new(), false);
    all_paths.len()
}

fn solve2(input: &str) -> usize {
    let graph = build_graph(input);
    // Traverse graph:
    let start = graph.get("start").unwrap();
    let all_paths = start.borrow().traverse_all(&Vec::new(), true);
    all_paths.len()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/12.txt");
    println!("Part 1: path count: {}", solve1(INPUT));

    println!("Part 2: path count: {}", solve2(INPUT));
}

#[test]
fn test1() {
    const INPUT: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    assert_eq!(solve1(INPUT), 10);
    assert_eq!(solve2(INPUT), 36);
}

#[test]
fn test2() {
    assert_eq!(
        solve1("dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc"),
        19
    )
}

#[test]
fn test3() {
    assert_eq!(solve1("fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW"), 226)
}
