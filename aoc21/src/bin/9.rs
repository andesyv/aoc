use std::collections::{HashSet, VecDeque};

type Coord = (i32, i32);

fn main() {
    // const INPUT: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
    const INPUT: &str = include_str!("../inputs/9.txt");

    let grid: Vec<Vec<u8>> = INPUT.lines().map(|s|s.chars().map(|c|c.to_digit(10).unwrap().try_into().unwrap()).collect()).collect();

    let mut basins: Vec<HashSet<Coord>> = Vec::new();
    let mut sum: u32 = 0;
    for (j, xs) in grid.iter().enumerate() {
        for (i, val) in xs.iter().enumerate() {
            let coords = (i.try_into().unwrap(), j.try_into().unwrap());
            let neighbours = get_neighbours(&grid, coords);
            if neighbours.iter().filter_map(|v|*v).all(|n|val < n) {
                sum += u32::from(*val) + 1;
                basins.push(HashSet::from([coords]));
            }
        }
    }

    println!("Sum of the risk levels: {}", sum);
    let indices: Vec<usize> = basins.iter().enumerate().map(|(i, _)|i).collect();
    for i in indices {
        let mut search_queue = VecDeque::from([*basins[i].iter().next().unwrap()]);
        while !search_queue.is_empty() {
            if let Some(prev) = search_queue.pop_back() {
                // Prev cannot be added without being valid. Should never panic:
                let prev_val = get_2d(&grid, prev).unwrap();
                for coord in neighbour_coords(prev) {
                    if let Some(val) = get_2d(&grid, coord) {
                        if *val != 9 && val > prev_val && !(basins.iter().any(|b|b.contains(&coord))) {
                            search_queue.push_front(coord);
                            basins[i].insert(coord);
                        }
                    }
                }
            }
        }
    }

    let mut sinks: Vec<usize> = basins.into_iter().map(|h|h.into_iter().count()).collect();
    sinks.sort_unstable();
    println!("Product of 3 largest basins: {}", (&sinks.into_iter().rev().collect::<Vec<usize>>()[..3]).iter().fold(1, |ls, l|ls*l))
}

fn get_2d(grid: &Vec<Vec<u8>>, (i, j): Coord) -> Option<&u8> {
    let a: usize = j.try_into().ok()?;
    let xs: &Vec<u8> = grid.get(a)?;
    let b: usize = i.try_into().ok()?;
    xs.get(b)
}

fn neighbour_coords((i, j): Coord) -> [Coord; 4] {
    [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]
}

fn get_neighbours(grid: &Vec<Vec<u8>>, (i, j): Coord) -> [Option<&u8>; 4] {
    [get_2d(grid, (i-1, j)), get_2d(grid, (i+1, j)), get_2d(grid, (i, j-1)), get_2d(grid, (i, j+1))]
}