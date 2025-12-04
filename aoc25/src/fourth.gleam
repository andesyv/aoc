import gleam/int
import gleam/io
import gleam/list
import gleam/set.{type Set}
import gleam/string
import simplifile

pub fn main() {
  io.println("Hello day 4!")
  let assert Ok(input) = simplifile.read("./inputs/4.txt")
    as "Failed to read file"
  let accessible_paper_rolls =
    input
    |> parse
    |> count_accessible_paper_rolls
    |> int.to_string

  io.println("Accessible paper rolls (part 1): " <> accessible_paper_rolls)
}

fn parse_line(line: String, counter: Int) -> List(Int) {
  case line {
    "@" <> rest -> [counter, ..parse_line(rest, counter + 1)]
    "." <> rest -> parse_line(rest, counter + 1)
    _ -> []
  }
}

pub fn parse(input: String) -> Set(#(Int, Int)) {
  let lines = input |> string.split("\n")
  // In a similar language like Haskell, you'd typically add indices to a list by zipping
  // it together with an infinite list. However, gleam does not support infinite lists.
  // So we'll have to do this instead:
  lines
  |> list.zip(list.range(0, list.length(lines)))
  |> list.flat_map(fn(entry: #(String, Int)) {
    let #(line, y) = entry
    parse_line(line, 0)
    |> list.map(fn(x) { #(x, y) })
  })
  |> set.from_list
}

fn get_adjacents(pos: #(Int, Int)) -> List(#(Int, Int)) {
  [
    // Cardinals
    #(pos.0 + 1, pos.1),
    #(pos.0, pos.1 + 1),
    #(pos.0 - 1, pos.1),
    #(pos.0, pos.1 - 1),

    // Diagonals
    #(pos.0 + 1, pos.1 + 1),
    #(pos.0 + 1, pos.1 - 1),
    #(pos.0 - 1, pos.1 + 1),
    #(pos.0 - 1, pos.1 - 1),
  ]
}

fn can_be_accessed(
  map: Set(#(Int, Int)),
  adjacent_positions: List(#(Int, Int)),
  current_count: Int,
) -> Bool {
  case current_count {
    count if count >= 4 -> False
    _ -> {
      case adjacent_positions {
        [] -> True
        [next, ..rest] -> {
          let count = case set.contains(map, next) {
            True -> current_count + 1
            False -> current_count
          }
          can_be_accessed(map, rest, count)
        }
      }
    }
  }
}

pub fn count_accessible_paper_rolls(map: Set(#(Int, Int))) -> Int {
  map
  |> set.to_list
  |> list.filter(fn(pos) { can_be_accessed(map, get_adjacents(pos), 0) })
  |> list.length
}
