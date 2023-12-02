defmodule AOC do
  def parse_cubes(set) do
    String.split(set, ", ")
    |> Enum.map(fn instruction ->
      case Integer.parse(instruction) do
        {count, <<" ", type::binary>>} ->
          case type do
            "blue" -> {:blue, count}
            "red" -> {:red, count}
            "green" -> {:green, count}
            _ -> :error
          end

        _ ->
          :error
      end
    end)
    |> Enum.filter(fn cubes -> cubes != :error end)
  end

  def parse_round(rest_line) do
    for cubes <- String.split(rest_line, "; "),
        do:
          parse_cubes(cubes)
          |> Enum.filter(fn cube_set -> cube_set != [] end)
  end

  def skip_n_ascii_characters(s, n) do
    case s do
      <<_::binary-size(n), rest::binary>> -> rest
      _ -> s
    end
  end

  def parse_game(line) do
    case line do
      <<"Game ", rest::binary>> ->
        case Integer.parse(rest) do
          {id, rest} -> {id, parse_round(skip_n_ascii_characters(rest, 2))}
          :error -> :error
        end

      _ ->
        :error
    end
  end

  def parse_games(s) do
    String.split(s, "\n")
    |> Enum.map(&parse_game/1)
    |> Enum.filter(fn game -> game != :error end)
  end

  def game_is_possible(game, limits) do
    {_, rounds} = game

    rounds
    |> Enum.all?(fn round ->
      Enum.all?(limits, fn limit ->
        {key, limit_value} = limit

        case round[key] do
          nil -> true
          count -> count <= limit_value
        end
      end)
    end)
  end

  def sum_of_possible_games(input, limits) do
    parse_games(input)
    |> Enum.filter(fn game -> game_is_possible(game, limits) end)
    |> List.foldl(0, fn game, acc ->
      {n, _} = game
      n + acc
    end)
  end

  def power_of_game(game) do
    {_, rounds} = game

    scores =
      for key <- [:red, :green, :blue],
          do:
            {key,
             Enum.max(
               Enum.map(rounds, fn round ->
                 case round[key] do
                   nil -> 0
                   n -> n
                 end
               end)
             )}

    scores
    |> List.foldl(1, fn x, acc ->
      {_, count} = x
      count * acc
    end)
  end

  def sum_of_min_cube_powers(input) do
    parse_games(input)
    |> Enum.map(fn game -> power_of_game(game) end)
    |> Enum.sum()
  end
end

# Tests

example_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"

example_limits = [red: 12, green: 13, blue: 14]

[{:blue, 1}, {:green, 2}] = AOC.parse_cubes("1 blue, 2 green")
[{:green, 3}, {:blue, 4}, {:red, 1}] = AOC.parse_cubes("3 green, 4 blue, 1 red")

[[blue: 3, red: 4], [red: 1, green: 2, blue: 6], [green: 2]] =
  AOC.parse_round("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")

{1, [[blue: 3, red: 4], [red: 1, green: 2, blue: 6], [green: 2]]} =
  AOC.parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")

true =
  AOC.game_is_possible(
    {1, [[blue: 3, red: 4], [red: 1, green: 2, blue: 6], [green: 2]]},
    example_limits
  )

true =
  AOC.game_is_possible(
    {2, [[blue: 1, green: 2], [green: 3, blue: 4, red: 1], [green: 1, blue: 1]]},
    example_limits
  )

false =
  AOC.game_is_possible(
    {3, [[green: 8, blue: 6, red: 20], [blue: 5, red: 4, green: 13], [greenn: 5, red: 1]]},
    example_limits
  )

8 = AOC.sum_of_possible_games(example_input, example_limits)

48 = AOC.power_of_game({1, [[blue: 3, red: 4], [red: 1, green: 2, blue: 6], [green: 2]]})

12 =
  AOC.power_of_game({2, [[blue: 1, green: 2], [green: 3, blue: 4, red: 1], [green: 1, blue: 1]]})

630 =
  AOC.power_of_game(
    {4, [[green: 1, red: 3, blue: 6], [green: 3, red: 6], [green: 3, blue: 15, red: 14]]}
  )

{4, [[green: 1, red: 3, blue: 6], [green: 3, red: 6], [green: 3, blue: 15, red: 14]]} =
  AOC.parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")

2286 = AOC.sum_of_min_cube_powers(example_input)

# Puzzle solution
input = File.read!("src/inputs/2.txt")

IO.puts(
  "Sum of possible games: #{AOC.sum_of_possible_games(input, red: 12, green: 13, blue: 14)}"
)

IO.puts("Sum of cube powers: #{AOC.sum_of_min_cube_powers(input)}")
