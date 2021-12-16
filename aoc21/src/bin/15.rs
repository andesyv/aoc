use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::rc::{Rc, Weak};

#[derive(Eq, Clone, Copy, Debug)]
struct Node {
    distance: u32,
    cost: u8,
    coord: (usize, usize),
    prev: Option<(usize, usize)>,
}

impl Node {
    fn new(cost: u8) -> Node {
        Node {
            distance: u32::MAX,
            cost: cost,
            coord: (0, 0),
            prev: None,
        }
    }
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

fn main() {
    const INPUT: &str = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";

    let grid: Vec<Vec<Node>> = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Node::new(c.to_digit(10).unwrap().try_into().unwrap()))
                .collect()
        })
        .collect();

    println!("Grid is: {:?}", grid);

    let grid_size: (usize, usize) = (grid.get(0).unwrap().len(), grid.len());

    let shortest_path = djikstra(&grid, (0, 0), (grid_size.0 - 1, grid_size.0 - 1));
    println!("Shortest path: {:?}", shortest_path);
}

fn djikstra(grid: &Vec<Vec<Node>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut checked = HashMap::from([(start, Node { distance: 0, cost: 0, coord: start, prev: None })]);
    let mut unvisited = BinaryHeap::new();
    // let mut checked = HashSet::new();
    let grid_indices: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .map(|(j, v)| {
            v.iter()
                .enumerate()
                .map(|(i, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut check_neighbours = |current_coord: (usize, usize), current_distance: u32| {
        let surrounding = get_surrounding(current_coord);
        let mut neighbours = Vec::new();
        for coord in surrounding {
            if let Some(coord) = coord {
                let cost = index(&grid, coord).cost;
                let node = Node {
                    distance: current_distance + u32::from(cost),
                    cost: cost,
                    coord: coord,
                    prev: Some(current_coord),
                };
                if checked.insert(coord, node).is_none() {
                    neighbours.push(node)
                }
            }
        }
        neighbours
    };

    let find_path = |end: &Node| {
        let mut path = Vec::new();
        let mut curr = Some(end.coord);
        while let Some(coord) = curr {
            path.push(coord);
            if let Some(prev_node) = checked.get(&coord) {
                curr = prev_node.prev
            } else {
                break;
            }
        }
        path.into_iter().rev().collect()
    };

    let current_distance = u32::from(index(&grid, start).cost);
    for node in check_neighbours(start, current_distance) {
        unvisited.push(node);
    }

    while let Some(current) = unvisited.pop() {
        if current.coord == end {
            return find_path(index(&grid, current.coord));
        }
        unvisited.extend(check_neighbours(current.coord, current.distance).into_iter());
    }
    vec![]
}

fn index_mut(grid: &mut Vec<Vec<Node>>, (i, j): (usize, usize)) -> &mut Node {
    grid.get_mut(j).unwrap().get_mut(i).unwrap()
}

fn index(grid: &Vec<Vec<Node>>, (i, j): (usize, usize)) -> &Node {
    grid.get(j).unwrap().get(i).unwrap()
}

fn get_coords(grid: &Vec<Vec<Node>>, node: &Node) -> Option<(usize, usize)> {
    for j in 0..grid.len() {
        for i in 0..grid.get(j).unwrap().len() {
            return Some((i, j));
        }
    }
    None
}

fn get_surrounding((i, j): (usize, usize)) -> [Option<(usize, usize)>; 8] {
    let mut n = 0;
    let mut out = [None; 8];
    for y in -1..2 {
        for x in -1..2 {
            if y != 0 || x != 0 {
                if let (Some(a), Some(b)) = (checked_add_signed(i, x), checked_add_signed(j, y)) {
                    out[n] = Some((a, b));
                }
                n += 1;
            }
        }
    }
    out
}

fn checked_add_signed(a: usize, b: isize) -> Option<usize> {
    if b < 0 {
        a.checked_sub(b.checked_neg()?.try_into().ok()?)
    } else {
        a.checked_add(b.try_into().ok()?)
    }
}
