const std = @import("std");

const Equation = struct {
    result: u32,
    params: std.ArrayList(u32),
};

const ParseResult = struct {
    num: u32,
    rest: []const u8,
};

fn nextIsDigit(input: []const u8) bool {
    if (input.len == 0) {
        return false;
    }

    return switch (input[0]) { '0'...'9' => true, else => false };
}

fn parseNum(input: []const u8) !ParseResult {
    var i: usize = 0;
    while (nextIsDigit(input[i..])) : (i += 1) {}
    return ParseResult{
        .num = try std.fmt.parseInt(u32, input[0..i], 0),
        .rest = input[i..],
    };
}

fn parseNums(line: []const u8, list: *std.ArrayList(u32)) !void {
    if (line.len == 0) {
        return;
    }

    const rest = blk: {
        if (nextIsDigit(line)) {
            const result = try parseNum(line);
            try list.*.append(result.num);
            break :blk result.rest;
        } else {
            break :blk line[1..];
        }
    };

    try parseNums(rest, list);
}

fn parseLine(allocator: std.mem.Allocator, input: [] const u8) !Equation {
    var nums = std.ArrayList(u32).init(allocator);
    try parseNums(input, &nums);
    if (nums.items.len < 3) {
        return error.ParseError;
    }

    return Equation {
        .result = nums.orderedRemove(0),
        .params = nums,
    };
}

fn deinitEquations(equations: std.ArrayList(Equation)) void {
    for (equations.items) |eq| {
        eq.params.deinit();
    }
    equations.deinit();
}

fn parse(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Equation) {
    var equations = std.ArrayList(Equation).init(allocator);

    var line_it = std.mem.tokenizeScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        try equations.append(try parseLine(allocator, line));
    }

    return equations;
}

const Op = enum { Add, Multiply, };

fn eval(allocator: std.mem.Allocator, parameters: )

// meh...

pub fn main() void {
    std.debug.print("Hello AOC!\n", .{});
}

const example_input =
    \\190: 10 19
    \\3267: 81 40 27
    \\83: 17 5
    \\156: 15 6
    \\7290: 6 8 6 15
    \\161011: 16 10 13
    \\192: 17 8 14
    \\21037: 9 7 18 13
    \\292: 11 6 16 20
;

test "parse example" {
    const input =
        \\190: 10 19
        \\3267: 81 40 27
        \\83: 17 5
    ;
    const equations = try parse(std.testing.allocator, input);
    defer deinitEquations(equations);

    try std.testing.expectEqual(190, equations.items[0].result);
    try std.testing.expectEqualSlices(u32, &[_]u32{ 10, 19 }, equations.items[0].params.items);

    try std.testing.expectEqual(3267, equations.items[1].result);
    try std.testing.expectEqualSlices(u32, &[_]u32{ 81, 40, 27 }, equations.items[1].params.items);

    try std.testing.expectEqual(83, equations.items[2].result);
    try std.testing.expectEqualSlices(u32, &[_]u32{ 17, 5 }, equations.items[2].params.items);
}
