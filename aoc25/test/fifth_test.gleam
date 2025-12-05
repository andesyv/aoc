import fifth

pub fn parse_test() {
  let sut =
    "3-5
10-14
16-20
12-18
21-24

1
5
8
11
17
32"

  let expected =
    fifth.Inventory(
      // 10-14, 16-20, 12-18 and 21-24 have combined into 10-24
      fresh_ingredients: [#(3, 5), #(10, 24)],
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

pub fn count_unique_fresh_ingredients_test() {
  let sut =
    "3-5
10-14
16-20
12-18

"

  assert fifth.count_unique_fresh_ingredients(fifth.parse(sut)) == 14
}
