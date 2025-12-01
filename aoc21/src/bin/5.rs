use std::collections::HashMap;

type Point = (u32, u32);

fn main() {
    const INPUT: &str = include_str!("../inputs/5.txt");
    // const INPUT: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

    let lines: Vec<(Point, Point)> = INPUT.lines().filter_map(|s|{
        let parse_point = |s: &str| -> Option<Point> {
            if let [l,r,..] = &s.split(',').collect::<Vec<&str>>()[..] {
                if let (Ok(l), Ok(r)) = (l.parse::<u32>(), r.parse::<u32>()) {
                    return Some((l, r))
                }
            }
            None
        };
        if let [a,b,..] = &s.split(" -> ").collect::<Vec<&str>>()[..] {
            if let (Some(a), Some(b)) = (parse_point(a), parse_point(b)) {
                return Some((a, b))
            }
        }
        None
    }).collect();

    let mut points: HashMap<Point, usize> = HashMap::new();
    for line in &lines {
        for p in make_line(line.0, line.1, false) {
            let p_count = points.entry(p).or_insert(0);
            *p_count += 1;
        }
    }

    println!("Part 1: Number of points where atlest 2 lines overlap: {}", points.iter().filter(|(_, n)|1 < **n).count());


    let mut points: HashMap<Point, usize> = HashMap::new();
    for line in &lines {
        for p in make_line(line.0, line.1, true) {
            let p_count = points.entry(p).or_insert(0);
            *p_count += 1;
        }
    }

    println!("Part 2: Number of points where atlest 2 lines overlap (with diagonals): {}", points.iter().filter(|(_, n)|1 < **n).count());
}

fn make_line(a: Point, b: Point, diagonals: bool) -> Vec<Point> {
    let mut line = Vec::new();
    let make_range = |first,last| -> Vec<u32> {if first <= last { (first..=last).collect() } else { (last..=first).rev().collect() }};

    if a.0 == b.0 || a.1 == b.1 {
        if a.0 != b.0 {
            for i in make_range(a.0, b.0) {
                line.push((i, a.1))
            }
        } else {
            for i in make_range(a.1, b.1) {
                line.push((a.0, i))
            }
        }
    } else if diagonals {
        for (i, j) in (make_range(a.0,b.0).into_iter()).zip(make_range(a.1, b.1)) {
            line.push((i, j))
        }
    }
    line
}