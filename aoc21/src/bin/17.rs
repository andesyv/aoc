use std::collections::HashSet;

fn main() {
    // Too lazy to even parse the input probely correctly on this one. Can just hardcode:
    // find_velocites((20, 30), (-10, -5), 100);
    find_velocites((119, 176), (-141, -84), 500);
}

fn find_velocites(xminmax: (i64, i64), yminmax: (i64, i64), search_space: i64) {
    let mut uniques = HashSet::new();
    let mut top_highest = 0;
    for y in -search_space..search_space {
        let ts = valid_y(y, yminmax, search_space);
        for (t, highest) in ts {
            let xs = find_xs(t, xminmax, search_space);
            for x in xs {
                uniques.insert((x, y));
                if top_highest < highest {
                    top_highest = highest;
                    println!("New highest point {} achieved with velocity ({},{})", top_highest, x, y);
                }
            }
        }
    }

    println!("Distinct velocities: {}. Those are: {}", uniques.len(), uniques.into_iter().map(|(x,y)|format!("({},{})", x, y)).collect::<Vec<String>>().join(","));
}

fn valid_y(mut v: i64, (y_min, y_max): (i64, i64), search_space: i64) -> Vec<(i64, i64)> {
    let mut ts = Vec::new();
    let mut y = 0;
    let mut highest = 0;
    for t in 0..search_space {
        if y < y_min {
            break;
        } else if y <= y_max {
            ts.push((t, highest));
        }
        y += v;
        if highest < y {
            highest = y;
        }
        v -= 1;
    }
    ts
}

fn find_xs(t: i64, (x_min, x_max): (i64, i64), search_space: i64) -> Vec<i64> {
    let mut xs = Vec::new();
    for v0 in 0..search_space {
        let mut v = v0;
        let mut x = 0;
        for _ in 0..t {
            x += v;
            if v != 0 {
                v -= 1;
            }
        }
        if x < x_min {
            continue;
        } else if x <= x_max {
            xs.push(v0);
            continue;
        }
    }
    xs
}