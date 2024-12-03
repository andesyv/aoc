const std = @import("std");

const MultStatement = struct { lhs: u32, rhs: u32 };

fn isNum(char: u8) bool {
    return switch (char) {
        '0'...'9' => true,
        else => false,
    };
}

fn parseNumLen(slice: []const u8) ?usize {
    // Read 1-3 numbers:
    var i: usize = 0;
    while (i < slice.len and i < 3 and isNum(slice[i])) {
        i += 1;
    }

    if (i == 0 or i > 3) {
        return null;
    }

    return i;
}

fn peek(slice: []const u8, char: u8) bool {
    return slice.len != 0 and slice[0] == char;
}

fn parseMult(slice: []const u8) ?MultStatement {
    // std.debug.print("Current slice: {s}\n ", .{ slice[0..] });
    const prefix = "mul(";
    if (slice.len < prefix.len or !std.mem.eql(u8, slice[0..prefix.len], prefix)) {
        return null;
    }

    var curr_dist = prefix.len;

    // Parse a number
    const maybe_lhs_len = parseNumLen(slice[curr_dist..]);
    if (maybe_lhs_len == null) {
        return null;
    }

    const lhs = std.fmt.parseInt(u32, slice[curr_dist..(curr_dist + maybe_lhs_len.?)], 0) catch { unreachable; };

    curr_dist += maybe_lhs_len.?;

    // Read a singular comma:
    if (!peek(slice[curr_dist..], ',')) {
        return null;
    }

    curr_dist += 1;

    // Parse another number
    const maybe_rhs_len = parseNumLen(slice[curr_dist..]);
    if (maybe_rhs_len == null) {
        return null;
    }

    const rhs = std.fmt.parseInt(u32, slice[curr_dist..(curr_dist + maybe_rhs_len.?)], 0) catch { unreachable; };

    curr_dist += maybe_rhs_len.?;

    // Finally parse a singular right paranthesis:
    if (!peek(slice[curr_dist..], ')')) {
        return null;
    }

    return MultStatement {
        .lhs = lhs,
        .rhs = rhs,
    };
}

fn parseDo(slice: []const u8) ?bool {
    const prefix = "do()";
    if (slice.len < prefix.len) {
        return null;
    }

    if (!std.mem.eql(u8, slice[0..prefix.len], prefix)) {
        return null;
    }

    return true;
}

fn parseDont(slice: []const u8) ?bool {
    const prefix = "don't()";
    if (slice.len < prefix.len) {
        return null;
    }

    if (!std.mem.eql(u8, slice[0..prefix.len], prefix)) {
        return null;
    }

    return false;
}

fn parseDosAndDont(slice: []const u8) ?bool {
    return parseDo(slice) orelse parseDont(slice);
}

fn parseMults(allocator: std.mem.Allocator, input: []const u8, parse_dos_and_donts: bool) !std.ArrayList(MultStatement) {
    // This would've been super easy with regular expressions. However, Zig does not have em.
    // So this will have to be a good practice in manual parsing...
    var list = std.ArrayList(MultStatement).init(allocator);
    var mults_enabled = true;
    for (0..input.len) |i| {
        // We could optimize by skipping a few tokens whenever we parse something, but Zig is blazingly
        // fast as is so there probably would be no point.
        if (parseMult(input[i..])) |mult| {
            if (mults_enabled) {
                try list.append(mult);
            }
        }

        if (parse_dos_and_donts) {
            if (parseDosAndDont(input[i..])) |enable| {
                mults_enabled = enable;
            }
        }
    }

    return list;
}

fn sumOfMultsFromInput(allocator: std.mem.Allocator, input: []const u8, parse_dos_and_donts: bool) !u32 {
    // Zig also doesn't have a fold yet :/
    var sum: u32 = 0;
    const mults = try parseMults(allocator, input, parse_dos_and_donts);
    defer mults.deinit();

    for (mults.items) |mult| {
        sum += mult.lhs * mult.rhs;
    }

    return sum;
}

pub fn main() void {
    const res = @import("inputs");
    const allocator = std.heap.page_allocator;

    var sum_of_mults = sumOfMultsFromInput(allocator, res.input_3, false) catch |err| {
        std.debug.panic("Calculating sum of mults failed with this error: {any}\n", .{err});
    };

    const stdout = std.io.getStdOut();
    std.fmt.format(stdout.writer(), "Sum of multiplications: {d}\n", .{sum_of_mults}) catch |err| {
        std.debug.panic("Writing to stdout failed with the following error: {any}\n", .{err});
    };

    sum_of_mults = sumOfMultsFromInput(allocator, res.input_3, true) catch |err| {
        std.debug.panic("Calculating sum of mults failed with this error: {any}\n", .{err});
    };

    std.fmt.format(stdout.writer(), "Sum of multiplications with do() and don't() instructions: {d}\n", .{sum_of_mults}) catch |err| {
        std.debug.panic("Writing to stdout failed with the following error: {any}\n", .{err});
    };
}

const example_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

test "isNums" {
    const expect = std.testing.expect;

    for ("0123456789") |c| {
        try expect(isNum(c));
    }
}

test "multi digit parsing" {
    if (parseMult("mul(76,46)")) |mult| {
        try std.testing.expectEqual(MultStatement{ .lhs = 76, .rhs = 46 }, mult);
    } else {
        return error.ParsingFailed;
    }

    if (parseMult("mul(764,406)")) |mult| {
        try std.testing.expectEqual(MultStatement{ .lhs = 764, .rhs = 406 }, mult);
    } else {
        return error.ParsingFailed;
    }

    try std.testing.expect(parseMult("mul(7642,4046)") == null);

    if (parseMult("mul(7,406)")) |mult| {
        try std.testing.expectEqual(MultStatement{ .lhs = 7, .rhs = 406 }, mult);
    } else {
        return error.ParsingFailed;
    }
}

test "parse mults" {
    const test_allocator = std.testing.allocator;
    const sut = try parseMults(test_allocator, example_input, false);
    defer sut.deinit();

    const expected = [_]MultStatement{
        MultStatement{ .lhs = 2, .rhs = 4 },
        MultStatement{ .lhs = 5, .rhs = 5 },
        MultStatement{ .lhs = 11, .rhs = 8 },
        MultStatement{ .lhs = 8, .rhs = 5 }
    };
    try std.testing.expectEqual(expected.len, sut.items.len);
    for (expected, sut.items) |e, a| {
        try std.testing.expectEqual(e.lhs, a.lhs);
        try std.testing.expectEqual(e.rhs, a.rhs);
    }
}

test "sum of mults" {
    const test_allocator = std.testing.allocator;
    const sut = try sumOfMultsFromInput(test_allocator, example_input, false);

    try std.testing.expectEqual(161, sut);
}

const example_input_2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

test "sum of mults with dos and donts" {
    const test_allocator = std.testing.allocator;
    const sut = try sumOfMultsFromInput(test_allocator, example_input_2, true);

    try std.testing.expectEqual(48, sut);
}
