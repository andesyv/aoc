import sixth

pub fn parse_part1_test() {
  let sut =
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "

  let expected = [
    #([123, 45, 6], sixth.Multiplication),
    #([328, 64, 98], sixth.Addition),
    #([51, 387, 215], sixth.Multiplication),
    #([64, 23, 314], sixth.Addition),
  ]

  assert sixth.parse_part1(sut) == expected
}

pub fn parse_part2_test() {
  let sut =
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "

  let expected = [
    #([4, 431, 623], sixth.Addition),
    #([175, 581, 32], sixth.Multiplication),
    #([8, 248, 369], sixth.Addition),
    #([356, 24, 1], sixth.Multiplication),
  ]

  assert sixth.parse_part2(sut) == expected
}

pub fn grand_total_test() {
  let sut =
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "

  assert sixth.calc_grand_total(sixth.parse_part1(sut)) == 4_277_556
  assert sixth.calc_grand_total(sixth.parse_part2(sut)) == 3_263_827
}
