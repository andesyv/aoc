use itertools::izip;

fn main() {
    const INPUT: &str = include_str!("../inputs/1.txt");
    let nums = INPUT.lines().map(|x| x.parse::<i32>().unwrap());
    println!("Number of increases: {}", count_larger(nums.clone()));

    let triple_sums = izip!(nums.clone(), nums.clone().skip(1), nums.skip(2)).map(|(a,b,c)| a+b+c);
    println!("Number of increases on triples: {}", count_larger(triple_sums));
}

fn count_larger(ls: impl Iterator<Item = i32>) -> i32 {
    let mut count = 0;
    let mut last = 100000000;
    for num in ls {
        if last < num {
            count += 1;
        }
        last = num;
    }
    count
}