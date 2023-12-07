defmodule AOC do
  def get_char_rank(c, joker_rules \\ false) do
    char_rank =
      if joker_rules do
        ["J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"]
      else
        ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"]
      end

    case Enum.find_index(char_rank, fn x -> x == c end) do
      nil -> 0
      i -> i + 1
    end
  end

  def get_hand_type(hand, joker_rules \\ false) do
    jokers =
      if joker_rules,
        do: String.graphemes(hand) |> Enum.filter(fn c -> c == "J" end) |> length,
        else: 0

    card_counts =
      String.graphemes(hand)
      |> List.foldl(%{}, fn x, acc ->
        if joker_rules and x == "J" do
          acc
        else
          Map.put(
            acc,
            x,
            case acc[x] do
              nil -> 0
              count -> count
            end + 1
          )
        end
      end)
      |> Map.values()
      |> Enum.sort()

    # Append the jokers to the last count (the one that would favor the joker the most)
    card_counts =
      Enum.take(card_counts, length(card_counts) - 1) ++
        [
          case List.last(card_counts) do
            nil -> 0
            x -> x
          end + jokers
        ]


    case card_counts do
      # Five of a kind
      [5] -> 6
      # Four of a kind
      [1, 4] -> 5
      # Full house
      [2, 3] -> 4
      # Three of a kind
      [1, 1, 3] -> 3
      # Two pair
      [1, 2, 2] -> 2
      [1, 1, 1, 2] -> 1
      _ -> 0
    end
  end

  def hand_is_less_ordered(hands, joker_rules \\ false) do
    case hands do
      [] ->
        true

      [{l, r} | rest] ->
        l_rank = get_char_rank(l, joker_rules)
        r_rank = get_char_rank(r, joker_rules)

        if l_rank != r_rank do
          l_rank < r_rank
        else
          hand_is_less_ordered(rest, joker_rules)
        end
    end
  end

  def order_cards(lhs, rhs, joker_rules \\ false) do
    lhs_t = get_hand_type(lhs, joker_rules)
    rhs_t = get_hand_type(rhs, joker_rules)

    cond do
      lhs_t > rhs_t ->
        false

      lhs_t < rhs_t ->
        true

      true ->
        hand_is_less_ordered(Enum.zip(String.graphemes(lhs), String.graphemes(rhs)), joker_rules)
    end
  end

  def parse(s) do
    String.split(s, "\n")
    |> Enum.map(fn line ->
      case String.split(line, " ") do
        [hand, bid] -> {hand, bid |> Integer.parse() |> elem(0)}
        _ -> :error
      end
    end)
    |> Enum.filter(fn hand -> hand != :error end)
  end

  def get_total_winnings(s) do
    parse(s)
    |> Enum.sort_by(&elem(&1, 0), &order_cards/2)
    |> Enum.with_index(1)
    |> List.foldl(0, fn {{_, bid}, rank}, acc ->
      # IO.puts("Hand #{hand} has rank #{rank} and a bid of #{bid}")
      bid * rank + acc
    end)
  end

  def get_total_winnings_with_joker(s) do
    parse(s)
    |> Enum.sort_by(&elem(&1, 0), &order_cards(&1, &2, true))
    |> Enum.with_index(1)
    |> List.foldl(0, fn {{_, bid}, rank}, acc ->
      # IO.puts("Hand #{hand} has rank #{rank} and a bid of #{bid}")
      bid * rank + acc
    end)
  end
end

# Tests

# true = AOC.order_cards("32T3K", "KK677")
# true = AOC.order_cards("KTJJT", "KK677")
# false = AOC.order_cards("KK677", "KTJJT")
# true = AOC.order_cards("T55J5", "QQQJA")
# true = AOC.order_cards("KK677", "T55J5")

true = AOC.order_cards("JJQQQ", "2J222", true)

example_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"

# https://www.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/
example_input_2 = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"

# IO.puts("Total winnings: #{AOC.get_total_winnings(example_input)}")
# IO.puts("Total winnings with joker: #{AOC.get_total_winnings_with_joker(example_input)}")
# IO.puts("Total winnings with joker 2: #{AOC.get_total_winnings_with_joker(example_input_2)}")

# Puzzle solution
input = File.read!("src/inputs/7.txt")
IO.puts("Total winnings: #{AOC.get_total_winnings(input)}")
IO.puts("Total winnings with joker: #{AOC.get_total_winnings_with_joker(input)}")
