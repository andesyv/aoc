use std::collections::VecDeque;

enum SimulationResult {
    FlashCount(u32),
    Synchronized(u32)
}

fn main() {
//     const INPUT: &str = "5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526";
    const INPUT: &str = include_str!("../inputs/11.txt");

    let octopuses: Vec<Vec<u32>> = INPUT.lines().map(|l|l.chars().map(|c|c.to_digit(10).unwrap()).collect()).collect();
    for simulation_steps in [100, 1000] {
        let results = simulate(octopuses.clone(), simulation_steps);
        match results {
            SimulationResult::FlashCount(flash_count) => println!("Flash count: {}", flash_count),
            SimulationResult::Synchronized(step) => println!("Syncronized on step {}", step)
        }
    }
}

fn simulate(mut octopi: Vec<Vec<u32>>, n: u32) -> SimulationResult {
    let mut flash_count = 0;
    for m in 0..n {
        let mut flash_stack: VecDeque<(usize, usize)> = VecDeque::new();
        let mut done_flash: Vec<(usize, usize)> = Vec::new();

        for (j, xs) in octopi.iter_mut().enumerate() {
            for (i, octopus) in xs.iter_mut().enumerate() {
                *octopus += 1;
                if 9 < *octopus {
                    flash_stack.push_front((i, j));
                }
            }
        }

        while let Some(coord) = flash_stack.pop_back() {
            let surrounding = get_surrounding_coords(coord);
            for neighbour in surrounding {
                if let Some((i, j)) = neighbour {
                    if let Some(xs) = octopi.get_mut(j) {
                        if let Some(octopus) = xs.get_mut(i) {
                            *octopus += 1;
                            if 10 == *octopus {
                                flash_stack.push_front((i, j));
                            }
                        }
                    }
                }
            }
            done_flash.push(coord);
            flash_count += 1;
        }

        if done_flash.len() == octopi.iter().map(|ls|ls.len()).sum() {
            return SimulationResult::Synchronized(m + 1);
        }

        for (i, j) in done_flash {
            if let Some(xs) = octopi.get_mut(j) {
                if let Some(octopus) = xs.get_mut(i) {
                    *octopus = 0;
                }
            }
        }
    }
    SimulationResult::FlashCount(flash_count)
}

fn get_surrounding_coords((i, j): (usize, usize)) -> [Option<(usize, usize)>; 8] {
    let mut k = 0;
    let mut out = [None; 8];
    for y in -1..2 {
        for x in -1..2 {
            if x == 0 && y == 0 {
                continue;
            }
            if let (Ok(a), Ok(b)) = (i32::try_from(i), i32::try_from(j)) {
                if let (Ok(a), Ok(b)) = (usize::try_from(a + x), usize::try_from(b + y)) {
                    out[k] = Some((a, b));
                    k += 1;
                }
            }
        }
    }
    out
}