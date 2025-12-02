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

pub fn valid_id_test() {
  assert !second.is_invalid_id(0)
  assert !second.is_invalid_id(1)
  assert !second.is_invalid_id(0101)

  assert !second.is_invalid_id(10)
  assert second.is_invalid_id(11)
  assert second.is_invalid_id(22)
  assert !second.is_invalid_id(23)

  assert !second.is_invalid_id(95)
  assert second.is_invalid_id(99)

  assert !second.is_invalid_id(222_224)
  assert second.is_invalid_id(222_222)
}

pub fn find_invalid_ids_in_range_test() {
  assert second.find_invalid_ids_in_range(11, 22) == [11, 22]
  assert second.find_invalid_ids_in_range(95, 115) == [99]
  assert second.find_invalid_ids_in_range(998, 1012) == [1010]
  assert second.find_invalid_ids_in_range(1_188_511_880, 1_188_511_890)
    == [1_188_511_885]
}

pub fn split_range_test() {
  assert second.split_range(#(30, 31)) == [#(30, 31)]
  assert second.split_range(#(30, 130)) == [#(30, 130)]
  assert second.split_range(#(30, 131)) == [#(30, 130), #(131, 131)]
  assert second.split_range(#(30, 232))
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
  assert second.sum_of_invalid_indices(ranges) == 1_227_775_554
  // echo instant.since(timer)

  // let timer = instant.now()
  assert second.sum_of_invalid_indices_parallel(ranges) == 1_227_775_554
  // echo instant.since(timer)
}
