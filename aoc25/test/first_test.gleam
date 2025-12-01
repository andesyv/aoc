import first
import gleam/option

pub fn parse_test() {
  let sut =
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
  let expected = [
    first.Left(68),
    first.Left(30),
    first.Right(48),
    first.Left(5),
    first.Right(60),
    first.Left(55),
    first.Left(1),
    first.Left(99),
    first.Right(14),
    first.Left(82),
  ]
  assert first.parse(sut) == expected
}

pub fn example_instruction_test() {
  assert first.apply_instruction(first.Right(8), 11) == 19
  assert first.apply_instruction(first.Left(19), 19) == 0

  assert first.apply_instruction(first.Left(1), 0) == 99
  assert first.apply_instruction(first.Right(1), 99) == 0

  assert first.apply_instruction(first.Left(10), 5) == 95
  assert first.apply_instruction(first.Right(5), 95) == 0
}

pub fn count_times_dial_ended_up_as_position_zero_test() {
  let input = [
    first.Left(68),
    first.Left(30),
    first.Right(48),
    first.Left(5),
    first.Right(60),
    first.Left(55),
    first.Left(1),
    first.Left(99),
    first.Right(14),
    first.Left(82),
  ]

  assert first.count_times_dial_ended_up_as_position_zero(input, option.None)
    == 3
}
