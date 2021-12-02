fn main() {
    const INPUT: &str = include_str!("../inputs/2.txt");
    let (x, y) = INPUT
        .lines()
        .map(cmd_to_coord)
        .fold((0, 0), |l, r| (l.0 + r.0, l.1 + r.1));
    println!(
        "Part 1: Final horizontal position multiplication: {}",
        x * y
    );

    let (x, y, ..) = INPUT
        .lines()
        .map(|s| {
            let (x, y) = cmd_to_coord(s);
            (x, y, 0)
        })
        .fold((0, 0, 0), |l, r| (l.0 + r.0, l.1 + l.2 * r.0, l.2 + r.1));
    println!(
        "Part 2: Final horizontal position multiplication: {}",
        x * y
    );
}

fn cmd_to_coord(cmd: &str) -> (i32, i32) {
    let arr: Vec<&str> = cmd.split_whitespace().collect();
    match arr[..] {
        [dir, amount] => {
            let num = amount.parse::<i32>().unwrap();
            match dir {
                "forward" => (num, 0),
                "down" => (0, num),
                "up" => (0, -num),
                _ => (0, 0),
            }
        }
        _ => (0, 0),
    }
}
