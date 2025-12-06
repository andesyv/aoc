import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import second
import simplifile

pub fn main() {
  io.println("Hello day 6!")
  let assert Ok(input) = simplifile.read("./inputs/6.txt")
    as "Failed to read file"
  let grand_total =
    input
    |> parse_part1
    |> calc_grand_total
    |> int.to_string

  io.println("Grand total (part 1): " <> grand_total)

  let grand_total =
    input
    |> parse_part2
    |> calc_grand_total
    |> int.to_string

  io.println("Grand total (part 2): " <> grand_total)
}

pub type Op {
  Addition
  Multiplication
}

pub type Problem =
  #(List(Int), Op)

pub fn parse_part1(input: String) -> List(Problem) {
  let lines = input |> string.split("\n") |> list.filter_map(second.strip)

  let numbers =
    lines
    |> list.take(list.length(lines) - 1)
    |> list.map(fn(line) {
      line
      |> string.split(" ")
      |> list.filter_map(second.strip)
      |> list.filter_map(fn(col) { col |> int.parse })
    })
    |> list.transpose

  let operators = case lines |> list.drop(list.length(lines) - 1) {
    [line] ->
      line
      |> string.split(" ")
      |> list.filter_map(second.strip)
      |> list.filter_map(fn(s) {
        case s {
          "+" -> Ok(Addition)
          "*" -> Ok(Multiplication)
          _ -> Error(Nil)
        }
      })
    _ -> panic as "Failed to parse"
  }

  numbers |> list.zip(operators)
}

fn parse_number_and_operator(
  segment: String,
  number_part: String,
) -> Result(#(Int, option.Option(Op)), Nil) {
  case string.pop_grapheme(segment) {
    Error(_) -> {
      use number <- result.map(number_part |> string.trim |> int.parse)
      #(number, option.None)
    }
    Ok(#("+", _)) -> {
      use number <- result.map(number_part |> string.trim |> int.parse)
      #(number, option.Some(Addition))
    }
    Ok(#("*", _)) -> {
      use number <- result.map(number_part |> string.trim |> int.parse)
      #(number, option.Some(Multiplication))
    }
    Ok(#(char, rest)) ->
      parse_number_and_operator(rest, string.append(number_part, char))
  }
}

fn parse_problems(
  columns: List(String),
  current_numbers: List(Int),
) -> List(Problem) {
  case columns {
    [] -> []
    [next, ..rest] ->
      case parse_number_and_operator(next, "") {
        Ok(#(number, option.None)) ->
          parse_problems(rest, [number, ..current_numbers])
        Ok(#(number, option.Some(op))) -> [
          #(list.reverse([number, ..current_numbers]), op),
          ..parse_problems(rest, [])
        ]
        _ -> parse_problems(rest, current_numbers)
      }
  }
}

pub fn parse_part2(input: String) -> List(Problem) {
  // - The amount of spaces between the operators is the same as the column widths
  // - The lines are the same length (might not need to ever split on \n)
  // What if I parse "downwards" instead, from right to left?
  // E.g. "  4 ", "431 ", "623+", "    "

  let columns =
    input
    |> string.split("\n")
    |> list.map(string.to_utf_codepoints)
    |> list.transpose
    |> list.map(string.from_utf_codepoints)
    |> list.reverse

  // echo columns

  parse_problems(columns, [])
}

fn solve_problem(problem: Problem) -> Int {
  let #(fold_op, init) = case problem.1 {
    Addition -> #(int.add, 0)
    Multiplication -> #(int.multiply, 1)
  }

  problem.0 |> list.fold(init, fold_op)
}

pub fn calc_grand_total(problems: List(Problem)) -> Int {
  problems
  |> list.map(solve_problem)
  |> list.fold(0, int.add)
}
