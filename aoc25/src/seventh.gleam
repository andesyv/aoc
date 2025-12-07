import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/set.{type Set}
import gleam/string
import simplifile

pub fn main() {
  io.println("Hello day 7!")
  let assert Ok(input) = simplifile.read("./inputs/7.txt")
    as "Failed to read file"
  let split_count =
    input
    |> parse
    |> get_split_count
    |> int.to_string

  io.println("Split count (part 1): " <> split_count)
}

pub type Pos =
  #(Int, Int)

pub type Map {
  Map(start: Pos, splitters: Set(Pos), length: Int)
}

type Token {
  Start
  Splitter
}

fn parse_line(line: String, x: Int) -> List(#(Token, Int)) {
  case line {
    "." <> rest -> parse_line(rest, x + 1)
    "S" <> rest -> [#(Start, x), ..parse_line(rest, x + 1)]
    "^" <> rest -> [#(Splitter, x), ..parse_line(rest, x + 1)]
    _ -> []
  }
}

pub fn parse(input: String) -> Map {
  let lines =
    input |> string.split("\n") |> list.map(fn(line) { parse_line(line, 0) })

  let tokens =
    lines
    |> list.zip(list.range(0, list.length(lines)))
    |> list.flat_map(fn(tokens_and_line_num) {
      let #(tokens, y) = tokens_and_line_num
      tokens
      |> list.map(fn(token_and_horizontal_pos) {
        #(token_and_horizontal_pos.0, #(token_and_horizontal_pos.1, y))
      })
    })

  let assert Ok(#(_, start)) =
    tokens |> list.find(fn(token_and_pos) { token_and_pos.0 == Start })

  let splitters =
    tokens
    |> list.filter_map(fn(token_and_pos) {
      let #(token, pos) = token_and_pos
      case token {
        Splitter -> Ok(pos)
        _ -> Error(Nil)
      }
    })

  Map(start:, splitters: set.from_list(splitters), length: list.length(lines))
}

fn simulate_step(map: Map, beams: List(Pos)) -> #(Set(Pos), Int) {
  case beams {
    [#(x, y), ..rest] -> {
      let next_beam_pos = #(x, y + 1)
      let #(new_positions, splits) = simulate_step(map, rest)
      case set.contains(map.splitters, next_beam_pos) {
        True -> #(
          set.insert(
            set.insert(new_positions, #(next_beam_pos.0 - 1, next_beam_pos.1)),
            #(next_beam_pos.0 + 1, next_beam_pos.1),
          ),
          splits + 1,
        )
        False -> #(set.insert(new_positions, next_beam_pos), splits)
      }
    }
    [] -> #(set.new(), 0)
  }
}

fn simulate_loop(
  map: Map,
  beams: List(Pos),
  simulate_steps: Int,
  current_split_count: Int,
) -> #(List(Pos), Int) {
  use <- bool.guard(simulate_steps <= 0, #(beams, current_split_count))
  let #(new_positions, splits) = simulate_step(map, beams)
  simulate_loop(
    map,
    set.to_list(new_positions),
    simulate_steps - 1,
    current_split_count + splits,
  )
}

pub fn get_split_count(map: Map) -> Int {
  let #(_final_positions, splits) =
    simulate_loop(map, [map.start], map.length, 0)
  splits
}
