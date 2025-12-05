import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  io.println("Hello day 5!")
  let assert Ok(input) = simplifile.read("./inputs/5.txt")
    as "Failed to read file"
  let fresh_ingredients =
    input
    |> parse
    |> count_fresh_ingredients
    |> int.to_string

  io.println("Fresh ingredients (part 1): " <> fresh_ingredients)
}

pub type Inventory {
  Inventory(
    fresh_ingredients: List(#(Int, Int)),
    available_ingredients: List(Int),
  )
}

pub fn parse(input: String) -> Inventory {
  let assert Ok(#(ranges_str, ingredients_str)) =
    string.split_once(input, "\n\n")

  let fresh_ingredients =
    ranges_str
    |> string.split("\n")
    |> list.filter_map(fn(line) {
      case line |> string.trim |> string.to_option {
        option.Some(line) -> {
          use #(start_str, end_str) <- result.try(string.split_once(line, "-"))
          case start_str |> int.parse {
            Ok(start) ->
              case end_str |> int.parse {
                Ok(end) -> Ok(#(start, end))
                _ -> Error(Nil)
              }
            _ -> Error(Nil)
          }
        }
        _ -> Error(Nil)
      }
    })

  let available_ingredients =
    ingredients_str
    |> string.split("\n")
    |> list.filter_map(fn(line) {
      case line |> string.trim |> string.to_option {
        option.Some(line) -> line |> int.parse
        _ -> Error(Nil)
      }
    })

  Inventory(fresh_ingredients:, available_ingredients:)
}

fn is_fresh(fresh_ranges: List(#(Int, Int)), ingredient: Int) -> Bool {
  case fresh_ranges {
    [] -> False
    [#(min, max), ..rest] ->
      ingredient >= min && ingredient <= max || is_fresh(rest, ingredient)
  }
}

fn count_fresh_ingredients_2(
  fresh_ranges: List(#(Int, Int)),
  ingredients: List(Int),
) -> Int {
  case ingredients {
    [] -> 0
    [next, ..rest] -> {
      case is_fresh(fresh_ranges, next) {
        True -> 1
        False -> 0
      }
      + count_fresh_ingredients_2(fresh_ranges, rest)
    }
  }
}

pub fn count_fresh_ingredients(inventory: Inventory) -> Int {
  count_fresh_ingredients_2(
    inventory.fresh_ingredients,
    inventory.available_ingredients,
  )
}
