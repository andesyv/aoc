import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() {
  io.println("Hello day 3!")
  let assert Ok(input) = simplifile.read("./inputs/3.txt")
    as "Failed to read file"
  let input = input |> parse
  let joltage_part1 =
    input
    |> sum_of_joltage_from_banks_part1
    |> int.to_string

  io.println("Total output joltage (part 1): " <> joltage_part1)

  let joltage_part2 =
    input
    |> sum_of_joltage_from_banks_part2
    |> int.to_string

  io.println("Total output joltage (part 2): " <> joltage_part2)
}

pub fn parse(input: String) -> List(List(Int)) {
  input
  |> string.split("\n")
  |> list.map(fn(line) {
    line
    |> string.to_graphemes
    |> list.filter_map(int.parse)
  })
}

fn integer_power(num: Int, n: Int) -> Int {
  assert n >= 0
  case n == 0 {
    True -> 1
    False -> num * integer_power(num, n - 1)
  }
}

fn find_largest_digit(bank: List(Int), battery_count: Int) -> #(Int, List(Int)) {
  case list.length(bank) < battery_count, bank {
    True, _ -> #(0, [])
    False, [] -> #(0, [])
    // False, [x] -> x
    False, [x, ..rest] -> {
      let subresult = find_largest_digit(rest, battery_count)
      case x >= subresult.0 {
        True -> #(x, rest)
        False -> subresult
      }
    }
  }
}

pub fn find_largest_joltage_in_bank(bank: List(Int), battery_count: Int) -> Int {
  // We'll make use of the fact that a larger digit on the very left of an exact digit number,
  // is always greater than another number with a smaller digit to the very left.
  case battery_count {
    0 -> 0
    _ -> {
      let #(digit, rest) = find_largest_digit(bank, battery_count)
      digit
      * integer_power(10, battery_count - 1)
      + find_largest_joltage_in_bank(rest, battery_count - 1)
    }
  }
}

pub fn sum_of_joltage_from_banks_part1(banks: List(List(Int))) -> Int {
  banks
  |> list.map(fn(bank) { find_largest_joltage_in_bank(bank, 2) })
  |> list.fold(0, int.add)
}

pub fn sum_of_joltage_from_banks_part2(banks: List(List(Int))) -> Int {
  banks
  |> list.map(fn(bank) { find_largest_joltage_in_bank(bank, 12) })
  |> list.fold(0, int.add)
}
