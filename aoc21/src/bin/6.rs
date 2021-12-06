#[derive(Debug)]
struct Fish {
    birth_timer: i32,
}

impl Fish {
    fn new() -> Self {
        Fish { birth_timer: 8 }
    }
}

fn main() {
    // const INPUT: &str = include_str!("../inputs/6.txt");
    const INPUT: &str = "3,4,3,1,2";

    let mut fishies: Vec<Fish> = INPUT
        .trim()
        .split(',')
        .map(|s| Fish { birth_timer: s.parse().unwrap() }).collect();
    // println!("Initial fishies {:?}", fishies);
    simulate(&mut fishies, 80);
    println!("Fish count after 80 days is {}", fishies.len());
}

fn simulate(fishies: &mut Vec<Fish>, days: u32) {
    for _ in 0..days {
        let mut new_fish_count = 0;
        for fish in &mut *fishies {
            fish.birth_timer -= 1;
            if fish.birth_timer < 0 {
                new_fish_count += 1;
                fish.birth_timer = 6;
            }
        }

        for _ in 0..new_fish_count {
            fishies.push(Fish::new());
        }
    }
}
