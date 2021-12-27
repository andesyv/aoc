fn main() {
    // const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    const INPUT: &str = include_str!("../inputs/7.txt");
    let mut positions: Vec<i64> = INPUT.trim().split(",").filter_map(|s|s.parse().ok()).collect();
    // println!("Positions: {:?}", positions);
    
    let (mut best_pos, mut best_cost) = (0, i64::MAX);
    positions.sort_unstable();
    let (min_pos, max_pos) = (*positions.get(0).unwrap(), *positions.get(positions.len() - 1).unwrap());
    for center in min_pos..=max_pos {
        let sum_fuel = positions.iter().map(|p|(p - center).abs()).sum();
        if sum_fuel < best_cost {
            best_cost = sum_fuel;
            best_pos = center;
        }
    }

    println!("Best linear pos was {}, which will cost {} fuel", best_pos, best_cost);

    let (mut best_pos, mut best_cost) = (0, i64::MAX);
    positions.sort_unstable();
    let (min_pos, max_pos) = (*positions.get(0).unwrap(), *positions.get(positions.len() - 1).unwrap());
    for center in min_pos..=max_pos {
        let sum_fuel = positions.iter().map(|p|{
            let n = (p - center).abs();
            // arithmetic sum:
            n * (1+n) / 2
        }).sum();
        if sum_fuel < best_cost {
            best_cost = sum_fuel;
            best_pos = center;
        }
    }

    println!("Best pos was {}, which will cost {} fuel", best_pos, best_cost);
}