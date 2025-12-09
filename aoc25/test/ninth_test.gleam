import ninth

pub fn parse_test() {
  let sut =
    "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"

  assert ninth.parse(sut)
    == [
      #(7, 1),
      #(11, 1),
      #(11, 7),
      #(9, 7),
      #(9, 5),
      #(2, 5),
      #(2, 3),
      #(7, 3),
    ]
}

pub fn largest_square_size_test() {
  assert ninth.get_largest_square_area([
      #(7, 1),
      #(11, 1),
      #(11, 7),
      #(9, 7),
      #(9, 5),
      #(2, 5),
      #(2, 3),
      #(7, 3),
    ])
    == 50
}
