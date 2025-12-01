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

  let count_part1 =
    input
    |> parse
    |> count_times_dial_ended_up_as_position_zero(option.None)
    |> int.to_string

  io.println("Times dial ended up in position zero: " <> count_part1)

  let count_part2 =
    input
    |> parse
    |> count_times_dial_passed_position_zero(option.None)
    |> int.to_string
  io.println("Times dial passed position zero: " <> count_part2)
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

pub fn apply_instruction_and_count_times_passed_zero(
  instruction: Instruction,
  pos: Int,
) -> #(Int, Int) {
  case instruction {
    Right(0) -> #(pos, 0)
    Left(0) -> #(pos, 0)
    x -> {
      let #(new_instruction, new_pos, times_passed_zero) = case x {
        Right(val) if pos + 1 == 100 -> #(Right(val - 1), 0, 1)
        Right(val) -> #(Right(val - 1), pos + 1, 0)
        Left(val) if pos == 0 -> #(Left(val - 1), 99, 0)
        Left(val) if pos - 1 == 0 -> #(Left(val - 1), 0, 1)
        Left(val) -> #(Left(val - 1), pos - 1, 0)
      }

      let #(pos, count) =
        apply_instruction_and_count_times_passed_zero(new_instruction, new_pos)
      #(pos, count + times_passed_zero)
    }
  }
  // let times_passed_zero = case instruction {
  //   Right(val) -> {
  //     let assert Ok(divisor) = int.divide(pos + val, 100)
  //     divisor
  //   }
  //   Left(val) -> {
  //     let assert Ok(divisor) = int.divide(val, 100)
  //     case val > pos {
  //       True -> divisor + 1
  //       False -> divisor
  //     }
  //   }
  // }

  // #(apply_instruction(instruction, pos), times_passed_zero)
}

pub fn count_times_dial_passed_position_zero(
  instructions: List(Instruction),
  pos: Option(Int),
) -> Int {
  let pos = option.unwrap(pos, 50)
  case instructions {
    [] -> 0
    [next, ..rest] -> {
      let #(pos, count_passed_position_zero) =
        apply_instruction_and_count_times_passed_zero(next, pos)

      // case count_passed_position_zero {
      //   0 -> Nil
      //   _ -> {
      //     echo next
      //     echo "^passed zero"
      //     Nil
      //   }
      // }

      count_passed_position_zero
      + count_times_dial_passed_position_zero(rest, option.Some(pos))
    }
  }
}
