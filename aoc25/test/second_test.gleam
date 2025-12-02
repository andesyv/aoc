import second

// import tempo/instant

pub fn parse_test() {
  let sut =
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"

  let expected = [
    #(11, 22),
    #(95, 115),
    #(998, 1012),
    #(1_188_511_880, 1_188_511_890),
    #(222_220, 222_224),
    #(1_698_522, 1_698_528),
    #(446_443, 446_449),
    #(38_593_856, 38_593_862),
    #(565_653, 565_659),
    #(824_824_821, 824_824_827),
    #(2_121_212_118, 2_121_212_124),
  ]

  assert second.parse(sut) == expected
}

pub fn valid_id_part1_test() {
  assert !second.is_invalid_id_part1(0)
  assert !second.is_invalid_id_part1(1)
  assert !second.is_invalid_id_part1(0101)

  assert !second.is_invalid_id_part1(10)
  assert second.is_invalid_id_part1(11)
  assert second.is_invalid_id_part1(22)
  assert !second.is_invalid_id_part1(23)

  assert !second.is_invalid_id_part1(95)
  assert second.is_invalid_id_part1(99)

  assert !second.is_invalid_id_part1(222_224)
  assert second.is_invalid_id_part1(222_222)
}

pub fn valid_id_part2_test() {
  assert second.prefix_is_repeating("1", "11")
  assert second.prefix_is_repeating("1", "111")
  assert !second.prefix_is_repeating("1", "2111")

  assert second.prefix_is_repeating("12", "1212")

  assert second.prefix_is_repeating("123", "123123")

  assert !second.number_matches_on_itself(0)
  assert !second.number_matches_on_itself(1)

  assert !second.number_matches_on_itself(10)
  assert second.number_matches_on_itself(11)
  assert second.number_matches_on_itself(1212)
  assert second.number_matches_on_itself(123_123)
  assert !second.number_matches_on_itself(4_123_123)
}

pub fn find_invalid_ids_in_range_test() {
  assert second.find_invalid_ids_in_range_part1(#(11, 22)) == [11, 22]
  assert second.find_invalid_ids_in_range_part1(#(95, 115)) == [99]
  assert second.find_invalid_ids_in_range_part1(#(998, 1012)) == [1010]
  assert second.find_invalid_ids_in_range_part1(#(1_188_511_880, 1_188_511_890))
    == [1_188_511_885]

  assert second.find_invalid_ids_in_range_part2(#(11, 22)) == [11, 22]
  assert second.find_invalid_ids_in_range_part2(#(95, 115)) == [99, 111]
  assert second.find_invalid_ids_in_range_part2(#(998, 1012)) == [999, 1010]
  assert second.find_invalid_ids_in_range_part2(#(1_188_511_880, 1_188_511_890))
    == [1_188_511_885]
}

pub fn split_range_test() {
  assert second.split_range(#(30, 31), 100) == [#(30, 31)]
  assert second.split_range(#(30, 130), 100) == [#(30, 130)]
  assert second.split_range(#(30, 131), 100) == [#(30, 130), #(131, 131)]
  assert second.split_range(#(30, 232), 100)
    == [#(30, 130), #(131, 231), #(232, 232)]
}

pub fn sum_of_invalid_indices_test() {
  let ranges = [
    #(11, 22),
    #(95, 115),
    #(998, 1012),
    #(1_188_511_880, 1_188_511_890),
    #(222_220, 222_224),
    #(1_698_522, 1_698_528),
    #(446_443, 446_449),
    #(38_593_856, 38_593_862),
    #(565_653, 565_659),
    #(824_824_821, 824_824_827),
    #(2_121_212_118, 2_121_212_124),
  ]

  // let timer = instant.now()
  assert second.sum_of_invalid_indices(
      ranges,
      second.find_invalid_ids_in_range_part1,
    )
    == 1_227_775_554
  // echo instant.since(timer)

  // let timer = instant.now()
  assert second.sum_of_invalid_indices_parallel(
      ranges,
      second.find_invalid_ids_in_range_part1,
      100,
    )
    == 1_227_775_554
  // echo instant.since(timer)
}
