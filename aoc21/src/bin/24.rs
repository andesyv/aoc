// Merry christmas!
use itertools::Itertools;
use std::collections::VecDeque;

enum Value {
    Literal(i64),
    Variable(usize),
}

enum Instruction {
    Inp(Value),
    Add(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),
    Eql(Value, Value),
}

fn parse_variable(input: &str) -> Option<Value> {
    let c = input.chars().next()?;
    match c {
        'w'..='z' => Some(Value::Variable(c as usize - 'w' as usize)),
        _ => Some(Value::Literal(input.parse().ok()?)),
    }
}

fn parse_instruction(input: &str) -> Option<Instruction> {
    use Instruction::{Add, Div, Eql, Inp, Mod, Mul};
    let (cmd, rest) = input.split_once(' ')?;

    let pat = cmd.chars().collect::<Vec<char>>();
    if let ['i', 'n', 'p', ..] = &pat[..] {
        Some(Inp(parse_variable(rest)?))
    } else {
        let (a, b) = rest.split(' ').map(parse_variable).collect_tuple()?;

        match &pat[..] {
            ['a', 'd', 'd', ..] => Some(Add(a?, b?)),
            ['m', 'u', 'l', ..] => Some(Mul(a?, b?)),
            ['d', 'i', 'v', ..] => Some(Div(a?, b?)),
            ['m', 'o', 'd', ..] => Some(Mod(a?, b?)),
            ['e', 'q', 'l', ..] => Some(Eql(a?, b?)),
            _ => None,
        }
    }
}

fn get_input(input_stream: &mut VecDeque<i64>) -> i64 {
    input_stream.pop_front().unwrap_or(0)
}

fn get_second_param(memory: &[i64; 4], v: &Value) -> i64 {
    match v {
        Value::Variable(i) => memory[*i],
        Value::Literal(n) => *n,
    }
}

fn run_instructions(instructions: &Vec<Instruction>, mut inputs: VecDeque<i64>) -> [i64; 4] {
    use Instruction::{Add, Div, Eql, Inp, Mod, Mul};
    use Value::Variable;
    let mut memory = [0; 4];
    for op in instructions {
        match op {
            Inp(Variable(i)) => memory[*i] = get_input(&mut inputs),
            Add(Variable(i), b) => memory[*i] = memory[*i] + get_second_param(&memory, b),
            Mul(Variable(i), b) => memory[*i] = memory[*i] * get_second_param(&memory, b),
            Div(Variable(i), b) => memory[*i] = memory[*i] / get_second_param(&memory, b),
            Mod(Variable(i), b) => memory[*i] = memory[*i] % get_second_param(&memory, b),
            Eql(Variable(i), b) => {
                memory[*i] = if memory[*i] == get_second_param(&memory, b) {
                    1
                } else {
                    0
                }
            }
            _ => (),
        }
    }

    memory
}

struct ALU {
    instructions: Vec<Instruction>,
}

impl ALU {
    fn eval(&self, inputs: VecDeque<i64>) -> [i64; 4] {
        run_instructions(&self.instructions, inputs)
    }

    fn new(programming: &str) -> ALU {
        ALU {
            instructions: programming.lines().filter_map(parse_instruction).collect(),
        }
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/24.txt");
    let alu = ALU::new(INPUT);

    const MAX: u128 = 99999999999999;
    for i in 0..=MAX {
        let num = MAX - i;
        let n = num.to_string();
        if n.chars().any(|c| c == '0') {
            continue;
        }
        let inputs: VecDeque<i64> = n
            .chars()
            .map(|c| i64::try_from(c.to_digit(10).unwrap()).unwrap())
            .collect();
        let output = alu.eval(inputs);
        // println!("Inputs: {:?}, outputs: {:?}", inputs, output);
        if output[3] == 0 {
            println!("Found a valid model number: {}", num);
            break;
        }
    }
}

#[test]
fn test1() {
    const INPUT: &str = "inp x\nmul x -1";
    let alu = ALU::new(INPUT);

    for n in 0..5 {
        let output = alu.eval(VecDeque::from(vec![n]));
        assert_eq!(output, [0, -n, 0, 0]);
    }
}

#[test]
fn test2() {
    const INPUT: &str = "inp z\ninp x\nmul z 3\neql z x";
    let alu = ALU::new(INPUT);

    for n in 1..4 {
        let output = alu.eval(VecDeque::from(vec![n, n * 3]));
        assert_eq!(output[3], 1);
    }
}

#[test]
fn test3() {
    const INPUT: &str = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2";
    let alu = ALU::new(INPUT);

    for n in 0..7 {
        let output = alu.eval(VecDeque::from(vec![n]));
        let binary = output
            .iter()
            .map(|n| char::from_digit(u32::try_from(*n).unwrap(), 10).unwrap())
            .collect::<String>();
        assert_eq!(n, i64::from_str_radix(&binary[..], 2).unwrap());
    }
}
