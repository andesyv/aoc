use regex::{Captures, Regex};

/**
 * Today is an equation based problem
 * Given buttons A and B are 2D vectors, the linear equation to solve looks like this:
 * ```text
 * | A_x |       | B_x |        | P_x |
 * | A_y | * a + | B_y | * b =  | P_y |
 * ```
 * which can be expressed like this matrix equation:
 * ```text
 * | A_x B_x |   | a |   | P_x |
 * | A_y B_y | x | b | = | P_y |
 * ```
 * Most matrix equations Ax = y can be solved by doing Ax = y => A^-1 * A x = A^-1 y => x = A^-1 * y
 * Given that an inverse matrix exists, we know there's a possible way to solve the equation.
 * However, solving the equation via a matrix equation gives us the "optimal" solution given equal
 * weights. This is not really the case here as the buttons have different weights (although it
 * coincidentally works for the examples). So while we can use this as an initial guess,
 * we have to employ some heuristics to find the best result.
 *
 * Another solution given the same 2 equations is to simply solve using substitution:
 * ```text
 * A_x * a + B_x * b = P_x
 * A_y * a + B_y * b = P_y
 * =>
 * a = (P_x - B_x * b) / A_x
 * A_y * (P_x - B_x * b) / A_x + B_y * b = P_y
 * A_y * P_x / A_x - A_y * B_x * b / A_x + B_y * b = P_y
 * A_y * P_x / A_x + b * (B_y - A_y * B_x / A_x) = P_y
 * b = (P_y - P_x * A_y / A_x) / (B_y - B_x * A_y / A_x)
 * =>
 * A_x * a + B_x * b = P_x
 * a = (P_x - B_x * b) / A_x
 * ```
 * E.g. b = (5400 - 34 * 8400 / 94) / (67 - 34 * 22 / 94) = 40
 */

// type Mat2D = [[i32; 2]; 2];
// type FMat2D = [[f64; 2]; 2];
// type Vec2D = [i32; 2];
//
// fn adjugate(mat: Mat2D) -> FMat2D {
//     // a, b      d, -b
//     // c, d  ->  -c, a
//    [
//        [ f64::from(mat[1][1]), -f64::from(mat[0][1])],
//        [ -f64::from(mat[1][0]), f64::from(mat[0][0])],
//    ]
// }
//
// fn determinant(mat: Mat2D) -> f64 {
//     f64::from(mat[0][0]) * f64::from(mat[1][1]) - f64::from(mat[0][1]) * f64::from(mat[1][0])
// }
//
// fn inverse(mat: Mat2D) -> Option<FMat2D> {
//     let det = determinant(mat);
//     if det.abs() < 0.001 {
//         return None;
//     }
//
//     return Some(adjugate(mat) / det);
// }

type Vec2D = (u64, u64);

#[derive(Debug)]
struct ClawMachine {
    a_button: Vec2D,
    b_button: Vec2D,
    prize: Vec2D,
}

fn parse_vec2d_from_captures(captures: Captures) -> Option<Vec2D> {
    let x = captures.get(1)?.as_str().parse().ok()?;
    let y = captures.get(2)?.as_str().parse().ok()?;
    Some((x, y))
}

fn parse_chunk(chunk: &str) -> Option<ClawMachine> {
    let button_reg = Regex::new(r"Button [AB]: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_reg = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let mut button_it = button_reg.captures_iter(chunk);
    let a_button = button_it.next().map(parse_vec2d_from_captures).flatten()?;
    let b_button = button_it.next().map(parse_vec2d_from_captures).flatten()?;
    let prize = prize_reg
        .captures(chunk)
        .map(parse_vec2d_from_captures)
        .flatten()?;
    Some(ClawMachine {
        a_button,
        b_button,
        prize,
    })
}

fn parse(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();

    for chunk in input.trim().split("\n\n") {
        if let Some(m) = parse_chunk(chunk) {
            machines.push(m);
        }
    }

    machines
}

fn calc_start_b(a: Vec2D, b: Vec2D, goal: Vec2D) -> Option<u64> {
    // b = (P_y - P_x * A_y / A_x) / (B_y - B_x * A_y / A_x)
    // b = (goal.1 - goal.0 * a_inv) / (b.1 - b.0 * a_inv)
    let a_inv = a.1 as f64 / a.0 as f64;
    let result =
        (goal.1 as f64 - goal.0 as f64 * a_inv) / (b.1 as f64 - b.0 as f64 * a_inv);
    // The as keyword does "safe" saturating casts from floating point to integer. Which
    // unfortunately leaves us in an awkward spot where we have to cast to an intermediate i128 to
    // know if the u64 is within bounds using i128::try_into.
    (result.round() as i128).try_into().ok()
}

fn find_a_from_b(a_vec: Vec2D, b_vec: Vec2D, goal: Vec2D, b_factor: u64) -> Option<u64> {
    // a = (P_x - B_x * b) / A_x
    let x = u64::try_from(
        ((goal.0 as f64 - b_vec.0 as f64 * b_factor as f64) / a_vec.0 as f64)
            .round() as i128,
    )
    .ok()?;
    // Now check if the equation resolves back into the solution. If not, it's unsolvable.
    if a_vec.0 * x + b_vec.0 * b_factor != goal.0 || a_vec.1 * x + b_vec.1 * b_factor != goal.1 {
        return None;
    }

    Some(x)
}

fn find_best_a_and_b(machine: &ClawMachine) -> Option<(u64, u64)> {
    let initial_b = calc_start_b(machine.a_button, machine.b_button, machine.prize)?;
    // Optimization: There should be no possible solutions if the equations failed. I think...
    let initial_a = find_a_from_b(machine.a_button, machine.b_button, machine.prize, initial_b)?;
    let mut current_lowest = (initial_a, initial_b);
    let mut current_lowest_cost = initial_a * 3 + initial_b;

    // Search first in positive b direction:
    for i in 0..100 {
        let b = initial_b + i;
        if b > 100 {
            break;
        }
        if let Some(a) = find_a_from_b(machine.a_button, machine.a_button, machine.prize, b) {
            if a > 100 {
                continue;
            }
            let cost = a * 3 + b;
            if cost < current_lowest_cost {
                current_lowest_cost = cost;
                current_lowest = (a, b);
            }
        }
    }

    // Then in negative b direction:
    for i in 0..100 {
        let b = initial_b.saturating_sub(i);
        if b > 100 {
            break;
        }
        if let Some(a) = find_a_from_b(machine.a_button, machine.a_button, machine.prize, b) {
            if a > 100 {
                continue;
            }
            let cost = a * 3 + b;
            if cost < current_lowest_cost {
                current_lowest_cost = cost;
                current_lowest = (a, b);
            }
        }
    }

    Some(current_lowest)
}

// This function cannot guarantee it has found the best answer, but it should be really close :)
fn guesstimate_best_a_and_b(machine: &ClawMachine) -> Option<(u64, u64)> {
    let initial_b = calc_start_b(machine.a_button, machine.b_button, machine.prize)?;
    // Optimization: There should be no possible solutions if the equations failed. I think...
    let initial_a = find_a_from_b(machine.a_button, machine.b_button, machine.prize, initial_b)?;
    let mut current_lowest = (initial_a, initial_b);
    let mut current_lowest_cost = initial_a * 3 + initial_b;

    let mut previous_cost = current_lowest_cost;
    // This counter increases every time the cost has increased from the previous one. It's a
    // heuristic we can use to determine that we are getting further away from the solution space.
    let mut diverging_counter = 0;

    const SEARCH_WIDTH: u64 = 1000;

    // Search first in positive b direction:
    for i in 0..SEARCH_WIDTH {
        let b = initial_b + i;
        if let Some(a) = find_a_from_b(machine.a_button, machine.a_button, machine.prize, b) {
            let cost = a * 3 + b;
            if cost < current_lowest_cost {
                current_lowest_cost = cost;
                current_lowest = (a, b);
                diverging_counter = 0;
            } else if cost > previous_cost {
                diverging_counter += 1;
            }

            previous_cost = cost;
        } else {
            diverging_counter += 1;
        }

        // When we've been diverging for 100 iterations, we probably won't get any better results.
        // break
        if diverging_counter > 100 { break; }
    }

    previous_cost = current_lowest_cost;
    diverging_counter = 0;

    // Then in negative b direction:
    for i in 0..SEARCH_WIDTH {
        let b = initial_b.saturating_sub(i);
        if let Some(a) = find_a_from_b(machine.a_button, machine.a_button, machine.prize, b) {
            let cost = a * 3 + b;
            if cost < current_lowest_cost {
                current_lowest_cost = cost;
                current_lowest = (a, b);
                diverging_counter = 0;
            } else if cost > previous_cost {
                diverging_counter += 1;
            }

            previous_cost = cost;
        } else {
            diverging_counter += 1;
        }

        // When we've been diverging for 100 iterations, we probably won't get any better results.
        // break
        if diverging_counter > 100 { break; }
    }

    Some(current_lowest)
}

fn find_min_required_tokens(input: &str) -> u64 {
    let machines = parse(input);
    let mut sum = 0;

    for machine in machines {
        if let Some((a, b)) = find_best_a_and_b(&machine) {
            sum += a * 3 + b;
        }
    }

    sum
}

fn find_min_required_tokens_with_huge_numbers(input: &str) -> u64 {
    let machines: Vec<_> = parse(input)
        .into_iter()
        .map(|machine| ClawMachine {
            a_button: machine.a_button,
            b_button: machine.b_button,
            prize: (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            ),
        })
        .collect();
    let mut sum = 0;

    for machine in machines {
        if let Some((a, b)) = guesstimate_best_a_and_b(&machine) {
            sum += a * 3 + b;
        }
    }

    sum
}

fn main() {
    const INPUT: &str = include_str!("../inputs/13.txt");
    println!(
        "Min tokens required to get all prizes: {}",
        find_min_required_tokens(INPUT)
    );

    println!(
        "Min tokens required to get all prizes when the prize numbers are huge!: {}",
        find_min_required_tokens_with_huge_numbers(INPUT)
    );
}

const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[test]
fn parse_test() {
    let expected = vec![
        ClawMachine {
            a_button: (94, 34),
            b_button: (22, 67),
            prize: (8400, 5400),
        },
        ClawMachine {
            a_button: (26, 66),
            b_button: (67, 21),
            prize: (12748, 12176),
        },
        ClawMachine {
            a_button: (17, 86),
            b_button: (84, 37),
            prize: (7870, 6450),
        },
        ClawMachine {
            a_button: (69, 23),
            b_button: (27, 71),
            prize: (18641, 10279),
        },
    ];

    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", parse(&EXAMPLE_INPUT))
    );
}

#[test]
fn find_best_combination_test() {
    let expected: (u64, u64) = (80, 40);
    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (94, 34),
        b_button: (22, 67),
        prize: (8400, 5400),
    });
    assert_eq!(sut.unwrap(), expected);

    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (26, 66),
        b_button: (67, 21),
        prize: (12748, 12176),
    });
    assert!(sut.is_none());

    let expected: (u64, u64) = (38, 86);
    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (17, 86),
        b_button: (84, 37),
        prize: (7870, 6450),
    });
    assert_eq!(sut.unwrap(), expected);

    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (69, 23),
        b_button: (27, 71),
        prize: (18641, 10279),
    });
    assert!(sut.is_none());
}

#[test]
fn example_input() {
    assert_eq!(find_min_required_tokens(EXAMPLE_INPUT), 480);
}

#[test]
fn find_best_combination_with_huge_numbers_test() {
    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (94, 34),
        b_button: (22, 67),
        prize: (8400 + 10000000000000, 5400 + 10000000000000),
    });
    assert!(sut.is_none());

    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (26, 66),
        b_button: (67, 21),
        prize: (12748 + 10000000000000, 12176 + 10000000000000),
    });
    assert!(sut.is_some());

    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (17, 86),
        b_button: (84, 37),
        prize: (7870 + 10000000000000, 6450 + 10000000000000),
    });
    assert!(sut.is_none());

    let sut = find_best_a_and_b(&ClawMachine {
        a_button: (69, 23),
        b_button: (27, 71),
        prize: (18641 + 10000000000000, 10279 + 10000000000000),
    });
    assert!(sut.is_some());
}
