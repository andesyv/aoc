import gleam/int
import gleam/io
import gleam/list
import gleam/string
import second
import simplifile

pub fn main() {
  io.println("Hello day 6!")
  let assert Ok(input) = simplifile.read("./inputs/6.txt")
    as "Failed to read file"
  let grand_total =
    input
    |> parse
    |> calc_grand_total
    |> int.to_string

  io.println("Grand total (part 1): " <> grand_total)
}

pub type Op {
  Addition
  Multiplication
}

pub type Problem =
  #(List(Int), Op)

pub fn parse(input: String) -> List(Problem) {
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
