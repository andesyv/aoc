defmodule AOC do
  def parse_all_numbers(line) do
    case Integer.parse(line) do
      {n, rest} ->
        [n] ++ parse_all_numbers(rest |> String.graphemes() |> Enum.drop(1) |> Enum.join())

      :error ->
        []
    end
  end

  def parse_chunk(chunk) do
    case String.split(chunk, [" ", "\n"], parts: 3) do
      [name | [_ | [rest | []]]] ->
        {name,
         rest
         |> String.split("\n")
         |> Enum.map(fn nums -> parse_all_numbers(nums) |> List.to_tuple() end)}

      _ ->
        :error
    end
  end

  def parse(s) do
    case s do
      <<"seeds: ", rest::binary>> ->
        [seeds | [rest | []]] = rest |> String.split("\n", parts: 2)

        {seeds |> parse_all_numbers,
         rest
         |> String.split("\n\n")
         |> Enum.map(fn chunk -> chunk |> String.trim_leading("\n") |> parse_chunk end)
         |> Enum.filter(fn chunk -> chunk != :error end)
         |> Enum.into(%{})}

      _ ->
        {}
    end
  end

  def map_num_with_range(pos, modifying_ranges) do
    case modifying_ranges do
      [] ->
        pos

      [{dest_range_start, src_range_start, range_length} | rest] ->
        if src_range_start <= pos and pos < src_range_start + range_length do
          dest_range_start + (pos - src_range_start)
        else
          map_num_with_range(pos, rest)
        end
    end
  end

  def map_range_with_range({range_start, range_length}, modifying_ranges) do
    case modifying_ranges do
      [] ->
        range

      [{dest_range_start, src_range_start, map_range_length} | rest] ->
        cond do
          range_start <= src_range_start and src_range_start < range_start + range_length ->

          range_start < src_range_start + map_range_length and src_range_start + map_range_length < range_start + range_length ->
        end
        if src_ra do
          dest_range_start + (pos - src_range_start)
        else
          map_num_with_range(pos, rest)
        end
    end
  end

  def map_ranges_with_range(ranges, modifying_ranges) do

  end

  def get_smallest_seed_positions(s) do
    {seeds, maps} = parse(s)

    seeds
    |> Enum.map(fn seed ->
      seed
      |> map_num_with_range(maps["seed-to-soil"])
      |> map_num_with_range(maps["soil-to-fertilizer"])
      |> map_num_with_range(maps["fertilizer-to-water"])
      |> map_num_with_range(maps["water-to-light"])
      |> map_num_with_range(maps["light-to-temperature"])
      |> map_num_with_range(maps["temperature-to-humidity"])
      |> map_num_with_range(maps["humidity-to-location"])
    end)
    |> Enum.min()
  end

  def get_smallest_position_from_range_of_seeds(s) do
    {seeds, maps} = parse(s)

    # New idea:
    # 1. save the ranges as "ranges"
    # 2. for each range, map the range via it's mapping ranges, potentially splitting original ranges and generating more ranges
    # 3. Repeat process
    # 4. Sort ranges and take the first ranges first element

    seeds
    |> Enum.chunk_every(2)
    |> Enum.flat_map(fn seed_range ->
      # Algorithm gets stuck at this step as we have just too many numbers
      case seed_range do
        [pos | [count | []]] -> for n <- pos..(pos + count), do: n
        _ -> []
      end
    end)
    |> Enum.uniq()
    |> Enum.map(fn seed ->
      seed
      |> map_num_with_range(maps["seed-to-soil"])
      |> map_num_with_range(maps["soil-to-fertilizer"])
      |> map_num_with_range(maps["fertilizer-to-water"])
      |> map_num_with_range(maps["water-to-light"])
      |> map_num_with_range(maps["light-to-temperature"])
      |> map_num_with_range(maps["temperature-to-humidity"])
      |> map_num_with_range(maps["humidity-to-location"])
    end)
    |> Enum.min()
  end
end

# Testing

[79, 14, 55, 13] = AOC.parse_all_numbers("79 14 55 13")

example_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"

# [] = AOC.parse(example_input)

IO.puts("Smallest seed position: #{AOC.get_smallest_position_from_range_of_seeds(example_input)}")

# Puzzle solution
input = File.read!("src/inputs/5.txt")
IO.puts("Smallest seed position: #{AOC.get_smallest_seed_positions(input)}")
IO.puts("Smallest seed position when a range: #{AOC.get_smallest_position_from_range_of_seeds(input)}")
