use std::{
    char::MAX,
    collections::{HashMap, HashSet},
};

use itertools::{Itertools, MinMaxResult};

#[derive(Clone)]
enum Tile {
    Air,
    Rock,
    Sand,
    Source,
}

type Coord = (i64, i64);
type Grid = (HashMap<Coord, Tile>, Option<i64>);

fn parse_path((a, b): (&str, &str)) -> Vec<Coord> {
    let start: Coord = a
        .split_once(',')
        .and_then(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
        .unwrap();
    let end: Coord = b
        .split_once(',')
        .and_then(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
        .unwrap();

    if start.0.abs_diff(end.0) != 0 {
        (if start.0 <= end.0 {
            start.0..=end.0
        } else {
            end.0..=start.0
        })
        .map(|i| (i, start.1))
        .collect()
    } else {
        (if start.1 <= end.1 {
            start.1..=end.1
        } else {
            end.1..=start.1
        })
        .map(|i| (start.0, i))
        .collect()
    }
}

fn parse(input: &str) -> Grid {
    let rock_coords: HashSet<Coord> = input
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .tuple_windows()
                .flat_map(|p| parse_path(p).into_iter())
        })
        .collect();

    let mut grid = HashMap::new();
    for coord in rock_coords {
        grid.insert(coord, Tile::Rock);
    }

    grid.insert((500, 0), Tile::Source);

    (grid, None)
}

fn get_grid_dims(grid: &Grid) -> ((usize, usize), Coord) {
    match (
        grid.0.keys().map(|(x, _)| x).minmax(),
        grid.0.keys().map(|(_, y)| y).minmax(),
    ) {
        (MinMaxResult::MinMax(xmin, xmax), MinMaxResult::MinMax(ymin, ymax)) => (
            (
                (xmax - xmin + 1).try_into().unwrap(),
                (ymax - ymin + 1).try_into().unwrap(),
            ),
            (*xmin, *ymin),
        ),
        _ => panic!("Has to have a min and a max in order to make a grid"),
    }
}

fn visualize_grid(grid: &Grid) -> String {
    let mut lines = Vec::new();
    let mut line = String::new();
    let (grid_dims, grid_offset) = get_grid_dims(grid);

    for y in 0..grid_dims.1 {
        for x in 0..grid_dims.0 {
            let tile = grid
                .0
                .get(&(
                    i64::try_from(x).unwrap() + grid_offset.0,
                    i64::try_from(y).unwrap() + grid_offset.1,
                ))
                .unwrap_or(&Tile::Air);
            line.push(match tile {
                Tile::Air => '.',
                Tile::Rock => '#',
                Tile::Sand => 'O',
                Tile::Source => '+',
            });
        }
        lines.push(line.clone());
        line.clear();
    }
    lines.join("\n")
}

fn draw_grid(grid: &Grid) {
    println!("{}\n", visualize_grid(grid));
}

fn get_tile_at_pos(pos: &Coord, grid: &Grid) -> Option<Tile> {
    let get_floor = || {
        grid.1.and_then(|floor_y| {
            if floor_y <= pos.1 {
                Some(Tile::Rock)
            } else {
                None
            }
        })
    };
    grid.0.get(pos).cloned().or_else(get_floor)
}

fn get_next_pos(current: &Coord, grid: &Grid) -> Option<Coord> {
    const AVAILABLE_DIRS: [Coord; 3] = [(0, 1), (-1, 1), (1, 1)];
    for dir in AVAILABLE_DIRS {
        let next_pos = (current.0 + dir.0, current.1 + dir.1);
        if let Tile::Air = get_tile_at_pos(&next_pos, grid).unwrap_or(Tile::Air) {
            return Some(next_pos);
        }
    }
    None
}

fn simulate(grid: &mut Grid) -> usize {
    const MAX_STEPS: usize = 100000;
    let mut sand_counter = 0;
    'outer: loop {
        let mut sand_pos = (500, 0);
        // draw_grid(grid);
        for _ in 0..MAX_STEPS {
            match get_next_pos(&sand_pos, grid) {
                Some(pos) => sand_pos = pos,
                None => {
                    grid.0.insert(sand_pos, Tile::Sand);
                    sand_counter += 1;
                    if sand_pos == (500, 0) {
                      break;
                    }
                    
                    continue 'outer;
                }
            }
        }
        return sand_counter;
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/14.txt");
    let mut grid = parse(INPUT);
    println!("Initial grid looks like this:");
    draw_grid(&grid);
    println!(
        "After all sand has settled, there's {} still units of sand",
        simulate(&mut grid)
    );
    draw_grid(&grid);

    println!("After adding a floor...");
    let max_y = grid.0.keys().map(|(_, y)| *y).max().unwrap();
    grid.1 = Some(max_y + 2);

    println!(
        "After all sand has settled, there's {} still units of sand",
        simulate(&mut grid)
    );
    draw_grid(&grid);
}

const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

#[test]
fn parse_test() {
    let expected = "......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.";
    assert_eq!(visualize_grid(&parse(EXAMPLE_INPUT)), expected);
}

#[test]
fn example_1() {
    assert_eq!(simulate(&mut parse(EXAMPLE_INPUT)), 24);
}

#[test]
fn example_2() {
    let mut grid = parse(EXAMPLE_INPUT);
    let max_y = grid.0.keys().map(|(_, y)| *y).max().unwrap();
    grid.1 = Some(max_y + 2);

    assert_eq!(simulate(&mut grid), 93);
}
