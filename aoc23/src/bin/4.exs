defmodule AOC do
  def s_2_num(s) do
    case Integer.parse(s) do
      {n, _} -> n
      _ -> :error
    end
  end

  def parse(s) do
    String.split(s, "\n")
    |> Enum.filter(fn line -> 0 < String.length(line) end)
    |> Enum.map(fn line ->
      String.split(line, ": ") |> Enum.at(1)
      # case line do
      #   <<"Card ", game_id::binary-size(1), ": ", rest::binary>> ->
      #     {s_2_num(game_id), rest}

      #   <<"Card ", game_id::binary-size(2), ": ", rest::binary>> ->
      #     {s_2_num(game_id), rest}

      #   _ ->
      #     :error
      # end
    end)
    |> Enum.map(fn card ->
      case String.split(card, " | ")
           |> Enum.map(fn part ->
             String.split(part, " ")
             |> Enum.filter(fn s -> 0 < String.length(s) end)
             |> Enum.map(fn num -> s_2_num(num) end)
           end) do
        [winners, actuals] -> {winners, actuals}
        _ -> :error
      end
    end)
  end

  # Quite slick:
  def get_card_wins({winners, actuals}) do
    winners = MapSet.new(winners)
    length(actuals |> Enum.filter(fn n -> MapSet.member?(winners, n) end))
  end

  def get_card_scores(s) do
    parse(s)
    |> Enum.map(fn card ->
      matching_cards = get_card_wins(card)
      if 0 < matching_cards, do: 2 ** (matching_cards - 1), else: 0
    end)
    |> Enum.sum()
  end

  # def count_increasing_cards(original_stack, depth \\ 0) do
  #   case IO.inspect(original_stack) do
  #     [] ->
  #       0

  #     [{n, i} | rest] ->
  #       # IO.puts("n = #{n}, depth = #{depth}, card number = #{i}")
  #       # copies = rest |> Enum.take(n)
  #       sum_of_copies = for x <- 0..n, do: count_increasing_cards(rest |> Enum.drop(x), depth + 1)
  #       # IO.puts("Partial sum: #{Enum.sum(sum_of_copies)}")
  #       Enum.sum(sum_of_copies) + 1
  #   end
  # end

  def count_increasing_cards(cards, copy_count \\ 0) do
    case cards do
      [] ->
        0

      [n | rest] ->
        IO.puts("n = #{n}, copy count = #{copy_count}")
        count_increasing_cards((rest |> Enum.take(n) |> Enum.map(fn x -> x + 1 end)) ++ (rest |> Enum.drop(n))) + n
    end
  end

  def get_num_of_cards_with_copy_rules(s) do
    parse(s)
    |> Enum.map(&get_card_wins/1)
    # |> Enum.with_index()
    |> count_increasing_cards
  end
end

# Tests

example_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"

[
  {[41, 48, 83, 86, 17], [83, 86, 6, 31, 17, 9, 48, 53]},
  {[13, 32, 20, 16, 61], [61, 30, 68, 82, 17, 32, 24, 19]},
  {[1, 21, 53, 59, 44], [69, 82, 63, 72, 16, 21, 14, 1]},
  {[41, 92, 73, 84, 69], [59, 84, 76, 51, 58, 5, 54, 83]},
  {[87, 83, 26, 28, 32], [88, 30, 70, 12, 93, 22, 82, 36]},
  {[31, 18, 13, 56, 72], [74, 77, 10, 23, 35, 67, 36, 11]}
] = AOC.parse(example_input)

# IO.puts("Card points sum: #{AOC.get_card_scores(example_input)}")

# 123 = AOC.count_increasing_cards([0, 2, 1, 0])
# IO.puts("Number of cards: #{AOC.get_num_of_cards_with_copy_rules(example_input)}")
30 = AOC.get_num_of_cards_with_copy_rules("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")

# Puzzle solution
# input = File.read!("src/inputs/4.txt")
# IO.puts("Card points sum: #{AOC.get_card_scores(input)}")
