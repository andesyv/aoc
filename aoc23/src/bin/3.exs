defmodule AOC do
  def get_indexed_characters(s) do
    String.split(s, "\n")
    |> Enum.with_index()
    |> Enum.flat_map(fn {line, y} ->
      String.graphemes(line)
      |> Enum.with_index()
      |> Enum.map(fn {char, x} -> {char, x, y} end)
    end)
  end

  def str_is_num(str) do
    case str do
      <<c, _::binary>> -> ?0 <= c and c <= ?9
      _ -> false
    end
  end

  # Note: Parses in reverse for linked list optimizations (E.g. "114.." -> [[], [], [4, 1, 1]])
  def accumulate_number_ranges_from_char_list(char_list, acc \\ []) do
    case char_list do
      [] ->
        acc

      [{c, x, y} | rest] ->
        cond do
          str_is_num(c) ->
            case acc do
              [] ->
                accumulate_number_ranges_from_char_list(rest, [[{c, x, y}]])

              [num | acc_rest] ->
                accumulate_number_ranges_from_char_list(rest, [[{c, x, y}] ++ num | acc_rest])
            end

          true ->
            accumulate_number_ranges_from_char_list(rest, [[]] ++ acc)
        end
    end
  end

  def parse_numbers_from_char_list(char_list) do
    accumulate_number_ranges_from_char_list(char_list)
    |> Enum.reverse()
    |> Enum.filter(fn l -> l != [] end)
    |> Enum.map(fn list -> list |> Enum.reverse() end)
  end

  def parse_symbols_from_char_list(char_list) do
    char_list |> Enum.filter(fn {c, _, _} -> c != "." and not str_is_num(c) end)
  end

  def parse(s) do
    characters = get_indexed_characters(s)

    {parse_numbers_from_char_list(characters), parse_symbols_from_char_list(characters)}
  end

  def is_part_number(number, symbols) do
    symbols
    |> Enum.any?(fn {_, symbol_x, symbol_y} ->
      number
      |> Enum.any?(fn {_, x, y} ->
        symbol_x - 1 <= x and x <= symbol_x + 1 and symbol_y - 1 <= y and y <= symbol_y + 1
      end)
    end)
  end

  def number_list_to_number(number_list) do
    number_list
    |> List.foldr("", fn {c, _, _}, acc -> c <> acc end)
    |> Integer.parse()
    |> Kernel.elem(0)
  end

  def sum_of_part_numbers(input) do
    {numbers, symbols} = parse(input)
    # IO.puts("Number count: #{length(numbers)}, symbol count: #{length(symbols)}")
    # IO.puts("All numbers: #{numbers |> Enum.map(fn number_list -> "#{number_list_to_number(number_list)}" end) |> Enum.join(", ")}")

    numbers
    |> Enum.filter(fn number_list -> is_part_number(number_list, symbols) end)
    |> Enum.map(&number_list_to_number/1)
    |> Enum.sum()
  end

  def sum_of_gear_ratios(input) do
    {numbers, symbols} = parse(input)

    symbols
    |> Enum.filter(fn {c, _, _} -> c == "*" end)
    |> Enum.map(fn gear_symbol ->
      adjacent_numbers =
        numbers |> Enum.filter(fn number -> is_part_number(number, [gear_symbol]) end)

      if length(adjacent_numbers) == 2 do
        adjacent_numbers
        |> Enum.map(&number_list_to_number/1)
        |> List.foldr(1, fn x, acc -> x * acc end)
      else
        0
      end
    end)
    |> Enum.sum()
  end
end

# Tests

example_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."

[{"a", 0, 0}, {"b", 1, 0}, {"c", 2, 0}, {"1", 0, 1}, {"2", 1, 1}, {"3", 2, 1}] =
  AOC.get_indexed_characters("abc\n123")

expected_first_line = [
  {"4", 0, 0},
  {"6", 1, 0},
  {"7", 2, 0},
  {".", 3, 0},
  {".", 4, 0},
  {"1", 5, 0},
  {"1", 6, 0},
  {"4", 7, 0},
  {".", 8, 0},
  {".", 9, 0}
]

^expected_first_line = AOC.get_indexed_characters("467..114..")

[[{"4", 0, 0}, {"6", 1, 0}, {"7", 2, 0}], [{"1", 5, 0}, {"1", 6, 0}, {"4", 7, 0}]] =
  AOC.parse_numbers_from_char_list(expected_first_line)

4361 = AOC.sum_of_part_numbers(example_input)
467_835 = AOC.sum_of_gear_ratios(example_input)

# Puzle solution
input = File.read!("src/inputs/3.txt")

IO.puts("Sum of part numbers: #{AOC.sum_of_part_numbers(input)}")
IO.puts("Sum of gear ratios: #{AOC.sum_of_gear_ratios(input)}")
