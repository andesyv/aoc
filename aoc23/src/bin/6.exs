defmodule AOC do
  def get_distance_permutations(max_time) do
    for n <- 0..(max_time - 1), do: n * max(max_time - n, 0)
  end

  def parse(s) do
    String.split(s, "\n")
    |> Enum.filter(fn line -> 0 < String.length(line) end)
    |> Enum.map(fn line ->
      String.split(line, " ", trim: true)
      |> Enum.drop(1)
      |> Enum.map(fn num ->
        elem(Integer.parse(num), 0)
      end)
    end)
    |> Enum.zip()
  end

  def parse2(s) do
    [
      String.split(s, "\n")
      |> Enum.filter(fn line -> 0 < String.length(line) end)
      |> Enum.map(fn line ->
        num =
          String.split(line, " ", trim: true)
          |> Enum.drop(1)
          |> List.foldr("", fn x, acc -> x <> acc end)

        elem(Integer.parse(num), 0)
      end)
      |> List.to_tuple()
    ]
  end

  def number_of_ways_to_beat_races(input, part_2 \\ false) do
    if part_2 do
      parse2(input)
    else
      parse(input)
    end
    |> Enum.map(fn {max_time, record_distance} ->
      get_distance_permutations(max_time)
      |> Enum.filter(fn race_distance -> record_distance < race_distance end)
      |> Enum.count()
    end)
    |> List.foldl(1, fn x, acc -> x * acc end)
  end
end

# Test
example_input = "Time:      7  15   30
Distance:  9  40  200"

[{7, 9}, {15, 40}, {30, 200}] = AOC.parse(example_input)
# IO.puts("Number of ways to beat race: #{AOC.number_of_ways_to_beat_races(example_input)}")
# IO.puts("Number of ways to beat race: #{AOC.number_of_ways_to_beat_races(example_input, true)}")

# Puzzle solution
input = File.read!("src/inputs/6.txt")
IO.puts("Number of ways to beat race: #{AOC.number_of_ways_to_beat_races(input)}")
IO.puts("Number of ways to beat race: #{AOC.number_of_ways_to_beat_races(input, true)}")
