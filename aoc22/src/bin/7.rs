use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum Object {
    File(usize),
    Dir(Vec<Box<Object>>),
}

enum Command<'a> {
    List,
    Navigate(&'a [&'a str]),
}

fn parse_tree(input: &str) -> Option<Object> {
    parse_sub_tree(&mut input.lines())
}

fn parse_sub_tree(input: &mut std::str::Lines) -> Option<Object> {
    // Continue consuming lines
    while let Some(line) = input.next() {
        // explicitly stop on ".." - done parsing subtree
        if line.starts_with("$ cd ..") {
            return None;
        } else if line.starts_with("$ cd") {
            let mut dir = Vec::new();
            while let Some(object) = parse_sub_tree(input) {
                dir.push(Box::new(object));
            }
            return Some(Object::Dir(dir));
        } else if line.chars().nth(0).map(char::is_numeric).unwrap_or(false) {
            return Some(Object::File(line.split(' ').next()?.parse().ok()?));
        }
    }
    None
}

fn parse_command<'a>(line: &'a [&'a str]) -> Option<Command<'a>> {
    match &line[..4] {
        ["$ cd"] => Some(Command::Navigate(&line[4..])),
        ["$ ls"] => Some(Command::List),
        _ => None,
    }
}

fn object_size(tree: &Object) -> usize {
    match tree {
        Object::File(s) => *s,
        Object::Dir(d) => d.iter().map(|b| object_size(b.as_ref())).sum(),
    }
}

fn find_sum_of_max_size_directories(tree: &Object, max_size: usize) -> usize {
    let rest = if let Object::Dir(d) = tree {
        d.iter()
            .map(|b| match b.as_ref() {
                // Only count if sub-object is a directory (pretty messy. I know)
                Object::Dir(_) => find_sum_of_max_size_directories(b.as_ref(), max_size),
                _ => 0,
            })
            .sum()
    } else {
        0
    };

    let curr_size = object_size(tree);
    (if curr_size <= max_size { curr_size } else { 0 }) + rest
}

fn find_smallest_directory_to_delete_for_space(tree: &Object, to_free: usize) -> Option<usize> {
    match tree {
        Object::Dir(d) => {
            let rest = d
                .iter()
                .filter_map(|b| find_smallest_directory_to_delete_for_space(b.as_ref(), to_free))
                .min();
            let current_size = object_size(tree);
            if to_free <= current_size {
                Some(rest.unwrap_or(current_size).min(current_size))
            } else {
                rest
            }
        }
        _ => None,
    }
}

fn find_smallest_eligible_directory_to_delete(tree: &Object) -> Option<usize> {
    let currently_free = 70000000 - object_size(tree);
    find_smallest_directory_to_delete_for_space(tree, 30000000 - currently_free)
}

fn main() {
    const INPUT: &str = include_str!("../inputs/7.txt");
    let tree = parse_tree(INPUT).unwrap();
    println!(
        "Sum of whatever: {}",
        find_sum_of_max_size_directories(&tree, 100000)
    );

    println!("Total size used: {}", object_size(&tree));
    println!(
      "The smallest directory that can be deleted to achieve the space increase has a size of {}",
      find_smallest_eligible_directory_to_delete(&tree).unwrap()
  );
}

const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[test]
fn test_parse() {
    let expected = Object::Dir(vec![
        Box::new(Object::File(14848514)),
        Box::new(Object::File(8504156)),
        // a
        Box::new(Object::Dir(vec![
            // e
            Box::new(Object::File(29116)),
            Box::new(Object::File(2557)),
            Box::new(Object::File(62596)),
            Box::new(Object::Dir(vec![Box::new(Object::File(584))])),
        ])),
        // d
        Box::new(Object::Dir(vec![
            Box::new(Object::File(4060174)),
            Box::new(Object::File(8033020)),
            Box::new(Object::File(5626152)),
            Box::new(Object::File(7214296)),
        ])),
    ]);
    assert_eq!(parse_tree(EXAMPLE_INPUT).unwrap(), expected);
}

#[test]
fn example_1() {
    let tree = parse_tree(EXAMPLE_INPUT).unwrap();
    assert_eq!(find_sum_of_max_size_directories(&tree, 100000), 95437);
}

#[test]
fn example_2() {
    let tree = parse_tree(EXAMPLE_INPUT).unwrap();
    assert_eq!(
        find_smallest_eligible_directory_to_delete(&tree).unwrap(),
        24933642
    );
}
