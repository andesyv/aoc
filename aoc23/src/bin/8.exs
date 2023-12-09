defmodule AOC do
  def parse_nodes(nodes) do
    String.split(nodes, "\n")
    |> Enum.map(fn line ->
      case line do
        <<name::binary-size(3), " = (", left::binary-size(3), ", ", right::binary-size(3),
          _::binary>> ->
          {name, {left, right}}

        _ ->
          :error
      end
    end)
    |> Enum.filter(fn e -> e != :error end)
    |> Map.new()
  end

  def parse(s) do
    [directions | [nodes | []]] = String.split(s, "\n\n", trim: true)
    {String.graphemes(directions), parse_nodes(nodes)}
  end

  def iterate_step(pos, directions, nodes, step \\ 0) do
    if pos == "ZZZ" do
      step
    else
      {left, right} = nodes[pos]

      IO.puts("Position: #{pos}, direction: #{hd(Enum.take(directions, 1))}")

      [dir | rest_dir] = directions

      iterate_step(
        if(dir == "L", do: left, else: right),
        rest_dir,
        nodes,
        step + 1
      )
    end
  end

  def steps_to_reach_goal(s) do
    {directions, nodes} = parse(s)
    iterate_step("AAA", Stream.cycle(directions), nodes)
  end
end

# Tests

example_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"

example_input_2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"

2 = AOC.steps_to_reach_goal(example_input)
6 = AOC.steps_to_reach_goal(example_input_2)

# Puzzle solution
input = File.read!("src/inputs/8.txt")
IO.puts("Steps required: #{AOC.steps_to_reach_goal(input)}")
