use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Grid = Vec<Vec<u8>>;

fn main() {
    const INPUT: &str = include_str!("../inputs/15.txt");
    // const INPUT: &str = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";
    let grid: Grid = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let (x_max, y_max) = find_grid_max(&grid);
    let path = djikstra(&grid, (0, 0), (x_max - 1, y_max - 1));
    // println!("Path is: {:?}", path.iter().map(|n|n.pos).collect::<Vec<(usize, usize)>>());
    println!("Path's risk sum is {}", path.iter().map(|n|n.cost).sum::<u32>());

    // Part 2:
    let big_grid = gen_large_grid(&grid);
    // println!("Big grid: \n{}", big_grid.iter().map(|l|l.iter().map(|n|n.to_string()).collect::<String>()).collect::<Vec<String>>().join("\n"));
    let (x_max, y_max) = find_grid_max(&big_grid);
    let path = djikstra(&big_grid, (0, 0), (x_max - 1, y_max - 1));
    println!("Big grid's path's risk sum is {}", path.iter().map(|n|n.cost).sum::<u32>());

}

fn find_grid_max(grid: &Grid) -> (usize, usize) {
    (grid.len(), grid.iter().nth(0).unwrap().len())
}

#[derive(Eq, Clone, Copy, Debug)]
struct Node {
    cost: u32,
    distance: u32,
    pos: (usize, usize),
    prev: (usize, usize),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn djikstra(grid: &Grid, start: (usize, usize), end: (usize, usize)) -> Vec<Node> {
    let mut checked = HashMap::new();
    let mut search_heap = BinaryHeap::new();

    // Initial nodes:
    checked.insert(
        start,
        Node {
            cost: 0,
            distance: 0,
            pos: start,
            prev: start,
        },
    );
    for neighbour in neighbour_indices(start) {
        if let Some(coord) = neighbour {
            if let Some(value) = index(grid, coord) {
                let node = Node {
                    cost: u32::from(*value),
                    distance: u32::from(*value),
                    pos: coord,
                    prev: start,
                };
                if checked.insert(coord, node).is_none() {
                    search_heap.push(node);
                }
            }
        }
    }

    // Main logic:
    while let Some(mut next) = search_heap.pop() {
        if next.pos == end {
            let mut rets = Vec::from([next]);
            let mut prev = next.prev;
            while prev != start {
                next = *checked.get(&prev).unwrap();
                prev = next.prev;
                rets.push(next);
            }

            return rets.into_iter().rev().collect();
        }

        for neighbour in neighbour_indices(next.pos) {
            if let Some(coord) = neighbour {
                if let Some(value) = index(grid, coord) {
                    let node = Node {
                        cost: u32::from(*value),
                        distance: next.distance + u32::from(*value),
                        pos: coord,
                        prev: next.pos,
                    };
                    // Cannot use insert to check, because it also updates the entry
                    // (can use try_insert whenever it becomes stable)
                    if !checked.contains_key(&coord) {
                        checked.insert(coord, node);
                        search_heap.push(node);
                    }
                }
            }
        }
    }

    vec![]
}

fn neighbour_indices((i, j): (usize, usize)) -> [Option<(usize, usize)>; 4] {
    [
        safe_add((i, j), (-1, 0)),
        safe_add((i, j), (1, 0)),
        safe_add((i, j), (0, -1)),
        safe_add((i, j), (0, 1)),
    ]
}

fn safe_add(a: (usize, usize), b: (i32, i32)) -> Option<(usize, usize)> {
    let l = usize::try_from(i32::try_from(a.0).ok()? + b.0).ok()?;
    let r = usize::try_from(i32::try_from(a.1).ok()? + b.1).ok()?;
    Some((l, r))
}

fn index(grid: &Grid, (i, j): (usize, usize)) -> Option<&u8> {
    grid.get(j)?.get(i)
}

fn gen_large_grid(small_grid: &Grid) -> Grid {
    let small_grid_size = find_grid_max(small_grid);
    let mut grid = Vec::with_capacity(small_grid_size.1 * 5);
    for y in 0..5 {
        for l in small_grid {
            let mut newline = Vec::with_capacity(small_grid_size.0 * 5);
            for x in 0..5 {
                newline.extend(l.iter().map(|v|{
                    let b = v + x + y;
                    if 9 < b {
                        b - 9
                    } else {
                        b
                    }
                }));
            }
            grid.push(newline);
        }
    }
    grid
}

#[test]
fn test1() {
    const INPUT: &str = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";
    let grid: Grid = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let (x_max, y_max) = find_grid_max(&grid);
    let path = djikstra(&grid, (0, 0), (x_max - 1, y_max - 1));

    assert_eq!(path.iter().map(|n|n.cost).sum::<u32>(), 40);
}

#[test]
fn test2() {
    const INPUT: &str = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";
    let mut grid: Grid = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    grid = gen_large_grid(&grid);
    let (x_max, y_max) = find_grid_max(&grid);
    let path = djikstra(&grid, (0, 0), (x_max - 1, y_max - 1));

    assert_eq!(path.iter().map(|n|n.cost).sum::<u32>(), 315);
}