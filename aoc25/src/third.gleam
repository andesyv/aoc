import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  io.println("Hello day 3!")
  let assert Ok(input) = simplifile.read("./inputs/3.txt")
    as "Failed to read file"
  let input = input |> parse
  let joltage_part1 =
    input
    |> sum_of_joltage_from_banks
    |> int.to_string

  io.println("Total output joltage: " <> joltage_part1)
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

fn calc_joltage(left_battery: Int, right_battery: Int) -> Int {
  left_battery * 10 + right_battery
}

pub fn find_largest_joltage_in_bank(bank: List(Int)) -> Int {
  // Our logic is simple:
  // We start with the left-most number. Then we find the largest number of the remaining ones calculate the "joltage".
  // Finally, we continue this process down the list and return the largest value

  case bank {
    [] -> 0
    [left, right] -> calc_joltage(left, right)
    [left, ..rest] -> {
      let max = rest |> list.max(int.compare) |> result.unwrap(0)
      let joltage = calc_joltage(left, max)
      let sub_joltage = find_largest_joltage_in_bank(rest)
      case joltage > sub_joltage {
        True -> joltage
        False -> sub_joltage
      }
    }
  }
}

pub fn sum_of_joltage_from_banks(banks: List(List(Int))) -> Int {
  banks
  |> list.map(find_largest_joltage_in_bank)
  |> list.fold(0, int.add)
}
