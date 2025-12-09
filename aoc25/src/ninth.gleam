import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  io.println("Hello day 9!")

  let assert Ok(input) = simplifile.read("./inputs/9.txt")
    as "Failed to read file"
  let largest_area = input |> parse |> get_largest_square_area |> int.to_string

  io.println("Largest area (part 1): " <> largest_area)
}

pub type Pos =
  #(Int, Int)

pub fn parse(input: String) -> List(Pos) {
  input
  |> string.split("\n")
  |> list.filter_map(fn(line) {
    use #(x, y) <- result.try(line |> string.split_once(","))
    use x <- result.try(x |> int.parse)
    use y <- result.try(y |> int.parse)
    Ok(#(x, y))
  })
}

pub fn get_largest_square_area(tile_positions: List(Pos)) -> Int {
  let assert Ok(size) =
    tile_positions
    |> list.combination_pairs
    |> list.map(fn(entry) {
      let #(left, right) = entry
      { int.absolute_value(right.0 - left.0) + 1 }
      * { int.absolute_value(right.1 - left.1) + 1 }
    })
    |> list.max(int.compare)
  size
}
