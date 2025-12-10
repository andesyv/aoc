import common
import gleam/bool
import gleam/erlang/process
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/result
import nibble
import nibble/lexer
import simplifile
import tempo/duration
import tempo/instant

pub fn main() {
  io.println("Hello day 10!")

  let assert Ok(input) = simplifile.read("./inputs/10.txt")
    as "Failed to read file"
  let input = input |> parse
  let timer = instant.now()
  let sum_of_fewest_buttons_required =
    input
    |> sum_of_min_required_button_presses_to_activate_machines
    |> int.to_string
  let elapsed_ms =
    timer |> instant.since |> duration.as_milliseconds |> int.to_string

  io.println(
    "Sum of fewest button presses required to configure machines (part 1): "
    <> sum_of_fewest_buttons_required,
  )
  io.println("And it only took " <> elapsed_ms <> "ms to figure out...")
}

pub type Machine {
  Machine(
    indicator_lights: List(Bool),
    wiring_schematics: List(List(Int)),
    joltage_requirements: List(Int),
  )
}

type Token {
  Num(Int)
  LSquareBracket
  RSquareBracket
  LParan
  RParan
  LBracket
  RBracket
  Comma
  Dot
  Square
}

pub fn parse(input: String) -> List(Machine) {
  let lexer =
    lexer.simple([
      lexer.int(Num),
      lexer.token("[", LSquareBracket),
      lexer.token("]", RSquareBracket),
      lexer.token("(", LParan),
      lexer.token(")", RParan),
      lexer.token("{", LBracket),
      lexer.token("}", RBracket),
      lexer.token(",", Comma),
      lexer.token(".", Dot),
      lexer.token("#", Square),
      lexer.whitespace(Nil) |> lexer.ignore,
    ])

  let int_parser = {
    use tok <- nibble.take_map("expected number")
    case tok {
      Num(n) -> option.Some(n)
      _ -> option.None
    }
  }

  let token_with_value_parser = fn(token, value) {
    use _ <- nibble.do(nibble.token(token))
    nibble.return(value)
  }

  let indicator_lights_parser = {
    use _ <- nibble.do(nibble.token(LSquareBracket))
    use states <- nibble.do(
      nibble.many(
        nibble.one_of([
          token_with_value_parser(Square, True),
          token_with_value_parser(Dot, False),
        ]),
      ),
    )
    use _ <- nibble.do(nibble.token(RSquareBracket))
    nibble.return(states)
  }

  let number_list_parser = fn(start_token, end_token) {
    use _ <- nibble.do(nibble.token(start_token))
    use numbers <- nibble.do(nibble.sequence(int_parser, nibble.token(Comma)))
    use _ <- nibble.do(nibble.token(end_token))
    nibble.return(numbers)
  }

  let schematic_parser = number_list_parser(LParan, RParan)
  let joltage_parser = number_list_parser(LBracket, RBracket)

  let machine_parser = {
    use indicator_lights <- nibble.do(indicator_lights_parser)
    use wiring_schematics <- nibble.do(nibble.many1(schematic_parser))
    use joltage_requirements <- nibble.do(joltage_parser)
    nibble.return(Machine(
      indicator_lights:,
      wiring_schematics:,
      joltage_requirements:,
    ))
  }

  let parser = {
    use machines <- nibble.do(nibble.many(machine_parser))
    nibble.return(machines)
  }

  let assert Ok(tokens) = lexer.run(input, lexer)
  let assert Ok(machines) = nibble.run(tokens, parser)

  machines
}

pub fn indicator_light_to_binary(light: List(Bool)) -> Int {
  // The left-most of the most-significal bit
  case light {
    [significant_bit, ..rest] ->
      case significant_bit {
        True -> common.integer_power(2, list.length(rest))
        False -> 0
      }
      + indicator_light_to_binary(rest)
    [] -> 0
  }
}

pub fn button_to_binary(light_size: Int, button: List(Int)) -> Int {
  case button {
    [] -> 0
    [index, ..rest] ->
      common.integer_power(2, light_size - index - 1)
      + button_to_binary(light_size, rest)
  }
}

pub fn toggle_light(light: Int, button: Int) -> Int {
  int.bitwise_exclusive_or(light, button)
}

fn button_presses_required_to_enable_machine(
  current: Int,
  buttons: List(Int),
) -> Int {
  case buttons {
    // Big number!
    [] -> 100_000_000_000_000_000_000
    [last] ->
      case toggle_light(current, last) {
        0 -> 1
        _ -> 100_000_000_000_000_000_000
      }
    _ -> {
      // Choose candidates that will toggle the currently enabled lights
      let candidates =
        buttons
        |> list.filter(fn(candidate) { int.bitwise_and(current, candidate) > 0 })

      candidates
      |> list.map(fn(next_candidate) {
        let current = toggle_light(current, next_candidate)
        use <- bool.guard(current == 0, 1)

        1
        + button_presses_required_to_enable_machine(
          current,
          buttons |> list.filter(fn(el) { el != next_candidate }),
        )
      })
      |> list.max(fn(a, b) { int.compare(b, a) })
      |> result.unwrap(100_000_000_000_000_000_000)
    }
  }
}

// fn button_presses_required_to_enable_machine(
//   target: Int,
//   current: Int,
//   buttons: List(Int),
// ) -> Int {
//   case buttons {
//     // Big number!
//     [] -> 100_000_000_000_000_000_000
//     [next_button, ..rest] -> {
//       let current = toggle_light(current, next_button)
//       use <- bool.guard(current == target, 1)
//       1 + button_presses_required_to_enable_machine(target, current, rest)
//     }
//   }
// }

pub fn get_min_button_presses_to_active_machine(machine: Machine) -> Int {
  let target_as_bin = machine.indicator_lights |> indicator_light_to_binary

  let buttons =
    machine.wiring_schematics
    |> list.map(fn(button) {
      button_to_binary(list.length(machine.indicator_lights), button)
    })
  // |> list.permutations
  // |> list.map(fn(combination) {
  //   button_presses_required_to_enable_machine(target_as_bin, 0, combination)
  // })
  // |> list.max(fn(a, b) { int.compare(b, a) })
  // |> result.unwrap(100_000_000_000_000_000_000)

  let required =
    button_presses_required_to_enable_machine(target_as_bin, buttons)
  required
}

pub fn sum_of_min_required_button_presses_to_activate_machines(
  machines: List(Machine),
) -> Int {
  io.println(
    "Spawning " <> list.length(machines) |> int.to_string <> " processes",
  )
  let subject = process.new_subject()
  machines
  |> list.zip(list.range(0, list.length(machines)))
  |> list.map(fn(machine_and_id) {
    let #(machine, id) = machine_and_id
    process.spawn(fn() {
      // let timer = instant.now()
      process.send(subject, get_min_button_presses_to_active_machine(machine))
      // let elapsed_ms =
      //   timer |> instant.since |> duration.as_milliseconds |> int.to_string
      // io.println("Took a process " <> elapsed_ms <> " to finish it's work")
      io.println("Process " <> id |> int.to_string <> " finished")
    })
  })
  |> list.map(fn(_pid) { process.receive_forever(subject) })
  |> list.fold(0, int.add)
  // machines
  // |> list.zip(list.range(0, list.length(machines)))
  // |> list.map(fn(machine_and_id) {
  //   let #(machine, id) = machine_and_id
  //   get_min_button_presses_to_active_machine(machine)
  // })
  // |> list.fold(0, int.add)
}
