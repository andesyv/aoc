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
    register_a: u32,
    register_b: u32,
    register_c: u32,
    instructions: std.ArrayList(u3),

    fn deinit(self: *@This()) void {
        self.instructions.deinit();
    }

    fn readOperand(self: @This(), value: u3, operand_type: Operand) u32 {
        return switch (operand_type) {
            Operand.Literal => @as(u32, @intCast(value)),
            Operand.Combo => switch (value) {
                0...3 => @as(u32, @intCast(value)),
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                7 => unreachable,
            }
        };
    }

    fn evaluate(self: *@This(), allocator: std.mem.Allocator, instructions: []const u3) !std.ArrayList(u32) {
        var outputs = std.ArrayList(u32).init(allocator);

        var ip: usize = 0;
        while (ip < instructions.len) {
            const opcode = @as(Opcode, @enumFromInt(instructions[ip]));
            if (opcode != Opcode.jnz and ip + 1 >= instructions.len) {
                std.debug.panic("Instruction pointer reached and invalid state", .{});
            }

            switch (opcode) {
                Opcode.adv, Opcode.bdv, Opcode.cdv => {
                    const denomerator = std.math.pow(u32, 2, self.readOperand(instructions[ip + 1], Operand.Combo));
                    const result = try std.math.divTrunc(u32, self.*.register_a, denomerator);
                    (if (opcode == Opcode.adv) self.*.register_a else if (opcode == Opcode.bdv) self.*.register_b else self.*.register_c) = result;
                },
                Opcode.bxl => {
                    self.*.register_b = self.*.register_b ^ self.readOperand(instructions[ip + 1], Operand.Literal);
                },
                Opcode.bst => {
                    self.*.register_b = try std.math.mod(u32, self.readOperand(instructions[ip + 1], Operand.Combo), 8);
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
                    const value = try std.math.mod(u32, self.readOperand(instructions[ip + 1], Operand.Combo), 8);
                    try outputs.append(value);
                },
            }

            ip += 2;
        }

        return outputs;
    }

    fn execute(self: *@This(), allocator: std.mem.Allocator) !void {
        const io_out = std.io.getStdOut();
        var outputs = try self.evaluate(allocator, self.*.instructions.items);
        defer outputs.deinit();

        var formatted_list = format_number_list(allocator, outputs.items);
        defer formatted_list.deinit();

        try std.fmt.format(io_out, "Program outputs: {s}\n", .{ formatted_list });
    }
};

fn format_number_list(allocator: std.mem.Allocator, numbers: []const u32) !std.ArrayList(u8) {
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
    var line_it = std.mem.split(u8, input, "\n");

    var line = try (line_it.next() orelse error.ParseError);
    const register_a = try std.fmt.parseInt(u32, std.mem.trimLeft(u8, line, "Register A: "), 0);

    line = try (line_it.next() orelse error.ParseError);
    const register_b = try std.fmt.parseInt(u32, std.mem.trimLeft(u8, line, "Register B: "), 0);

    line = try (line_it.next() orelse error.ParseError);
    const register_c = try std.fmt.parseInt(u32, std.mem.trimLeft(u8, line, "Register C: "), 0);

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
}

const example_input =
    \\Register A: 729
    \\Register B: 0
    \\Register C: 0
    \\
    \\Program: 0,1,5,4,3,0
; // expected output: 4,6,3,5,6,3,5,2,1,0

test "evaluate example input" {
    var sut = try parse(std.testing.allocator, example_input);
    defer sut.deinit();

    const results = try sut.evaluate(std.testing.allocator, sut.instructions.items);
    defer results.deinit();

    try std.testing.expectEqualSlices(u32, &[_]u32{ 4,6,3,5,6,3,5,2,1,0 }, results.items);
}
