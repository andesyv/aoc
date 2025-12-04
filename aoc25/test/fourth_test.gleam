import fourth
import gleam/set

pub fn parse_test() {
  let sut =
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."

  let parsed_map = fourth.parse(sut)
  // Can't be bothered to write out the entire map, so I'll just check specific ones:

  assert !set.contains(parsed_map, #(0, 0))
  assert !set.contains(parsed_map, #(1, 0))
  assert set.contains(parsed_map, #(2, 0))
  assert set.contains(parsed_map, #(3, 0))
  assert !set.contains(parsed_map, #(4, 0))

  assert set.contains(parsed_map, #(0, 1))
  assert set.contains(parsed_map, #(1, 1))
}

pub fn count_accessible_rolls_of_paper_test() {
  let sut =
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."

  assert fourth.count_accessible_paper_rolls(fourth.parse(sut)) == 13
}

pub fn count_total_paper_rolls_that_can_be_removed_test() {
  let sut =
    "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."

  assert fourth.iteratively_remove_accessible_paper_rolls(fourth.parse(sut), 0)
    == 43
}
