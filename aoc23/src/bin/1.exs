defmodule AOC do
  def get_pattern(args, i) do
    patterns = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

    if args[:is_reverse] do
      String.reverse(Enum.at(patterns, i))
    else
      Enum.at(patterns, i)
    end
  end

  def match_number(s, args \\ [is_reverse: false], i \\ 0) do
    patterns = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

    cond do
      i < length(patterns) and String.starts_with?(s, get_pattern(args, i)) -> i + 1
      i < length(patterns) -> match_number(s, args, i + 1)
      true -> nil
    end
  end

  def parse_first_number(s, args \\ [is_reverse: false, match_number_words: true]) do
    x = match_number(s, args)

    if args[:match_number_words] and x != nil do
      x
    else
      case s do
        <<x, rest::binary>> ->
          cond do
            ?0 <= x and x <= ?9 -> x - ?0
            true -> parse_first_number(rest)
          end

        _ ->
          nil
      end
    end
  end

  def parse_first_and_last_number(s) do
    {parse_first_number(s, is_reverse: false, match_number_words: true),
     parse_first_number(String.reverse(s), is_reverse: true, match_number_words: true)}
  end

  # def parse_first_and_last_number(s) do
  #   {first, i} = parse_first_number(s)
  #   {last, j} = parse_first_number(String.reverse(s))
  #   # Make sure we don't parse the same number twice
  #   if i == String.length(s) - j - 1 do
  #     {first, nil}
  #   else
  #     {first, last}
  #   end
  # end

  def parse_number_from_first_and_last_number_in_string(s) do
    case parse_first_and_last_number(s) do
      {nil, nil} ->
        0

      {first, nil} ->
        first

      {nil, last} ->
        last

      {first, last} ->
        case Integer.parse("#{first}#{last}") do
          {n, _} -> n
          :error -> 0
        end
    end
  end

  def get_sum_of_calibration_values(s) do
    String.split(s, "\n")
    |> Enum.map(&parse_number_from_first_and_last_number_in_string/1)
    |> Enum.sum()
  end
end

# Tests
1 = AOC.parse_first_number("abc123")

12 = AOC.parse_number_from_first_and_last_number_in_string("1abc2")
32 = AOC.parse_number_from_first_and_last_number_in_string("3a3b2")
33 = AOC.parse_number_from_first_and_last_number_in_string("3asdasd")
55 = AOC.parse_number_from_first_and_last_number_in_string("asdasd5")
0 = AOC.parse_number_from_first_and_last_number_in_string("asdasd")
12 = AOC.parse_number_from_first_and_last_number_in_string("as1b2sd")

# Calculation

# example_input = "1abc2
# pqr3stu8vwx
# a1b2c3d4e5f
# treb7uchet"

# example_input_2 = "two1nine
# eightwothree
# abcone2threexyz
# xtwone3four
# 4nineeightseven2
# zoneight234
# 7pqrstsixteen"

# IO.puts("Sum of calibration values: #{AOC.get_sum_of_calibration_values(example_input_2)}")
IO.puts(
  "Sum of calibration values: #{AOC.get_sum_of_calibration_values(File.read!("src/inputs/1.txt"))}"
)
