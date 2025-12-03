import third

pub fn parse_test() {
  let sut =
    "987654321111111
811111111111119
234234234234278
818181911112111"

  let expected = [
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
    [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
    [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
    [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
  ]

  assert third.parse(sut) == expected
}

pub fn largest_joltage_test() {
  let bank = [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
  assert third.find_largest_joltage_in_bank(bank) == 98

  let bank = [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
  assert third.find_largest_joltage_in_bank(bank) == 89

  let bank = [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
  assert third.find_largest_joltage_in_bank(bank) == 78

  let bank = [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
  assert third.find_largest_joltage_in_bank(bank) == 92
}

pub fn sum_of_joltage_test() {
  let banks = [
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
    [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
    [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
    [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
  ]

  assert third.sum_of_joltage_from_banks(banks) == 357
}
