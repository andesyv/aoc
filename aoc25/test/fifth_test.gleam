import fifth

pub fn parse_test() {
  let sut =
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32"

  let expected =
    fifth.Inventory(
      fresh_ingredients: [#(3, 5), #(10, 14), #(16, 20), #(12, 18)],
      available_ingredients: [1, 5, 8, 11, 17, 32],
    )

  assert fifth.parse(sut) == expected
}

pub fn count_ingredients_test() {
  let sut =
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32"

  assert fifth.count_fresh_ingredients(fifth.parse(sut)) == 3
}
