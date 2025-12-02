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

  let timer = instant.now()
  let count_part1 =
    input
    |> parse
    |> sum_of_invalid_indices_parallel
    |> int.to_string
  let elapsed_ms =
    timer |> instant.since |> duration.as_milliseconds |> int.to_string

  io.println("Times dial ended up in position zero: " <> count_part1)
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

pub fn is_invalid_id(id: Int) -> Bool {
  let digits =
    id
    |> int.to_float
    |> maths.logarithm_10
    |> result.unwrap(0.0)
    |> float.add(1.0)
    |> float.floor
    |> float.round

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

pub fn find_invalid_ids_in_range(begin: Int, end: Int) -> List(Int) {
  case begin <= end {
    False -> []
    True -> {
      let rest = find_invalid_ids_in_range(begin + 1, end)
      case is_invalid_id(begin) {
        True -> [begin, ..rest]
        False -> rest
      }
    }
  }
}

pub fn sum_of_invalid_indices(ranges: List(#(Int, Int))) -> Int {
  ranges
  |> list.flat_map(fn(range) { find_invalid_ids_in_range(range.0, range.1) })
  |> list.fold(0, int.add)
}

/// Splits a range to a subrange of segments of max 100 length
pub fn split_range(range: #(Int, Int)) -> List(#(Int, Int)) {
  case range.1 - range.0 > 100 {
    True -> [
      #(range.0, range.0 + 100),
      ..split_range(#(range.0 + 101, range.1))
    ]
    False -> [range]
  }
}

// Making this version is obviously only for my own learning interest. Interestingly, Erlang
// actually runs this faster compared to the sequal version above (even taking into account
// thread startup and shutdown)
pub fn sum_of_invalid_indices_parallel(ranges: List(#(Int, Int))) -> Int {
  // First, split the ranges into smaller subranges (for parallel efficiency)
  let ranges = ranges |> list.flat_map(split_range)

  // Now split the operation into multiple "green" threads
  let subject = process.new_subject()
  ranges
  |> list.map(fn(range) {
    process.spawn(fn() {
      process.send(subject, find_invalid_ids_in_range(range.0, range.1))
    })
  })
  |> list.flat_map(fn(_pid) { process.receive_forever(subject) })
  |> list.fold(0, int.add)
}
