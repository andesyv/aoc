use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::Instant;

type Pos = (usize, usize);

fn get_neighbours(pos: Pos) -> [Pos; 4] {
    let (x, y) = pos;
    [
        (x, y + 1),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
    ]
}

fn point_intersects_with_cluster(point: Pos, cluster: &HashSet<Pos>) -> bool {
    for neighbour in get_neighbours(point) {
        if cluster.contains(&neighbour) {
            return true;
        }
    }

    false
}

fn cluster(mut points_to_process: Vec<Pos>) -> Vec<HashSet<Pos>> {
    // Logic:
    //  1. First pop a point in the processing list to become the current cluster
    //  2. Then, iterate through the list attempting to find any intersection.
    //    a. If a cluster intersection was found, add that point to the current set. Start over from step 2.
    //    b. Otherwise, start over from step 1.
    //  3. When the processing list is done, clusters are done.
    let mut clusters = Vec::new();

    while let Some(core_point) = points_to_process.pop() {
        let mut current_cluster = HashSet::from([core_point]);
        while let Some(index) = points_to_process.iter().position(|point|point_intersects_with_cluster(*point, &current_cluster)) {
            current_cluster.insert(points_to_process.remove(index));
        }
        clusters.push(current_cluster);
    }

    clusters
}

fn collect_connected_sets(input: &str) -> Vec<HashSet<Pos>> {
    let mut grouped_sets: HashMap<u8, Vec<Pos>> = HashMap::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            if !grouped_sets.contains_key(c) {
                grouped_sets.insert(*c, vec![(x, y)]);
            } else {
                grouped_sets.get_mut(c).unwrap().push((x, y));
            }
        }
    }

    // Now split the labeled groups into clusters
    grouped_sets
        .into_iter()
        .map(|(_, group)|cluster(group).into_iter())
        .flatten()
        .collect()
}

fn get_cluster_perimeter(cluster: &HashSet<Pos>) -> usize {
    let mut count = 0;

    for pos in cluster {
        count += 4;
        for neighbour in get_neighbours(*pos) {
            if cluster.contains(&neighbour) {
                count -= 1;
            }
        }
    }

    count
}

fn get_cluster_fencing_price(cluster: &HashSet<Pos>) -> usize {
    get_cluster_perimeter(cluster) * cluster.len()
}

fn get_price_of_fencing_for_garden(input: &str) -> usize {
    let start = Instant::now();
    let clusters = collect_connected_sets(input);
    println!("Clustering done after {}ms", start.elapsed().as_millis());
    clusters.iter().map(get_cluster_fencing_price).sum()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/12.txt");
    println!("Cost of fencing for the garden: {}", get_price_of_fencing_for_garden(INPUT));
}

const SMALL_EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

const MEDIUM_EXAMPLE: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const BIG_EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

#[test]
fn clustering_test() {
    // Note: Using a BTreeSet to ensure test stability (iterators into Vec and HashSet does not
    // have a guaranteed order).
    let sets: BTreeSet<BTreeSet<Pos>> = collect_connected_sets(SMALL_EXAMPLE)
        .into_iter()
        .map(|set| set.into_iter().collect())
        .collect();
    assert_eq!(sets.len(), 5);
    let mut sets_iter = sets.iter();
    assert_eq!(
        format!("{:?}", sets_iter.next().unwrap()),
        "{(0, 0), (1, 0), (2, 0), (3, 0)}"
    );
    assert_eq!(
        format!("{:?}", sets_iter.next().unwrap()),
        "{(0, 1), (0, 2), (1, 1), (1, 2)}"
    );
    assert_eq!(
        format!("{:?}", sets_iter.next().unwrap()),
        "{(0, 3), (1, 3), (2, 3)}"
    );
    assert_eq!(
        format!("{:?}", sets_iter.next().unwrap()),
        "{(2, 1), (2, 2), (3, 2), (3, 3)}"
    );
    assert_eq!(format!("{:?}", sets_iter.next().unwrap()), "{(3, 1)}");

    let sets: BTreeSet<BTreeSet<Pos>> = collect_connected_sets(MEDIUM_EXAMPLE)
        .into_iter()
        .map(|set| set.into_iter().collect())
        .collect();
    assert_eq!(sets.len(), 5);
    let mut sets_iter = sets.iter();
    assert_eq!(format!("{:?}", sets_iter.next().unwrap()), "{(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (1, 0), (1, 2), (1, 4), (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (3, 0), (3, 2), (3, 4), (4, 0), (4, 1), (4, 2), (4, 3), (4, 4)}");
    assert_eq!(format!("{:?}", sets_iter.next().unwrap()), "{(1, 1)}");
    assert_eq!(format!("{:?}", sets_iter.next().unwrap()), "{(1, 3)}");
    assert_eq!(format!("{:?}", sets_iter.next().unwrap()), "{(3, 1)}");
    assert_eq!(format!("{:?}", sets_iter.next().unwrap()), "{(3, 3)}");
}

#[test]
fn get_price_of_fencing_for_garden_from_small_example() {
    assert_eq!(get_price_of_fencing_for_garden(SMALL_EXAMPLE), 140);
}

#[test]
fn get_price_of_fencing_for_garden_from_medium_example() {
    assert_eq!(get_price_of_fencing_for_garden(MEDIUM_EXAMPLE), 772);
}

#[test]
fn get_price_of_fencing_for_garden_from_big_example() {
    assert_eq!(get_price_of_fencing_for_garden(BIG_EXAMPLE), 1930);
}
