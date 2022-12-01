fn parse(str: &str) -> Vec<Vec<u64>> {
    str.split("\n\n")
        .map(|subsec| {
            subsec
                .trim()
                .split("\n")
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

fn find_max_elf(input: &str) -> Option<u64> {
    parse(input).into_iter().map(|l| l.into_iter().sum()).max()
}

fn find_top_3_elfs(input: &str) -> u64 {
    let mut elfs: Vec<u64> = parse(input).iter().map(|l| l.iter().sum()).collect();
    elfs.sort();
    elfs.into_iter().rev().take(3).sum()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/1.txt");
    println!(
        "The elf with the most calories has {} calories",
        find_max_elf(INPUT).unwrap()
    );

    println!(
        "The top 3 elfs with the most calories has a combined {} calories",
        find_top_3_elfs(INPUT)
    );
}

const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn test_parse() {
    let expected_output = vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
    ];
    assert_eq!(parse(EXAMPLE_INPUT), expected_output)
}

#[test]
fn example_1() {
    assert_eq!(find_max_elf(EXAMPLE_INPUT), Some(24000))
}

#[test]
fn example_2() {
    assert_eq!(find_top_3_elfs(EXAMPLE_INPUT), 45000);
}
