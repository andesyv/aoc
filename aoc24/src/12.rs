use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::Instant;

type Pos = (i64, i64);

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
        while let Some(index) = points_to_process
            .iter()
            .position(|point| point_intersects_with_cluster(*point, &current_cluster))
        {
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
                grouped_sets.insert(*c, vec![(x as i64, y as i64)]);
            } else {
                grouped_sets.get_mut(c).unwrap().push((x as i64, y as i64));
            }
        }
    }

    // Now split the labeled groups into clusters
    grouped_sets
        .into_iter()
        .map(|(_, group)| cluster(group).into_iter())
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

fn get_price_of_fencing_for_garden(clusters: &[HashSet<Pos>]) -> usize {
    clusters.iter().map(get_cluster_fencing_price).sum()
}

fn find_edge_count_of_cluster(cluster: &HashSet<Pos>) -> usize {
    // We can utilize the fact that our clusters are always adjacent cells. This means, if we
    // were to put a point in the corner of each cell, every even number of points on the same
    // spot would mean either a straight edge or a surrounded point. So our logic is thus:
    //  1. First, create 4 points per cell in the cluster and count overlapping points.
    //  2. Remove every even set of points.
    //  3. Calculate the edges as the number of points

    let mut cell_corners = HashMap::new();
    for pos in cluster {
        // Create 4 corner points on the diagonals of the position. Usually one would do this by
        // subtracting and adding 0.5 to the coordinates to make the corner points. But we only
        // care about the amount of points, so we will just add 1 in the x and y directions.
        let (x, y) = *pos;
        let corners = [(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)];
        for corner in corners {
            if let Some(count) = cell_corners.get_mut(&corner) {
                *count += 1;
            } else {
                cell_corners.insert(corner, 1);
            }
        }
    }

    // Filter away even sets of points and return the count:
    let regular_edges = cell_corners
        .values()
        .filter(|&count| count % 2 == 1)
        .count();

    // Edge case: This algorithm does not work with cells part of the same cluster but on opposing
    // sides: E.g.
    // ```text
    // AAAA
    // AABA
    // ABAA
    // AAAA
    // ```
    // Here, the diagonal A's in the middle would create a point between them with an even count
    // of points. Thus, it will remove said points (while in reality they should be kept).
    //
    // Solution: Each such scenario creates a very specific arrangement, so we can probably just
    // manually search the points and look for this specific scenario and then add 2 points
    // for each such arrangement.
    let special_scenarios = [
        [((1, 0), false), ((0, 1), false), ((1, 1), true)],
        [((1, 0), false), ((0, -1), false), ((1, -1), true)],
        [((-1, 0), false), ((0, -1), false), ((-1, -1), true)],
        [((-1, 0), false), ((0, 1), false), ((-1, 1), true)],
    ];

    let mut edge_case_points = 0;
    for pos in cluster {
        'case: for special_case in special_scenarios {
            for (relative_pos, should_exist) in special_case {
                let new_pos = (pos.0 + relative_pos.0, pos.1 + relative_pos.1);
                if cluster.contains(&new_pos) != should_exist {
                    continue 'case;
                }
            }
            edge_case_points += 1;
        }
    }

    regular_edges + edge_case_points
}

fn get_price_of_fencing_for_garden_with_bulk_discount(clusters: &[HashSet<Pos>]) -> usize {
    clusters
        .iter()
        .map(|cluster| {
            let edge_count = find_edge_count_of_cluster(cluster);
            edge_count * cluster.len()
        })
        .sum()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/12.txt");

    let start = Instant::now();
    let clusters = collect_connected_sets(INPUT);
    println!("Clustering done after {}ms", start.elapsed().as_millis());

    println!(
        "Cost of fencing for the garden: {}",
        get_price_of_fencing_for_garden(clusters.as_slice())
    );

    println!(
        "Cost of fencing for the garden with a bulk discount: {}",
        get_price_of_fencing_for_garden_with_bulk_discount(clusters.as_slice())
    );
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
    let clusters = collect_connected_sets(SMALL_EXAMPLE);
    assert_eq!(get_price_of_fencing_for_garden(clusters.as_slice()), 140);
}

#[test]
fn get_price_of_fencing_for_garden_from_medium_example() {
    let clusters = collect_connected_sets(MEDIUM_EXAMPLE);
    assert_eq!(get_price_of_fencing_for_garden(clusters.as_slice()), 772);
}

#[test]
fn get_price_of_fencing_for_garden_from_big_example() {
    let clusters = collect_connected_sets(BIG_EXAMPLE);
    assert_eq!(get_price_of_fencing_for_garden(clusters.as_slice()), 1930);
}

#[test]
fn get_price_of_fencing_for_garden_with_bulk_discount_from_small_example() {
    let clusters = collect_connected_sets(SMALL_EXAMPLE);
    assert_eq!(
        get_price_of_fencing_for_garden_with_bulk_discount(clusters.as_slice()),
        80
    );
}

#[test]
fn get_price_of_fencing_for_garden_with_bulk_discount_from_medium_example() {
    let clusters = collect_connected_sets(MEDIUM_EXAMPLE);
    assert_eq!(
        get_price_of_fencing_for_garden_with_bulk_discount(clusters.as_slice()),
        436
    );
}

const E_SHAPE_EXAMPLE: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

const EXAMPLE_WITH_DIAGONAL_CLUSTERS: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

#[test]
fn get_price_of_fencing_for_garden_with_bulk_discount_from_e_shape_example() {
    let clusters = collect_connected_sets(E_SHAPE_EXAMPLE);
    assert_eq!(
        get_price_of_fencing_for_garden_with_bulk_discount(clusters.as_slice()),
        236
    );
}

#[test]
fn get_price_of_fencing_for_garden_with_bulk_discount_from_example_with_diagonal_clusters() {
    let clusters = collect_connected_sets(EXAMPLE_WITH_DIAGONAL_CLUSTERS);
    assert_eq!(
        get_price_of_fencing_for_garden_with_bulk_discount(clusters.as_slice()),
        368
    );
}

#[test]
fn get_price_of_fencing_for_garden_with_bulk_discount_from_big_example() {
    let clusters = collect_connected_sets(BIG_EXAMPLE);
    assert_eq!(
        get_price_of_fencing_for_garden_with_bulk_discount(clusters.as_slice()),
        1206
    );
}
