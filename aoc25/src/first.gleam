import gleam/int
import gleam/io
import gleam/list
import gleam/option.{type Option}
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  io.println("Hello day 1!")
  let assert Ok(input) = simplifile.read("./inputs/1.txt")
    as "Failed to read file"

  let count =
    input
    |> parse
    |> count_times_dial_ended_up_as_position_zero(option.None)
    |> int.to_string

  io.println("Times dial ended up in position zero: " <> count)
}

pub type Instruction {
  Left(Int)
  Right(Int)
}

fn parse_line(line: String) -> Result(Instruction, Nil) {
  case string.trim(line) {
    "L" <> rest -> result.map(int.base_parse(rest, 10), fn(val) { Left(val) })
    "R" <> rest -> result.map(int.base_parse(rest, 10), fn(val) { Right(val) })
    _ -> Error(Nil)
  }
}

pub fn parse(input: String) -> List(Instruction) {
  input
  |> string.split("\n")
  |> list.filter_map(parse_line)
}

pub fn apply_instruction(instruction: Instruction, pos: Int) -> Int {
  let val = case instruction {
    Right(val) -> val
    Left(val) -> -val
  }

  let assert Ok(result) = int.modulo(pos + val, 100)
  result
}

pub fn count_times_dial_ended_up_as_position_zero(
  instructions: List(Instruction),
  pos: Option(Int),
) -> Int {
  let pos = option.unwrap(pos, 50)
  case instructions {
    [] -> 0
    [next, ..rest] -> {
      let pos = apply_instruction(next, pos)
      case pos {
        0 -> 1
        _ -> 0
      }
      + count_times_dial_ended_up_as_position_zero(rest, option.Some(pos))
    }
  }
}
