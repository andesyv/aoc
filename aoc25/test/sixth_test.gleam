import sixth

pub fn parse_test() {
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

  assert sixth.parse(sut) == expected
}

pub fn grand_total_test() {
  let sut = [
    #([123, 45, 6], sixth.Multiplication),
    #([328, 64, 98], sixth.Addition),
    #([51, 387, 215], sixth.Multiplication),
    #([64, 23, 314], sixth.Addition),
  ]

  assert sixth.calc_grand_total(sut) == 4_277_556
}
