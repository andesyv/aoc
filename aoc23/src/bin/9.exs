defmodule AOC do
  def get_extrapolated_diffs(measures) do
    last = List.last(measures)

    if last |> Enum.all?(fn val -> val == 0 end) do
      measures
    else
      get_extrapolated_diffs(
        measures ++
          [
            last
            |> Enum.chunk_every(2, 1, :discard)
            |> Enum.map(fn [first | [second | []]] -> second - first end)
          ]
      )
    end
  end

  # def get_extrapolated_value(rev_extrapolated_measures, forwards \\ true) do
  #   if forwards do
  #     for measures <- rev_extrapolated_measures, do: List.last(measures)
  #   else

  #   end
  #   List.last(hd(rev_extrapolated_measures)) +
  #     if 1 < length(rev_extrapolated_measures) do
  #       get_extrapolated_value(tl(rev_extrapolated_measures))
  #     else
  #       0
  #     end
  # end

  def calc_extrapolated_value(measures, forwards \\ true) do
    extrapolated_measures = Enum.reverse(get_extrapolated_diffs([measures]))

    generated_diffs =
      for(
        measures <- extrapolated_measures,
        do: if(forwards, do: List.last(measures), else: List.first(measures))
      )

    if forwards do
      generated_diffs |> Enum.sum()
    else
      generated_diffs |> List.foldl(0, fn x, acc -> x - acc end)
    end
  end

  def parse(s) do
    String.split(s, "\n")
    |> Enum.filter(fn line -> 0 < String.length(line) end)
    |> Enum.map(fn line ->
      String.split(line, " ")
      |> Enum.map(&elem(Integer.parse(&1), 0))
    end)
  end

  def sum_of_extrapolated_values(input, forwards \\ true) do
    parse(input)
    |> Enum.map(&(calc_extrapolated_value(&1, forwards)))
    |> Enum.sum()
  end
end

# Tests
example_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"

[[0, 3, 6, 9, 12, 15], [3, 3, 3, 3, 3], [0, 0, 0, 0]] =
  AOC.get_extrapolated_diffs([hd(AOC.parse(example_input))])

# IO.puts("Sum of extrapolated forward values: #{AOC.sum_of_extrapolated_values(example_input)}")
# IO.puts("Sum of extrapolated backward values: #{AOC.sum_of_extrapolated_values(example_input, false)}")

# Puzzle solution
input = File.read!("src/inputs/9.txt")
IO.puts("Sum of extrapolated forwards values: #{AOC.sum_of_extrapolated_values(input)}")
IO.puts("Sum of extrapolated backwards values: #{AOC.sum_of_extrapolated_values(input, false)}")
