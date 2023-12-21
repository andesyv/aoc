defmodule AOC do
  def parse_to_rows(input) do
    input
    |> String.split("\n")
    |> Enum.filter(fn line -> 0 < String.length(line) end)
    |> Enum.map(&String.graphemes/1)
    |> Enum.zip()
    |> Enum.map(&Tuple.to_list/1)
  end

  def simulate_row(row) do
    case row do
      [a | [b | rest]] ->
        if b == "O" and a == "." do
          [b | simulate_row([a | rest])]
        else
          [a | simulate_row([b | rest])]
        end

      _ ->
        row
    end
  end

  # Very lazy way to do it :)
  def simulate_row_to_completion(row) do
    newRow = simulate_row(row)

    if newRow == row do
      row
    else
      simulate_row_to_completion(newRow)
    end
  end

  def get_simulated_rock_weight_sum(input) do
    rows = parse_to_rows(input)

    rows
    |> Enum.flat_map(fn row ->
      row |> simulate_row_to_completion |> Enum.reverse() |> Enum.with_index()
    end)
    |> Enum.filter(fn {c, _} -> c == "O" end)
    |> List.foldr(0, fn {_, i}, acc -> acc + i + 1 end)
  end
end

example_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."

[
  "OO.O.O..##",
  "...OO....O",
  ".O...#O..O",
  ".O.#......",
  ".#.O......",
  "#.#..O#.##",
  "..#...O.#.",
  "....O#.O#.",
  "....#.....",
  ".#.O.#O..."
] = AOC.parse_to_rows(example_input) |> Enum.map(fn line -> Enum.join(line, "") end)

# [] =
#   AOC.parse_to_rows(example_input)
#   |> Enum.map(fn row -> row |> AOC.simulate_row_to_completion() |> Enum.reverse() |> Enum.with_index() end)

# IO.puts("Rock weight after simulating: #{AOC.get_simulated_rock_weight_sum(example_input)}")
input = File.read!("src/inputs/14.txt")
IO.puts("Rock weight after simulating: #{AOC.get_simulated_rock_weight_sum(input)}")
