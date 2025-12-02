import gleam/erlang/process
import gleam/float
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import gleam_community/maths
import simplifile
import tempo/duration
import tempo/instant

pub fn main() {
  io.println("Hello day 2!")
  let assert Ok(input) = simplifile.read("./inputs/2.txt")
    as "Failed to read file"

  let input = input |> parse

  let timer = instant.now()
  let count_part1 =
    input
    |> sum_of_invalid_indices_parallel(find_invalid_ids_in_range_part1, 100)
    |> int.to_string
  let elapsed_ms =
    timer |> instant.since |> duration.as_milliseconds |> int.to_string

  io.println("Sum of invalid IDs part 1: " <> count_part1)
  io.println("(It took " <> elapsed_ms <> "ms to calculate)")

  let timer = instant.now()
  let count_part2 =
    input
    |> sum_of_invalid_indices_parallel(find_invalid_ids_in_range_part2, 100)
    |> int.to_string
  let elapsed_ms =
    timer |> instant.since |> duration.as_milliseconds |> int.to_string

  io.println("Sum of invalid IDs part 2: " <> count_part2)
  io.println("(It took " <> elapsed_ms <> "ms to calculate)")
}

// Helper to make syntax cleaner
fn strip(s: String) -> Result(String, Nil) {
  s
  |> string.trim()
  |> string.to_option
  |> option.to_result(Nil)
}

fn integer_power(num: Int, n: Int) -> Int {
  assert n >= 0
  case n == 0 {
    True -> 1
    False -> num * integer_power(num, n - 1)
  }
}

fn parse_line(line: String) -> Result(#(Int, Int), Nil) {
  use line <- result.try(strip(line))
  use #(begin_str, end_str) <- result.try(string.split_once(line, "-"))
  use begin <- result.try(int.parse(begin_str))
  use end <- result.try(int.parse(end_str))
  Ok(#(begin, end))
}

pub fn parse(input: String) -> List(#(Int, Int)) {
  input
  |> string.split(",")
  |> list.filter_map(parse_line)
}

fn digits(num: Int) -> Int {
  num
  |> int.to_float
  |> maths.logarithm_10
  |> result.unwrap(0.0)
  |> float.add(1.0)
  |> float.floor
  |> float.round
}

pub fn is_invalid_id_part1(id: Int) -> Bool {
  let digits = digits(id)

  // Kind of a weird syntax, but gleam only has if-else statements through case guards...
  case True {
    _ if id == 0 -> False
    _ if digits % 2 != 0 -> False
    _ -> {
      let decimal_divider = integer_power(10, digits / 2)
      let upper_half = id / decimal_divider
      let lower_half = id - upper_half * decimal_divider
      upper_half == lower_half
    }
  }
}

// pub fn prefix_is_repeating_in_number(prefix: Int, number: Int) -> Bool {
//   case number {
//     0 -> True
//     _ if prefix > number -> False
//     _ -> {
//       let prefix_divider = integer_power(10, digits(number) - digits(prefix))
//       prefix == number / prefix_divider
//       && prefix_is_repeating_in_number(
//         prefix,
//         number - { number / prefix_divider } * prefix_divider,
//       )
//     }
//   }
// }

pub fn prefix_is_repeating(prefix: String, number: String) -> Bool {
  string.is_empty(number)
  || string.starts_with(number, prefix)
  && prefix_is_repeating(
    prefix,
    string.drop_start(number, string.length(prefix)),
  )
}

fn number_matches_on_prefix_length(prefix_digits: Int, number: Int) -> Bool {
  case prefix_digits == 0 {
    True -> False
    False -> {
      let number_as_string = int.to_string(number)
      prefix_is_repeating(
        string.slice(number_as_string, 0, prefix_digits),
        number_as_string,
      )
      || number_matches_on_prefix_length(prefix_digits - 1, number)
    }
  }
}

pub fn number_matches_on_itself(number: Int) -> Bool {
  case digits(number) {
    0 | 1 -> False
    digits -> number_matches_on_prefix_length(digits - 1, number)
  }
}

pub fn find_invalid_ids_in_range(
  range: #(Int, Int),
  invalid_id_predicate: fn(Int) -> Bool,
) -> List(Int) {
  case range.0 <= range.1 {
    False -> []
    True -> {
      let rest =
        find_invalid_ids_in_range(#(range.0 + 1, range.1), invalid_id_predicate)
      case invalid_id_predicate(range.0) {
        True -> [range.0, ..rest]
        False -> rest
      }
    }
  }
}

// Gleam doesn't support "currying", which would simply let us do 
// let find_invalid_ids_in_range_part1 = find_invalid_ids_in_range(is_invalid_id_part1)
pub fn find_invalid_ids_in_range_part1(range: #(Int, Int)) -> List(Int) {
  find_invalid_ids_in_range(range, is_invalid_id_part1)
}

pub fn find_invalid_ids_in_range_part2(range: #(Int, Int)) -> List(Int) {
  find_invalid_ids_in_range(range, number_matches_on_itself)
}

pub fn sum_of_invalid_indices(
  ranges: List(#(Int, Int)),
  invalid_id_range_finder: fn(#(Int, Int)) -> List(Int),
) -> Int {
  ranges
  |> list.flat_map(fn(range) { invalid_id_range_finder(range) })
  |> list.fold(0, int.add)
}

/// Splits a range to a subrange of segments of max 100 length
pub fn split_range(range: #(Int, Int), max_len: Int) -> List(#(Int, Int)) {
  case range.1 - range.0 > max_len {
    True -> [
      #(range.0, range.0 + max_len),
      ..split_range(#(range.0 + max_len + 1, range.1), max_len)
    ]
    False -> [range]
  }
}

// Making this version is obviously only for my own learning interest. Interestingly, Erlang
// actually runs this faster compared to the sequal version above (even taking into account
// thread startup and shutdown)
pub fn sum_of_invalid_indices_parallel(
  ranges: List(#(Int, Int)),
  invalid_id_range_finder: fn(#(Int, Int)) -> List(Int),
  segment_length: Int,
) -> Int {
  // First, split the ranges into smaller subranges (for parallel efficiency)
  let ranges =
    ranges |> list.flat_map(fn(range) { split_range(range, segment_length) })

  // Now split the operation into multiple "green" threads
  let subject = process.new_subject()
  ranges
  |> list.map(fn(range) {
    process.spawn(fn() { process.send(subject, invalid_id_range_finder(range)) })
  })
  |> list.flat_map(fn(_pid) { process.receive_forever(subject) })
  |> list.fold(0, int.add)
}
