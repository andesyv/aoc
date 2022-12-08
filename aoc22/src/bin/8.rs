type Grid = Vec<Vec<u32>>;

fn parse_forest(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn get_value_at((x, y): (i64, i64), grid: &Grid) -> Option<u32> {
    let i: usize = x.try_into().ok()?;
    let j: usize = y.try_into().ok()?;
    grid.get(j).map(|row| row.get(i).copied()).flatten()
}

fn is_visible((x, y): (u32, u32), grid: &Grid) -> bool {
    if let Some(current_tree) = get_value_at((x.into(), y.into()), grid) {
        'direction: for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let mut i = 1;
            while let Some(other_tree) =
                get_value_at((i64::from(x) + dx * i, i64::from(y) + dy * i), grid)
            {
                if other_tree >= current_tree {
                    continue 'direction;
                }
                i += 1;
            }
            return true;
        }
    }
    false
}

fn count_visible(grid: &Grid) -> u32 {
    let mut sum_visible = 0;
    for (j, row) in grid.iter().enumerate() {
        for (i, _) in row.iter().enumerate() {
            if is_visible((i.try_into().unwrap(), j.try_into().unwrap()), &grid) {
                sum_visible += 1;
            }
        }
    }
    sum_visible
}

fn calculate_scenic_score((x, y): (u32, u32), grid: &Grid) -> i64 {
    let mut score = 1;
    if let Some(current_tree) = get_value_at((x.into(), y.into()), grid) {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let mut i = 1;
            'inner: while let Some(other_tree) =
                get_value_at((i64::from(x) + dx * i, i64::from(y) + dy * i), grid)
            {
                i += 1;
                if other_tree >= current_tree {
                    break 'inner;
                }
            }
            score *= i - 1;
        }
        score
    } else {
        0 // This is the ground. There's no trees here
    }
}

fn find_highest_scenic_score(grid: &Grid) -> Option<i64> {
    grid.iter()
        .enumerate()
        .filter_map(|(j, row)| {
            row.iter()
                .enumerate()
                .map(|(i, _)| {
                    calculate_scenic_score((i.try_into().unwrap(), j.try_into().unwrap()), &grid)
                })
                .max()
        })
        .max()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/8.txt");
    let forest = parse_forest(INPUT);
    println!(
        "There's {} visible trees from outside the grid.",
        count_visible(&forest)
    );

    println!(
        "The optimal tree has a scenic score of {}",
        find_highest_scenic_score(&forest).unwrap()
    );
}

const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

#[test]
fn test_visible() {
    let forest = parse_forest(EXAMPLE_INPUT);
    assert!(is_visible((1, 1), &forest));
    assert!(!is_visible((2, 2), &forest));
}

#[test]
fn example_1() {
    let forest = parse_forest(EXAMPLE_INPUT);
    assert_eq!(count_visible(&forest), 21);
}

#[test]
fn example_2() {
    let forest = parse_forest(EXAMPLE_INPUT);
    assert_eq!(calculate_scenic_score((2, 1), &forest), 4);
    assert_eq!(calculate_scenic_score((2, 3), &forest), 8);
    assert_eq!(find_highest_scenic_score(&forest).unwrap(), 8);
}
