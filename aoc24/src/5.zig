const std = @import("std");

const Pair = struct{ lhs: u32, rhs: u32 };

const Parsed = struct {
    ordering_rules: std.ArrayList(Pair),
    updates: std.ArrayList(std.ArrayList(u32)),

    fn deinit(self: @This()) void {
        for (self.updates.items) |update| {
            update.deinit();
        }
        self.updates.deinit();
        self.ordering_rules.deinit();
    }
};

fn parseSingleChar(slice: []const u8, char: u8) bool {
    return slice.len > 0 and slice[0] == char;
}

fn parseOrderingRule(line: []const u8) ?Pair {
    if (line.len < 5) {
        return null;
    }
    // All numbers in the task input are 2 digits, which we'll make us of here
    const lhs = std.fmt.parseInt(u32, line[0..2], 0) catch { return null; };
    if (line[2] != '|') {
        return null;
    }
    const rhs = std.fmt.parseInt(u32, line[3..5], 0) catch { return null; };
    return Pair{ .lhs = lhs, .rhs = rhs };
}

fn parseUpdate(allocator: std.mem.Allocator, line: []const u8) ?std.ArrayList(u32) {
    var list = std.ArrayList(u32).init(allocator);

    var i: usize = 0;
    while (1 < line.len) : (i += 3) {
        const num = std.fmt.parseInt(u32, line[(0 + i)..(2 + i)], 0) catch { return null; };
        list.append(num) catch { return null; };
        if (!parseSingleChar(line[(2 + i)..], ',')) {
            break;
        }
    }

    return list;
}

fn parse(allocator: std.mem.Allocator, input: []const u8) !Parsed {
    var line_it = std.mem.tokenizeScalar(u8, input, '\n');
    var ordering_rules = std.ArrayList(Pair).init(allocator);
    var updates = std.ArrayList(std.ArrayList(u32)).init(allocator);

    while (line_it.next()) |line| {
        if (parseOrderingRule(line)) |rule| {
            try ordering_rules.append(rule);
        } else if (parseUpdate(allocator, line)) |update| {
            try updates.append(update);
        }
    }

    return Parsed{
        .ordering_rules = ordering_rules,
        .updates = updates
    };
}

fn ruleAdheredForSinglePage(rest: []const u32, rule: Pair) bool {
    if (rest.len == 0) {
        return true;
    }

    if (rule.lhs == rest[0]) {
        return false;
    }

    return ruleAdheredForSinglePage(rest[1..], rule);
}

fn updateAdheresToRule(update: []const u32, rules: []const Pair) bool {
    if (update.len <= 1) {
        return true;
    }

    const lhs = update[0];
    for (rules) |rule| {
        if (rule.rhs == lhs and !ruleAdheredForSinglePage(update[1..], rule)) {
            return false;
        }
    }

    return updateAdheresToRule(update[1..], rules);
}

fn sumOfMiddleUpdates(parsed: Parsed) u32 {
    var sum: u32 = 0;
    for (parsed.updates.items) |update| {
        if (updateAdheresToRule(update.items, parsed.ordering_rules.items)) {
            sum += update.items[update.items.len / 2];
        }
    }
    return sum;
}

pub fn main() void {
    const input = @import("inputs").input_5;
    const allocator = std.heap.page_allocator;

    const parsed = parse(allocator, input) catch |err| {
        std.debug.panic("Parsing failed with the following error: {any}\n", .{err});
    };

    const stdout = std.io.getStdOut();
    std.fmt.format(stdout.writer(), "Sum of middle pages for valid updates: {d}\n", .{sumOfMiddleUpdates(parsed)}) catch |err| {
        std.debug.panic("Writing to stdout failed with the following error: {any}\n", .{err});
    };
}

const example_input =
    \\47|53
    \\97|13
    \\97|61
    \\97|47
    \\75|29
    \\61|13
    \\75|53
    \\29|13
    \\97|29
    \\53|29
    \\61|53
    \\97|53
    \\61|29
    \\47|13
    \\75|47
    \\97|75
    \\47|61
    \\75|61
    \\47|29
    \\75|13
    \\53|13
    \\
    \\75,47,61,53,29
    \\97,61,53,29,13
    \\75,29,13
    \\75,97,47,61,53
    \\61,13,29
    \\97,13,75,29,47
;

test "parsing" {
    const tiny_input =
        \\47|53
        \\97|13
        \\97|61
        \\
        \\75,47,61,53,29
        \\75,29,13
    ;

    const sut = try parse(std.testing.allocator, tiny_input);
    defer sut.deinit();
    try std.testing.expectEqual(3, sut.ordering_rules.items.len);
    const expected_ordering_rules = [_]Pair{
        Pair{ .lhs = 47, .rhs = 53 },
        Pair{ .lhs = 97, .rhs = 13 },
        Pair{ .lhs = 97, .rhs = 61 },
    };
    try std.testing.expectEqualSlices(Pair, &expected_ordering_rules, sut.ordering_rules.items);

    try std.testing.expectEqual(2, sut.updates.items.len);
    try std.testing.expectEqualSlices(u32, &.{ 75,47,61,53,29 }, sut.updates.items[0].items);
    try std.testing.expectEqualSlices(u32, &.{ 75,29,13 }, sut.updates.items[1].items);
}

test "sum of middle parts" {
    const sut = try parse(std.testing.allocator, example_input);
    defer sut.deinit();
    try std.testing.expectEqual(143, sumOfMiddleUpdates(sut));
}
