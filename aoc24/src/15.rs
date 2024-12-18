use std::collections::HashMap;
// "GPS" of box = 100 * y + x (0 indexed)
// Part 1 asks for sum of GPS of boxes

type Pos = (u32, u32);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
enum Token {
    Wall,
    Box,
    WideBoxL,
    WideBoxR,
    // Space,
}

struct Grid {
    tokens: HashMap<Pos, Token>,
    width: u32,
    height: u32,
    robot: Pos,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut tokens = HashMap::new();
        let mut robot = (0, 0);

        let mut height = 0;
        let mut width = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x as u32, y as u32);
                match c {
                    '#' => {
                        tokens.insert(pos, Token::Wall);
                    }
                    'O' => {
                        tokens.insert(pos, Token::Box);
                    }
                    '[' => {
                        tokens.insert(pos, Token::WideBoxL);
                    }
                    ']' => {
                        tokens.insert(pos, Token::WideBoxR);
                    }
                    '@' => {
                        robot = pos;
                    }
                    _ => {}
                }
                width = (x + 1) as u32;
            }
            height = (y + 1) as u32;
        }

        Grid {
            tokens,
            width,
            height,
            robot,
        }
    }

    fn can_move(&self, pos: Pos, direction: Direction, prev_pos: Pos) -> bool {
        match self.tokens.get(&pos) {
            Some(Token::Wall) => return false, // Can't move a wall
            None => return true,               // Air can always be moved
            _ => (),
        };

        if let Some(next_pos) = get_next_pos(self, pos, direction) {
            // If moving vertically, we gotta check whether the other box's position can be moved
            // before we try moving it. Otherwise, we can end up with a box that has split in two
            // as one part could be moved but the other could not.
            let maybe_other_pos = if direction == Direction::Up || direction == Direction::Down {
                get_wide_box_other_pos(self, pos)
            } else {
                None
            };
            if let Some(other_pos) = maybe_other_pos {
                if prev_pos != other_pos {
                    // To prevent an infinite recursion loop...
                    if !self.can_move(other_pos, direction, pos) {
                        return false;
                    }
                }
            }

            self.can_move(next_pos, direction, pos)
        } else {
            false // Things cannot be moved outside the grid either
        }
    }

    fn try_move(&mut self, pos: Pos, direction: Direction) -> bool {
        match self.tokens.get(&pos) {
            Some(Token::Wall) => return false, // Can't move a wall
            None => return true,               // Air can always be moved
            _ => (),
        };

        if let Some(next_pos) = get_next_pos(self, pos, direction) {
            // If moving vertically, we gotta check whether the other box's position can be moved before we try moving it.
            // Otherwise, we can end up with a box that has split in two as one part could be moved but the other could not.
            let maybe_other_pos = if direction == Direction::Up || direction == Direction::Down {
                get_wide_box_other_pos(self, pos)
            } else {
                None
            };
            if let Some(other_pos) = maybe_other_pos {
                if !self.can_move(other_pos, direction, pos) {
                    return false;
                }
            }

            let next_was_moved = self.try_move(next_pos, direction);
            if next_was_moved {
                let token = self
                    .tokens
                    .remove(&pos)
                    .expect("Token was moved from somewhere else");
                if self.tokens.insert(next_pos, token).is_some() {
                    panic!("Token was moved onto existing token");
                }

                // If we previously determined the box was part of a wider box, and moving it was
                // fine, do so now.
                if let Some(other_pos) = maybe_other_pos {
                    if !self.try_move(other_pos, direction) {
                        self.print();
                        panic!("It was already detected that moving the box should be no issue, but attempting to do so caused the box to not move. The original position was {:?} and the failing position was {:?}", pos, other_pos);
                    }
                }

                return true;
            }
        }

        false // Things cannot be moved outside the grid either
    }

    fn simulate(&mut self, direction: Direction) {
        let next_robot_pos = get_next_pos(self, self.robot, direction).unwrap();
        if self.try_move(next_robot_pos, direction) {
            self.robot = next_robot_pos;
        }
    }

    fn calc_gps_sum(&self) -> u32 {
        self.tokens
            .iter()
            .filter_map(|(pos, token)| match token {
                Token::Box => Some(100 * pos.1 + pos.0),
                _ => None,
            })
            .sum()
    }

    fn calc_gps_sum_for_wide_boxes(&self) -> u32 {
        self.tokens
            .iter()
            .filter_map(|(pos, token)| match token {
                Token::WideBoxL => Some(100 * pos.1 + pos.0),
                _ => None,
            })
            .sum()
    }

    fn print(&self) {
        println!("Grid:");
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(token) = self.tokens.get(&(x, y)) {
                    match token {
                        Token::Box => print!("O"),
                        Token::WideBoxL => print!("["),
                        Token::WideBoxR => print!("]"),
                        Token::Wall => print!("#"),
                    }
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }
}

fn parse(input: &str, widen: bool) -> Result<(Grid, Vec<Direction>), &'static str> {
    let (grid_str, directions_str) = input
        .trim()
        .split_once("\n\n")
        .ok_or("Could not split grid and directions")?;
    let grid = if widen {
        let wide_grid = widen_input_grid(grid_str.trim());
        Grid::new(wide_grid.as_str())
    } else {
        Grid::new(grid_str.trim())
    };
    let mut directions = Vec::with_capacity(directions_str.len());
    for c in directions_str.chars() {
        match c {
            '^' => directions.push(Direction::Up),
            '>' => directions.push(Direction::Right),
            'v' => directions.push(Direction::Down),
            '<' => directions.push(Direction::Left),
            _ => (),
        }
    }
    Ok((grid, directions))
}

fn get_next_pos(grid: &Grid, pos: Pos, direction: Direction) -> Option<Pos> {
    let new_pos = match direction {
        Direction::Up => (pos.0, pos.1.wrapping_sub(1)),
        Direction::Right => (pos.0 + 1, pos.1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0.wrapping_sub(1), pos.1),
    };

    if new_pos.0 < grid.width && new_pos.1 < grid.height {
        Some(new_pos)
    } else {
        None
    }
}

fn get_wide_box_other_pos(grid: &Grid, pos: Pos) -> Option<Pos> {
    match grid.tokens.get(&pos) {
        Some(Token::WideBoxL) => get_next_pos(grid, pos, Direction::Right),
        Some(Token::WideBoxR) => get_next_pos(grid, pos, Direction::Left),
        _ => None,
    }
}

fn get_gps_sum_after_navigating_directions(input: &str) -> u32 {
    let (mut grid, directions) = parse(input, false).unwrap();
    for direction in directions {
        grid.simulate(direction);
    }
    grid.calc_gps_sum()
}

fn widen_input_grid(input_grid: &str) -> String {
    let mut result = String::with_capacity(input_grid.len() * 2);
    for line in input_grid.lines() {
        for c in line.chars() {
            match c {
                '#' => result.push_str("##"),
                'O' => result.push_str("[]"),
                '@' => result.push_str("@."),
                '.' => result.push_str(".."),
                _ => (),
            }
        }
        result.push('\n');
    }
    // Pop the last \n
    result.pop();

    result
}

fn get_gps_sum_after_navigating_directions_with_wide_map(input: &str) -> u32 {
    let (mut grid, directions) = parse(input, true).unwrap();
    for direction in directions {
        grid.simulate(direction);
    }

    // grid.print();

    grid.calc_gps_sum_for_wide_boxes()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/15.txt");
    println!(
        "Sum of GPS after simulation: {}",
        get_gps_sum_after_navigating_directions(INPUT)
    );

    println!(
        "Sum of GPS after simulation using a wide map: {}",
        get_gps_sum_after_navigating_directions_with_wide_map(INPUT)
    );
}

const SMALL_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const BIG_EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

#[test]
fn test_parse_small_example() {
    let (grid, directions) = parse(SMALL_EXAMPLE, false).unwrap();

    // Check grid fields
    assert_eq!(grid.width, 8);
    assert_eq!(grid.height, 8);
    assert_eq!(grid.robot, (2, 2));

    // Check if specific boxes are correctly identified
    assert_eq!(grid.tokens.get(&(0, 0)).unwrap(), &Token::Wall);
    assert_eq!(grid.tokens.get(&(3, 1)).unwrap(), &Token::Box);
    assert_eq!(grid.tokens.get(&(4, 2)).unwrap(), &Token::Box);

    // Check the parsed directions
    let expected_directions = vec![
        Direction::Left,
        Direction::Up,
        Direction::Up,
        Direction::Right,
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Down,
        Direction::Left,
        Direction::Down,
        Direction::Right,
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Left,
    ];

    assert_eq!(directions, expected_directions);
}

#[test]
fn calculate_gps_sum_on_small_example() {
    assert_eq!(get_gps_sum_after_navigating_directions(SMALL_EXAMPLE), 2028);
}

#[test]
fn calculate_gps_sum_on_big_example() {
    assert_eq!(get_gps_sum_after_navigating_directions(BIG_EXAMPLE), 10092);
}

#[test]
fn widen_input_test() {
    let (grid_str, _) = BIG_EXAMPLE.split_once("\n\n").unwrap();
    let expected = "####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################";

    assert_eq!(widen_input_grid(grid_str.trim()), expected);
}

#[test]
fn calculate_gps_sum_on_complex_example_with_wide_map() {
    let EXAMPLE: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    // 100 + 5 + 200 + 7 + 300 + 6 = 616
    assert_eq!(
        get_gps_sum_after_navigating_directions_with_wide_map(EXAMPLE),
        618
    );
}

#[test]
fn calculate_gps_sum_on_big_example_with_wide_map() {
    assert_eq!(
        get_gps_sum_after_navigating_directions_with_wide_map(BIG_EXAMPLE),
        9021
    );
}
