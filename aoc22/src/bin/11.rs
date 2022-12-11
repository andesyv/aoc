use std::collections::VecDeque;

use evalexpr::context_map;
use itertools::Itertools;
use regex::Regex;

struct Monkey {
    items: VecDeque<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    inspection_count: usize,
}

fn parse_op(expression: &str) -> Option<Box<impl Fn(u64) -> u64>> {
    let func = evalexpr::build_operator_tree(expression).unwrap();
    Some(Box::new(move |old: u64| -> u64 {
        let context = context_map! {
          "old" => i64::try_from(old).unwrap()
        }
        .unwrap();

        func.eval_int_with_context(&context)
            .unwrap()
            .try_into()
            .unwrap()
    }))
}

fn parse_test(
    (test_arg, target_1, target_2): (u64, u64, u64),
) -> Option<Box<impl Fn(u64) -> usize>> {
    Some(Box::new(move |x: u64| -> usize {
        (if x % test_arg == 0 {
            target_1
        } else {
            target_2
        })
        .try_into()
        .unwrap()
    }))
}

fn parse(input: &str) -> Vec<Monkey> {
    let find_starting_items = Regex::new(r"Starting items: (.*)").unwrap();
    let find_operation = Regex::new(r"Operation: new = (.*)").unwrap();
    let find_test =
        Regex::new(r"Test: divisible by (?P<test>\d+)\n.*(?P<target_1>\d+)\n.*(?P<target_2>\d+)")
            .unwrap();

    input
        .split("\n\n")
        .map(|chunk| Monkey {
            items: find_starting_items
                .captures(chunk)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            op: parse_op(
                find_operation
                    .captures(chunk)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str(),
            )
            .unwrap(),
            test: parse_test(
                find_test
                    .captures_iter(chunk)
                    .nth(0)
                    .and_then(|capture_matches| {
                        Some((
                            (&capture_matches["test"]).parse::<u64>().ok()?,
                            (&capture_matches["target_1"]).parse::<u64>().ok()?,
                            (&capture_matches["target_2"]).parse::<u64>().ok()?,
                        ))
                    })
                    .unwrap(),
            )
            .unwrap(),
            inspection_count: 0,
        })
        .collect()
}

fn play_round(monkeys: &mut Vec<Monkey>, reduced_worryness: bool) {
    for i in 0..monkeys.len() {
        while let Some(mut item) = monkeys.get_mut(i).unwrap().items.pop_front() {
            let next_monkey_index = {
                let monkey = monkeys.get_mut(i).unwrap();
                let item_2 = item;
                item = monkey.op.as_ref()(item);
                if item < item_2 {
                  panic!("Worryness can never decrease from operation!");
                }
                if reduced_worryness {
                    item /= 3;
                }
                monkey.inspection_count += 1;
                monkey.test.as_ref()(item)
            };
            monkeys
                .get_mut(next_monkey_index)
                .unwrap()
                .items
                .push_back(item);
        }
    }
}

fn get_monkey_items(monkeys: &Vec<Monkey>) -> Vec<Vec<u64>> {
    monkeys
        .iter()
        .map(|m| m.items.iter().copied().collect_vec())
        .collect_vec()
}

fn get_monkey_business_after_20_rounds(input: &str) -> usize {
    let mut monkeys = parse(input);
    for _ in 0..20 {
        play_round(&mut monkeys, true);
    }
    monkeys
        .into_iter()
        .map(|monkey| monkey.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .reduce(|accum, item| accum * item)
        .unwrap()
}

fn get_monkey_business_after_10000_rounds(input: &str) -> usize {
    let mut monkeys = parse(input);
    for _ in 0..10000 {
        play_round(&mut monkeys, false);
    }
    monkeys
        .into_iter()
        .map(|monkey| monkey.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .reduce(|accum, item| accum * item)
        .unwrap()
}

fn main() {
    const INPUT: &str = include_str!("../inputs/11.txt");
    println!(
        "Monkey business after 20 days: {}",
        get_monkey_business_after_20_rounds(INPUT)
    );
}

const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[test]
fn test_parse() {
    let parsed = parse(EXAMPLE_INPUT);
    let first = parsed.first().unwrap();
    assert_eq!(first.items, vec![79, 98]);
    assert_eq!(first.op.as_ref()(1), 19);
    assert_eq!(first.op.as_ref()(2), 38);
    assert_eq!(first.test.as_ref()(23), 2);
    assert_eq!(first.test.as_ref()(24), 3);

    let second = parsed.get(1).unwrap();
    assert_eq!(second.items, vec![54, 65, 75, 74]);
    assert_eq!(second.op.as_ref()(1), 7);
    assert_eq!(second.op.as_ref()(2), 8);
    assert_eq!(second.test.as_ref()(19), 2);
    assert_eq!(second.test.as_ref()(20), 0);
}

#[test]
fn example_1_round() {
    let mut monkeys = parse(EXAMPLE_INPUT);
    play_round(&mut monkeys, true);
    assert_eq!(
        get_monkey_items(&monkeys),
        Vec::from([
            vec![20, 23, 27, 26],
            vec![2080, 25, 167, 207, 401, 1046],
            vec![],
            vec![]
        ])
    );
}

#[test]
fn example_2_rounds() {
    let mut monkeys = parse(EXAMPLE_INPUT);
    play_round(&mut monkeys, true);
    play_round(&mut monkeys, true);
    assert_eq!(
        get_monkey_items(&monkeys),
        Vec::from([
            vec![695, 10, 71, 135, 350],
            vec![43, 49, 58, 55, 362],
            vec![],
            vec![]
        ])
    );
}

#[test]
fn example_monkey_business() {
    assert_eq!(get_monkey_business_after_20_rounds(EXAMPLE_INPUT), 10605);
}

#[test]
fn example_20_rounds_increased_worryness() {
    let mut monkeys = parse(EXAMPLE_INPUT);
    for _ in 0..20 {
        play_round(&mut monkeys, false);
    }
    assert_eq!(
        monkeys
            .iter()
            .map(|monkey| monkey.inspection_count)
            .collect_vec(),
        vec![99, 97, 8, 103]
    );
    // assert_eq!(
    //   monkeys.iter().map(|monkey|monkey.inspection_count).collect_vec(),
    //   vec![101, 95, 7, 105]
    // );
}

#[test]
fn example_monkey_business_2() {
    assert_eq!(
        get_monkey_business_after_10000_rounds(EXAMPLE_INPUT),
        2713310158
    );
}
