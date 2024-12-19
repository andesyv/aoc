const std = @import("std");

const Operand = enum { Literal, Combo };

const Opcode = enum(u3) {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
};

const VM = struct {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instructions: std.ArrayList(u3),

    fn deinit(self: *@This()) void {
        self.instructions.deinit();
    }

    fn readOperand(self: @This(), value: u3, operand_type: Operand) u64 {
        return switch (operand_type) {
            Operand.Literal => @as(u64, @intCast(value)),
            Operand.Combo => switch (value) {
                0...3 => @as(u64, @intCast(value)),
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                7 => unreachable,
            }
        };
    }

    fn evaluate(self: *@This(), outputs: *std.ArrayList(u3)) !void {
        var ip: usize = 0;
        const instructions = self.instructions.items;

        while (ip < instructions.len) {
            const opcode = @as(Opcode, @enumFromInt(instructions[ip]));
            if (opcode != Opcode.jnz and ip + 1 >= instructions.len) {
                std.debug.panic("Instruction pointer reached and invalid state", .{});
            }

            switch (opcode) {
                Opcode.adv, Opcode.bdv, Opcode.cdv => {
                    const denomerator = std.math.pow(u64, 2, self.readOperand(instructions[ip + 1], Operand.Combo));
                    const result = try std.math.divTrunc(u64, self.*.register_a, denomerator);
                    (if (opcode == Opcode.adv) self.*.register_a else if (opcode == Opcode.bdv) self.*.register_b else self.*.register_c) = result;
                },
                Opcode.bxl => {
                    self.*.register_b = self.*.register_b ^ self.readOperand(instructions[ip + 1], Operand.Literal);
                },
                Opcode.bst => {
                    self.*.register_b = try std.math.mod(u64, self.readOperand(instructions[ip + 1], Operand.Combo), 8);
                },
                Opcode.jnz => {
                    if (self.*.register_a != 0) {
                        ip = self.readOperand(instructions[ip + 1], Operand.Literal);
                        continue;
                    }
                },
                Opcode.bxc => {
                    self.*.register_b = self.*.register_b ^ self.*.register_c;
                },
                Opcode.out => {
                    const value = try std.math.mod(u64, self.readOperand(instructions[ip + 1], Operand.Combo), 8);
                    try outputs.*.append(@as(u3, @intCast(value)));
                },
            }

            ip += 2;
        }
    }

    fn evaluate_for_target_output(self: *@This(), working_outputs: *std.ArrayList(u3), target_output: []const u3) !bool {
        working_outputs.clearRetainingCapacity();
        var ip: usize = 0;
        const instructions = self.instructions.items;

        while (ip < instructions.len) {
            const opcode = @as(Opcode, @enumFromInt(instructions[ip]));
            if (opcode != Opcode.jnz and ip + 1 >= instructions.len) {
                std.debug.panic("Instruction pointer reached and invalid state", .{});
            }

            switch (opcode) {
                Opcode.adv, Opcode.bdv, Opcode.cdv => {
                    const denomerator = std.math.pow(u64, 2, self.readOperand(instructions[ip + 1], Operand.Combo));
                    const result = try std.math.divTrunc(u64, self.*.register_a, denomerator);
                    (if (opcode == Opcode.adv) self.*.register_a else if (opcode == Opcode.bdv) self.*.register_b else self.*.register_c) = result;
                },
                Opcode.bxl => {
                    self.*.register_b = self.*.register_b ^ self.readOperand(instructions[ip + 1], Operand.Literal);
                },
                Opcode.bst => {
                    self.*.register_b = try std.math.mod(u64, self.readOperand(instructions[ip + 1], Operand.Combo), 8);
                },
                Opcode.jnz => {
                    if (self.*.register_a != 0) {
                        ip = self.readOperand(instructions[ip + 1], Operand.Literal);
                        continue;
                    }
                },
                Opcode.bxc => {
                    self.*.register_b = self.*.register_b ^ self.*.register_c;
                },
                Opcode.out => {
                    const value = try std.math.mod(u64, self.readOperand(instructions[ip + 1], Operand.Combo), 8);
                    // The only difference between this one and the other one, is that this one short-circuits at the
                    // earliest convenience
                    if (target_output[working_outputs.items.len] != value) {
                        return false;
                    }

                    try working_outputs.*.append(@as(u3, @intCast(value)));
                },
            }

            ip += 2;
        }

        return std.mem.eql(u3, working_outputs.items, target_output);
    }

    fn execute(self: *@This(), allocator: std.mem.Allocator) !void {
        const io_out = std.io.getStdOut().writer();
        var outputs = std.ArrayList(u3).init(allocator);
        try self.evaluate(&outputs);
        defer outputs.deinit();

        var formatted_list = try format_number_list(allocator, outputs.items);
        defer formatted_list.deinit();

        try std.fmt.format(io_out, "Program outputs: {s}\n", .{ formatted_list.items });
    }

    fn find_lowest_quine_registry_value(self: *@This(), allocator: std.mem.Allocator) !u64 {
        // Already checked range: 0 - 128170000000
        var i: u64 = find_quine_start_search_index(self.instructions.items) orelse 0;
        var outputs = std.ArrayList(u3).init(allocator);
        defer outputs.deinit();

        while (true) : (i += 1) {
            self.*.register_a = i;
            self.*.register_b = 0;
            self.*.register_c = 0;
            if (try self.evaluate_for_target_output(&outputs, self.instructions.items)) {
                return i;
            }
            if (i % 10000000 == 0) {
                std.debug.print("i = {}\n", .{i});
            }

            // if (std.mem.eql(u3, outputs.items, self.instructions.items)) {
            //     return i;
            // }
        }

        return i;
    }
};

// To be able to print the program itself (a quine), there are certain criteria our registry value needs to
// fulfill:
//  - There needs to be at least one print statement
//  - We need at least one jump statement, and the jump statement needs to execute an amount of times required to execute the print statement(s).
//  The amount of times we need to execute our jump statement is n = <program token count> / <print statement count>
//  Another thing to make note of, is that the jump statement is only affected by the A registry. And the A registry is only
//  modified by the "adv" statement.
fn find_quine_start_search_index(instructions: []const u3) ?u64 {
    // First, count prints and jump statements:
    var jump_statements: usize = 0;
    var print_statements: usize = 0;
    var ip: usize = 0;
    while (ip + 1 < instructions.len) : (ip += 2) {
        const opcode = @as(Opcode, @enumFromInt(instructions[ip]));
        switch (opcode) {
            Opcode.out => print_statements += 1,
            Opcode.jnz => jump_statements += 1,
            else => {},
        }
    }

    if (print_statements == 0 or jump_statements == 0) {
        std.debug.print("There no possible registry value that can make the program behave as a quine", .{});
        return null;
    }

    // Then figure out the required jump statement executions:
    if (print_statements > 1 and (instructions.len % print_statements) != 0) {
        std.debug.panic("The program has no possible registry values that can make it into a quine.", .{});
        return null;
    }

    if (jump_statements > 1) {
        std.debug.panic("This problem of multiple jumps is not handled by this logic :/", .{});
    }

    const n = (instructions.len / print_statements) - 1;

    // Now determine how many divisions are happening each "jump"
    ip = 0;
    var divisor: u64 = 1;
    while (ip + 1 < instructions.len) : (ip += 2) {
        const opcode = @as(Opcode, @enumFromInt(instructions[ip]));
        switch (opcode) {
            Opcode.jnz => break,
            Opcode.adv => {
                const operand = instructions[ip+1];
                if (operand > 3) {
                    std.debug.panic("This logic only works if the divisor is non changing each jump", .{});
                    return null;
                }
                divisor *= std.math.pow(u64, 2, @intCast(operand));
            },
            else => {},
        }
    }

    // Finally, find a start index that will be great enough to ensure that all required jumps will occour
    return std.math.pow(u64, divisor, n);
}

fn format_number_list(allocator: std.mem.Allocator, numbers: []const u3) !std.ArrayList(u8) {
    var formatted_list = std.ArrayList(u8).init(allocator);
    for (numbers, 0..) |number, i| {
        try std.fmt.format(formatted_list.writer(), "{d}", .{number});
        if (i + 1 != numbers.len) {
            _ = try std.fmt.format(formatted_list.writer(), ",", .{});
        }
    }
    return formatted_list;
}

fn parse(allocator: std.mem.Allocator, input: []const u8) !VM {
    var line_it = std.mem.splitScalar(u8, input, '\n');

    var line = try (line_it.next() orelse error.ParseError);
    const register_a = try std.fmt.parseInt(u64, std.mem.trimLeft(u8, line, "Register A: "), 0);

    line = try (line_it.next() orelse error.ParseError);
    const register_b = try std.fmt.parseInt(u64, std.mem.trimLeft(u8, line, "Register B: "), 0);

    line = try (line_it.next() orelse error.ParseError);
    const register_c = try std.fmt.parseInt(u64, std.mem.trimLeft(u8, line, "Register C: "), 0);

    _ = line_it.next();
    line = try (line_it.next() orelse error.ParseError);
    line = std.mem.trim(u8, std.mem.trimLeft(u8, line, "Program: "), "\n ");

    var instructions = std.ArrayList(u3).init(allocator);
    var i: usize = 0;
    while (i < line.len) : (i += 2) {
        try instructions.append(try std.fmt.parseInt(u3, line[i..i+1], 0));
    }

    return VM{
        .register_a = register_a,
        .register_b = register_b,
        .register_c = register_c,
        .instructions = instructions,
    };
}

pub fn main() void {
    const input = @import("inputs").input_17;
    const allocator = std.heap.page_allocator;

    var vm = parse(allocator, input) catch |err| {
        std.debug.panic("Parsing failed with the following error: {any}\n", .{err});
    };
    defer vm.deinit();

    vm.execute(allocator) catch |err| {
        std.debug.panic("VM execution failed with the following error: {any}\n", .{err});
    };

    const lowest_quine_register_value = vm.find_lowest_quine_registry_value(allocator) catch |err| {
        std.debug.panic("Failed to calculate lowest quine registry value: {any}\n", .{err});
    };

    const io_out = std.io.getStdOut().writer();
    std.fmt.format(io_out, "\nLowest registry value for A that makes the program behave like a quine: {d}\n", .{ lowest_quine_register_value }) catch |err| {
        std.debug.panic("Failed to print to stdout: {any}\n", .{err});
    };
}

const example_input_1 =
    \\Register A: 729
    \\Register B: 0
    \\Register C: 0
    \\
    \\Program: 0,1,5,4,3,0
; // expected output: 4,6,3,5,6,3,5,2,1,0

const example_input_2 =
    \\Register A: 2024
    \\Register B: 0
    \\Register C: 0
    \\
    \\Program: 0,3,5,4,3,0
;

test "evaluate example input" {
    var sut = try parse(std.testing.allocator, example_input_1);
    defer sut.deinit();

    var results = std.ArrayList(u3).init(std.testing.allocator);
    try sut.evaluate(&results);
    defer results.deinit();

    try std.testing.expectEqualSlices(u3, &[_]u3{ 4,6,3,5,6,3,5,2,1,0 }, results.items);
}

test "Find lowest quine from example" {
    var sut = try parse(std.testing.allocator, example_input_2);
    defer sut.deinit();

    const lowest_registry = try sut.find_lowest_quine_registry_value(std.testing.allocator);
    try std.testing.expectEqual(117440, lowest_registry);
}
