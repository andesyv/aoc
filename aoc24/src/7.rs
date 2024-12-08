use regex::Regex;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
struct Equation {
    result: u64,
    params: Vec<u64>,
}

fn parse_equation(line: &str) -> Option<Equation> {
    let re_first = Regex::new(r"([0-9]+)").unwrap();
    let re_rest = Regex::new(r"\s([0-9]+)").unwrap();
    let first_match = re_first.find(line)?;
    let result = u64::from_str(first_match.as_str()).ok()?;

    let mut nums = Vec::new();
    for (_, [num]) in re_rest.captures_iter(&line[(first_match.range().end + 1)..]).map(|c|c.extract()) {
        nums.push(u64::from_str(num).ok()?);
    }
    Some(Equation { result, params: nums })
}

fn parse(input: &str) -> Vec<Equation> {
    input.lines().filter_map(parse_equation).collect()
}

enum Op {
    Add,
    Multiply,
}

fn arithmetic(a: u64, b: u64, operators: &[Op]) -> HashSet<u64> {
    operators.iter().map(|op|match op {
        Op::Add => a + b,
        Op::Multiply => a * b,
    }).collect()
}

fn calculate(partial_sums: HashSet<u64>, rest: &[u64], operators: &[Op]) -> HashSet<u64> {
    // println!("Interim results: {:?}", partial_sums);
    if rest.is_empty() {
        return partial_sums;
    }

    let other = rest[0];
    let mut results = HashSet::with_capacity(partial_sums.len());

    for num in partial_sums.into_iter() {
        for result in arithmetic(num, other, operators) {
            results.insert(result);
        }
    }

    calculate(results, &rest[1..], operators)
}

fn can_equation_be_solved(equation: &Equation, operators: &[Op]) -> bool {
    if equation.params.is_empty() {
        return false;
    }

    if equation.params.len() == 1 {
        return equation.result == *equation.params.get(0).unwrap();
    }

    let results = calculate(HashSet::from([0]), equation.params.as_slice(), operators);

    for result in results {
        if equation.result == result {
            return true;
        }
    }

    false
}

fn sum_of_valid_equations(equations: &[Equation]) -> u64 {
    let mut sum = 0;
    for equation in equations {
        if can_equation_be_solved(equation, &[Op::Add, Op::Multiply]) {
            sum += equation.result;
        }
    }
    sum
}

fn main() {
    const INPUT: &str = include_str!("../inputs/7.txt");
    println!("Sum of valid equations: {}", sum_of_valid_equations(&parse(INPUT)));
}

const EXAMPLE_INPUT: &str =
r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

#[test]
fn parse_test() {
    let expected_equations = vec![
        Equation { result: 190, params: vec![10, 19 ] },
        Equation { result: 3267, params: vec![81, 40, 27 ] },
        Equation { result: 83, params: vec![17, 5 ] },
        Equation { result: 156, params: vec![15, 6 ] },
        Equation { result: 7290, params: vec![6, 8, 6, 15 ] },
        Equation { result: 161011, params: vec![16, 10, 13 ] },
        Equation { result: 192, params: vec![17, 8, 14 ] },
        Equation { result: 21037, params: vec![9, 7, 18, 13 ] },
        Equation { result: 292, params: vec![11, 6, 16, 20 ] },
    ];

    let sut = parse(EXAMPLE_INPUT);
    assert_eq!(sut.len(), expected_equations.len());
    for (actual, expected) in sut.iter().zip(expected_equations.iter()) {
        assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
    }
}

#[test]
fn sum_of_valid_equations_test() {
    let sut = parse(EXAMPLE_INPUT);
    assert_eq!(sum_of_valid_equations(&sut), 3749);
}
