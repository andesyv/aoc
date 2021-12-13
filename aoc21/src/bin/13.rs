use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    Horizontal(u32),
    Vertical(u32)
}

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Fold>) {
    let dots: HashSet<(u32, u32)> = input.lines().take_while(|s|s.contains(',')).map(|s|{
        let (a, b) = s.split_once(',').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect();

    let instructions: Vec<Fold> = input.lines().skip_while(|s|s.contains(',')).filter_map(|s|{
        if let Some(i) = s.find(char::is_numeric) {
            if let Ok(parsed) = &s[i..].parse() {
                return if s.contains('y') {
                    Some(Fold::Horizontal(*parsed))
                } else {
                    Some(Fold::Vertical(*parsed))
                }
            }
        }
        None
    }).collect();

    (dots, instructions)
}

#[test]
fn test1() {
    const INPUT: &str = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5";
    let (mut dots, instructions) = parse_input(INPUT);
    println!("Dots before first instruction:\n{}", gridify(&dots));
    dots = fold_paper(&dots, instructions.first().unwrap());
    println!("Dots after first instruction:\n{}", gridify(&dots));
    println!("... which is {} dots", dots.len());
    assert_eq!(dots.len(), 17)
}

#[test]
fn test2() {
    const INPUT: &str = include_str!("../inputs/13.txt");
    let (mut dots, instructions) = parse_input(INPUT);
    for instruction in instructions {
        dots = fold_paper(&dots, &instruction);
    }
    assert_eq!(dots.len(), 98)
}

fn main() {
    const INPUT: &str = include_str!("../inputs/13.txt");
    let (mut dots, instructions) = parse_input(INPUT);
    for instruction in instructions {
        dots = fold_paper(&dots, &instruction);
    }
    println!("Dots after all instructions:\n{}", gridify(&dots));
    println!("... which is {} dots", dots.len());
}

fn fold_paper(dots: &HashSet<(u32, u32)>, instruction: &Fold) -> HashSet<(u32, u32)> {
    let mut lefts: HashSet<(u32, u32)> = dots.iter().filter(|&(x,y)|{
        match instruction {
            Fold::Horizontal(line) => y < &line,
            Fold::Vertical(line) => x < &line
        }
    }).cloned().collect();

    lefts.extend(dots.iter().filter_map(|&(x,y)|{
        match instruction {
            // Reflection of y: y => y - 2 * diff = y - 2 * (y - line) = y - 2y + 2line = 2line - y
            Fold::Horizontal(line) => if *line < y { Some((x, line + line - y)) } else { None }
            Fold::Vertical(line) => if *line < x { Some((line + line - x, y)) } else { None }
        }
    }));
    lefts
}

fn gridify(dots: &HashSet<(u32, u32)>) -> String {
    let max_x = dots.iter().map(|t|t.0).max().unwrap();
    let max_y = dots.iter().map(|t|t.1).max().unwrap();

    let mut output = String::new();
    for y in 0..=max_y {
        output.extend((0..=max_x).into_iter().map(|x|if dots.contains(&(x, y)) { '#' } else { '.' }));
        output.push('\n');
    }
    output
}