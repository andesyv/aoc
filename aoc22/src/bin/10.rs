use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Noop,
    Addx(i32),
}

fn map_cycle_time(op: Op) -> (Op, u32) {
    match op {
        Op::Noop => (Op::Noop, 1),
        Op::Addx(i) => (Op::Addx(i), 2),
    }
}

fn parse(input: &str) -> Vec<(Op, u32)> {
    use Op::*;
    input
        .lines()
        .filter_map(|l| match &l[..] {
            "noop" => Some(Noop),
            _ if l.starts_with("addx") => Some(Addx(l.split_once(' ')?.1.parse().ok()?)),
            _ => unimplemented!(),
        })
        .map(map_cycle_time)
        .collect()
}

fn process_operation(ops: &mut VecDeque<(Op, u32)>) -> i32 {
    if let Some(op) = ops.pop_front() {
        match op {
            (Op::Addx(x), t) => {
                if t < 2 {
                    return x;
                } else {
                    ops.push_front((Op::Addx(x), t - 1))
                }
            }
            _ => (),
        }
    }
    0
}

fn process_signal_strengths(input: &str) -> i32 {
    let mut ops = VecDeque::from(parse(input));
    let mut register = 1;
    let mut cycle = 1;
    let mut cycles_to_check = vec![220, 180, 140, 100, 60, 20];
    let mut ret = 0;
    while !ops.is_empty() {
        if let Some(checkpoint) = cycles_to_check.last().copied() {
            if checkpoint == cycle {
                // println!("Register is at {}", register);
                ret += checkpoint * register;
                cycles_to_check.pop();
            }
        }
        register += process_operation(&mut ops);
        cycle += 1;
    }
    ret
}

fn draw_image_from_instructions(input: &str) -> String {
    let mut ops = VecDeque::from(parse(input));
    let mut register: i32 = 1;
    let mut cycle = 1;
    let mut scanlines = Vec::new();
    let mut scanline = String::new();
    while !ops.is_empty() {
        let pixel_pos = (cycle - 1) % 40;
        if pixel_pos == 0 {
            // println!("{}", scanline);
            scanlines.push(scanline.clone());
            scanline.clear();
        }
        // println!("{}", scanline);
        // println!("Register: {}", register);
        scanline.push(if register.abs_diff(pixel_pos) < 2 {
            '#'
        } else {
            '.'
        });

        register += process_operation(&mut ops);
        cycle += 1;
    }
    scanlines.push(scanline);
    // "Draw"
    println!("{}", scanlines.join("\n"));
    scanlines.join("\n")
}

const SMALL_EXAMPLE: &str = "noop
addx 3
addx -5";

const LARGER_EXAMPLE: &str = include_str!("../inputs/10-example.txt");

fn main() {
    const INPUT: &str = include_str!("../inputs/10.txt");
    println!(
        "Sum of signal strengths: {}",
        process_signal_strengths(INPUT)
    );

    println!("Image from signal: ..");
    draw_image_from_instructions(INPUT);
}

#[test]
fn parse_test() {
    assert_eq!(
        parse(SMALL_EXAMPLE),
        vec![(Op::Noop, 1), (Op::Addx(3), 2), (Op::Addx(-5), 2)]
    );
}

#[test]
fn example_1() {
    assert_eq!(process_signal_strengths(LARGER_EXAMPLE), 13140);
}

#[test]
fn example_2() {
    let image = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
    assert_eq!(draw_image_from_instructions(LARGER_EXAMPLE), image);
}
