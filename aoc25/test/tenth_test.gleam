import tenth

pub fn parse_test() {
  let sut =
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"

  assert tenth.parse(sut)
    == [
      tenth.Machine(
        indicator_lights: [False, True, True, False],
        wiring_schematics: [[3], [1, 3], [2], [2, 3], [0, 2], [0, 1]],
        joltage_requirements: [3, 5, 4, 7],
      ),
      tenth.Machine(
        indicator_lights: [False, False, False, True, False],
        wiring_schematics: [
          [0, 2, 3, 4],
          [2, 3],
          [0, 4],
          [0, 1, 2],
          [1, 2, 3, 4],
        ],
        joltage_requirements: [7, 5, 12, 7, 2],
      ),
      tenth.Machine(
        indicator_lights: [False, True, True, True, False, True],
        wiring_schematics: [[0, 1, 2, 3, 4], [0, 3, 4], [0, 1, 2, 4, 5], [1, 2]],
        joltage_requirements: [10, 11, 11, 5, 10, 5],
      ),
    ]
}

pub fn to_binary_test() {
  assert tenth.indicator_light_to_binary([True, False]) == 2
  assert tenth.indicator_light_to_binary([True, False, False]) == 4
  assert tenth.indicator_light_to_binary([False, True, False, True]) == 5
  assert tenth.indicator_light_to_binary([True, True, False, True]) == 13

  assert tenth.button_to_binary(1, [0]) == 1
  assert tenth.button_to_binary(2, [1]) == 1
  assert tenth.button_to_binary(2, [0, 1]) == 3
  assert tenth.button_to_binary(3, [0, 1, 2]) == 7
  assert tenth.button_to_binary(3, [0, 2]) == 5
  assert tenth.button_to_binary(3, [2]) == 1

  assert tenth.toggle_light(
      tenth.indicator_light_to_binary([True, False]),
      tenth.button_to_binary(2, [0]),
    )
    == 0

  assert tenth.toggle_light(
      tenth.indicator_light_to_binary([False, True]),
      tenth.button_to_binary(2, [1]),
    )
    == 0

  assert tenth.toggle_light(
      tenth.indicator_light_to_binary([True, True]),
      tenth.button_to_binary(2, [0, 1]),
    )
    == 0

  assert tenth.toggle_light(
      tenth.indicator_light_to_binary([True, False, True]),
      tenth.button_to_binary(3, [2]),
    )
    == tenth.indicator_light_to_binary([True, False, False])
}

pub fn min_button_presses_to_activate_machine_test() {
  let assert [machine] =
    tenth.parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
  assert tenth.get_min_button_presses_to_active_machine(machine) == 2

  let assert [machine] =
    tenth.parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
  assert tenth.get_min_button_presses_to_active_machine(machine) == 3

  let assert [machine] =
    tenth.parse(
      "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    )
  assert tenth.get_min_button_presses_to_active_machine(machine) == 2

  let sut =
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
  [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
  [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"

  assert tenth.sum_of_min_required_button_presses_to_activate_machines(
      tenth.parse(sut),
    )
    == 7
}
