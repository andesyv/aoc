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
  let input = input |> parse
  let fresh_ingredients =
    input
    |> count_fresh_ingredients
    |> int.to_string

  io.println("Fresh ingredients (part 1): " <> fresh_ingredients)

  let unique_ingredients =
    input
    |> count_unique_fresh_ingredients
    |> int.to_string

  io.println(
    "Total unique fresh ingredient count (part 2): " <> unique_ingredients,
  )
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

  // After parsing, restructure the ranges so there are none overlapping ones
  let fresh_ingredients = fresh_ingredients |> combine_ranges

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

fn count_fresh_ingredients_inner_loop(
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
      + count_fresh_ingredients_inner_loop(fresh_ranges, rest)
    }
  }
}

pub fn count_fresh_ingredients(inventory: Inventory) -> Int {
  count_fresh_ingredients_inner_loop(
    inventory.fresh_ingredients,
    inventory.available_ingredients,
  )
}

// We combine our ranges like this:
// 0. First, we sort each range based on it's minimum
// 1. Then, starting with the next range, we create a new range with the ranges minimum
// 2. Then we set the current position to the next range's max (if it's larger than the current position) and pop the next range in the list
// 3.a. If the next range in the list has a minimum lower than the current pos, continue from step 2
// 3.b. If the next range in the list has a minimum larger than the current pos, finish the new range and start over from step 1.
pub fn combine_ranges(ranges: List(#(Int, Int))) -> List(#(Int, Int)) {
  let sorted_ranges = ranges |> list.sort(fn(a, b) { int.compare(a.0, b.0) })
  case sorted_ranges {
    [first, ..rest] -> combine_ranges_inner_loop(rest, first)
    [] -> []
  }
}

fn max(left: Int, right: Int) -> Int {
  case left > right {
    True -> left
    False -> right
  }
}

fn combine_ranges_inner_loop(
  sorted_ranges: List(#(Int, Int)),
  current_range: #(Int, Int),
) -> List(#(Int, Int)) {
  case sorted_ranges {
    [next, ..rest] -> {
      // ranges are inclusive, so add a +1
      case next.0 <= current_range.1 + 1 {
        // Range is within currently built range, simply extend the max
        True -> {
          let new_max = max(next.1, current_range.1)
          combine_ranges_inner_loop(rest, #(current_range.0, new_max))
        }
        // Range is outside currently built range. Start a new range
        False -> [current_range, ..combine_ranges_inner_loop(rest, next)]
      }
    }
    [] -> [current_range]
  }
}

// fn append_list_to_set(set: set.Set(a), list: List(a)) -> set.Set(a) {
//   case list {
//     [] -> set
//     [x, ..rest] -> append_list_to_set(set.insert(set, x), rest)
//   }
// }

// fn range_to_list(range: #(Int, Int)) -> List(Int) {
//   list.range(range.0, range.1)
// }

fn count_unique_fresh_ingredients_inner_loop(
  fresh_ranges: List(#(Int, Int)),
) -> Int {
  // Given that no ranges overlap, the count of unique ingredients is simply the length of each range
  case fresh_ranges {
    [] -> 0
    [x, ..rest] ->
      case x.1 - x.0 {
        0 -> 0
        diff if diff < 0 -> panic as "How?"
        diff -> diff + 1
      }
      + count_unique_fresh_ingredients_inner_loop(rest)
  }
}

pub fn count_unique_fresh_ingredients(inventory: Inventory) -> Int {
  count_unique_fresh_ingredients_inner_loop(inventory.fresh_ingredients)
}
