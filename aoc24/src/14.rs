use std::collections::HashSet;

type Vec2D = (i32, i32);

#[derive(Debug)]
struct Robot {
    pos: Vec2D,
    dir: Vec2D,
}

struct Grid {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl Grid {
    fn new(input: &str, width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            robots: parse(input),
        }
    }

    fn simulate_n_steps(&mut self, n: usize) {
        for _ in 0..n {
            for robot in self.robots.iter_mut() {
                *robot = simulate(robot, self.width, self.height)
            }
        }
    }

    fn calc_safety_factor(&self) -> usize {
        let horizontal_skip_axis = self.height / 2;
        let vertical_skip_axis = self.width / 2;

        let top_left = self.robots.iter().filter(|robot|robot.pos.0 < vertical_skip_axis && robot.pos.1 < horizontal_skip_axis).count();
        let top_right = self.robots.iter().filter(|robot|robot.pos.0 > vertical_skip_axis && robot.pos.1 < horizontal_skip_axis).count();
        let bottom_left = self.robots.iter().filter(|robot|robot.pos.0 < vertical_skip_axis && robot.pos.1 > horizontal_skip_axis).count();
        let bottom_right = self.robots.iter().filter(|robot|robot.pos.0 > vertical_skip_axis && robot.pos.1 > horizontal_skip_axis).count();

        top_left * top_right * bottom_left * bottom_right
    }

    fn calc_safety_factor_by_simulating(&mut self) -> usize {
        self.simulate_n_steps(100);
        // self.print();
        self.calc_safety_factor()
    }

    fn visualize_simulation(&mut self) {
        'outer: for i in 1..10001 {
            self.simulate_n_steps(1);

            // I guessed that the Advent of Code creators probably would use as much "canvas" as
            // possible to draw the Christmas tree. And to that end I guess that the likely
            // candidates had to have no overlapping robots. Seems I was correct :)
            let mut pos_count = HashSet::new();
            for robot in &self.robots {
                if !pos_count.insert(robot.pos) {
                    continue 'outer;
                }
            }

            println!("\n\n After {} steps:", i);
            self.print();
        }
    }

    fn print(&self) {
        // println!("Robots: {:?}", self.robots);
        // println!("Grid:");
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.robots.iter().filter(|robot| robot.pos == (x, y)).count();
                if count > 0 {
                    print!("{}", count);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn parse_robot(line: &str) -> Option<Robot> {
    let (_, rest) = line.split_once("=")?;
    let (a, rest) = rest.split_once(",")?;
    let (b, rest) = rest.split_once(" v=")?;
    let (c, d) = rest.split_once(",")?;

    Some(Robot{
        pos: (a.parse().ok()?, b.parse().ok()?),
        dir: (c.parse().ok()?, d.trim().parse().ok()?),
    })
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().filter_map(parse_robot).collect()
}

fn simulate(robot: &Robot, width: i32, height: i32) -> Robot {
    let x = (robot.pos.0 + robot.dir.0).rem_euclid(width);
    let y = (robot.pos.1 + robot.dir.1).rem_euclid(height);
    Robot {
        pos: (x, y),
        dir: robot.dir,
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/14.txt");
    println!("Safety factor after simulating for 100 seconds: {}", Grid::new(INPUT, 101, 103).calc_safety_factor_by_simulating());
    Grid::new(INPUT, 101, 103).visualize_simulation();
}

const EXAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

#[test]
fn count_simulated_robots_in_quadrants_in_example() {
    let mut grid = Grid::new(EXAMPLE_INPUT, 11, 7);
    assert_eq!(grid.calc_safety_factor_by_simulating(), 12);
}