use std::collections::linked_list::LinkedList;
use memoize::memoize;
use std::thread;
// Notes:
//  - Initially I thought that using a vector would be slow due to all the reallocations with
//    exponentially huge lists, but surprisingly a linked list was slightly slower.
//  - Recursive function, going from a wide problem-solving tree to a deep problem-solving tree,
//    also does not help with speed.
//  - Using multiple thread also doesn't help me past the exponential complexity hurdle

fn transform_stone(stone: u64) -> LinkedList<u64> {
    if stone == 0 {
        return [1].into();
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        // E.g. 1024 / 10^(digits / 2) = 1024 / 100 = 10
        let exp = 10u64.pow(digits / 2);
        let lhs = stone / exp;
        let rhs = stone - lhs * exp;
        return [lhs, rhs].into();
    }

    [stone * 2024].into()
}

fn blink(mut stones: LinkedList<u64>) -> LinkedList<u64> {
    // growing capacity by 1.5 seems like a fine starting point
    // let mut new_stones = Vec::with_capacity(stones.len() * 3 / 2);

    let mut new_stones = LinkedList::new();
    while let Some(stone) = stones.pop_front() {
        new_stones.append(&mut transform_stone(stone));
    }

    new_stones
}

#[memoize]
fn get_count_of_transforming_stone_n_times(stone: u64, n: usize) -> u64 {
    if n == 0 {
        return 1;
    }

    if stone == 0 {
        return get_count_of_transforming_stone_n_times(1, n - 1);
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        // E.g. 1024 / 10^(digits / 2) = 1024 / 100 = 10
        let exp = 10u64.pow(digits / 2);
        let lhs = stone / exp;
        let rhs = stone - lhs * exp;
        // return if n == 40 {
        //     let other_tree = thread::spawn(move || {
        //         get_count_of_transforming_stone_n_times(lhs, n - 1)
        //     });
        //     get_count_of_transforming_stone_n_times(rhs, n - 1) + other_tree.join().unwrap()
        // } else {
        return get_count_of_transforming_stone_n_times(lhs, n - 1) + get_count_of_transforming_stone_n_times(rhs, n - 1)
        // }
    }

    get_count_of_transforming_stone_n_times(stone * 2024, n - 1)
}

fn stone_count_after_blinking_n_times(input: &str, n: usize) -> u64 {
    let mut stones = parse(input).into_iter().collect();
    for i in 0..n {
        println!("Blinked {} times", i);
        stones = blink(stones);
    }

    stones.len() as u64
}

fn stone_count_after_recursively_blinking_n_times(input: &str, n: usize) -> u64 {
    parse(input).into_iter().map(|stone|{
        get_count_of_transforming_stone_n_times(stone, n)
    }).sum()
}

fn parse(input: &str) -> Vec<u64> {
    input.trim().split(' ').filter_map(|x| x.parse().ok()).collect()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/11.txt");
    println!("Stone count after blinking 25 times: {}", stone_count_after_recursively_blinking_n_times(INPUT, 25));

    println!("After blinking 75 times...: {}", stone_count_after_recursively_blinking_n_times(INPUT, 75));
}

const EXAMPLE_INPUT: &str = "125 17";

#[test]
fn transform_stone_test() {
    assert_eq!(transform_stone(0), LinkedList::from([1]));
    assert_eq!(transform_stone(10), LinkedList::from([1, 0]));
    assert_eq!(transform_stone(11), LinkedList::from([1, 1]));
    assert_eq!(transform_stone(123), LinkedList::from([123 * 2024]));
    assert_eq!(transform_stone(2031), LinkedList::from([20, 31]));
}

#[test]
fn count_stones_after_blinking_25_times_test() {
    assert_eq!(stone_count_after_recursively_blinking_n_times(EXAMPLE_INPUT, 25), 55312);
}
