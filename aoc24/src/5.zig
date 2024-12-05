const std = @import("std");

const Rule = struct{ lhs: u32, rhs: u32 };

const Parsed = struct {
    ordering_rules: std.ArrayList(Rule),
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

fn parseOrderingRule(line: []const u8) ?Rule {
    if (line.len < 5) {
        return null;
    }
    // All numbers in the task input are 2 digits, which we'll make us of here
    const lhs = std.fmt.parseInt(u32, line[0..2], 0) catch { return null; };
    if (line[2] != '|') {
        return null;
    }
    const rhs = std.fmt.parseInt(u32, line[3..5], 0) catch { return null; };
    return Rule{ .lhs = lhs, .rhs = rhs };
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
    var ordering_rules = std.ArrayList(Rule).init(allocator);
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

fn ruleAdheredForSinglePage(rest: []const u32, rule: Rule) bool {
    if (rest.len == 0) {
        return true;
    }

    if (rule.lhs == rest[0]) {
        return false;
    }

    return ruleAdheredForSinglePage(rest[1..], rule);
}

fn updateAdheresToRule(update: []const u32, rules: []const Rule) bool {
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

// Here's an observation. As I already have a function to determine whether a current page is in the correct
// position relating to the ones on the right of it, I might be able to use this for a classic "bubble sort" solution.
// Starting from the left, determine whether the current page is in the correct position. If not, swap it with the next
// page. Otherwise, continue from the next page.
// E.g.:
// |97,13,75,29,47
// 97,|13,75,29,47
// 97,|75,13,29,47
// 97,75,|13,29,47
// 97,75,|29,13,47
// 97,75,29,|13,47
// 97,75,29,|47,13
// 97,75,29,47,|13

fn ruleBasedBubbleSort(sorted_list: []u32, curr: u32, rest: []const u32, rules: []const Rule) void {
    if (rest.len == 0) {
        sorted_list[0] = curr;
        return;
    }

    for (rules) |rule| {
        if (rule.rhs == curr and !ruleAdheredForSinglePage(rest[0..], rule)) {
            // If not ordered, append the next element, and continue with the
            // current as the next element.
            sorted_list[0] = rest[0];
            ruleBasedBubbleSort(sorted_list[1..], curr, rest[1..], rules);
            return;
        }
    }

    // Everything is ordered, we continue with the next element
    sorted_list[0] = curr;
    ruleBasedBubbleSort(sorted_list[1..], rest[0], rest[1..], rules);
}

fn sortUpdateList(sorted_list: []u32, update: []const u32, rules: []const Rule) !void {
    if (update.len < 0) {
        return;
    }

    ruleBasedBubbleSort(sorted_list, update[0], update[1..], rules);
}

fn sumOfMiddlesOfFixedUpdates(allocator: std.mem.Allocator, parsed: Parsed) !u32 {
    var working_list = std.ArrayList(u32).init(allocator);
    defer working_list.deinit();
    var sum: u32 = 0;

    for (parsed.updates.items) |update| {
        if (updateAdheresToRule(update.items, parsed.ordering_rules.items)) {
            continue;
        }

        working_list.clearRetainingCapacity();
        try working_list.appendNTimes(0, update.items.len);

        // Keep bubble sorting until nothing changes (at which point it should be done sorting)
        // (kind of a hacky solution to a problem I discovered a bit too late)
        var sort_counter: usize = 0;
        while (sort_counter == 0 or !updateAdheresToRule(working_list.items, parsed.ordering_rules.items)) : (sort_counter += 1) {
            if (update.items.len <= sort_counter) {
                std.debug.panic("List still wasn't sorted after max iterations", .{});
            }

            if (sort_counter == 0) {
                try sortUpdateList(working_list.items, update.items, parsed.ordering_rules.items);
            } else {
                try sortUpdateList(working_list.items, working_list.items, parsed.ordering_rules.items);
            }
            // std.debug.print("Sorted list is {any}\n", .{working_list.items});
        }

        sum += working_list.items[working_list.items.len / 2];
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

    const result = sumOfMiddlesOfFixedUpdates(allocator, parsed) catch |err| {
        std.debug.panic("Calculating the middle of fixed updates failed with: {any}", .{err});
    };
    std.fmt.format(stdout.writer(), "Sum of middle pages for the fixed updates: {d}\n", .{result}) catch |err| {
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
    const expected_ordering_rules = [_]Rule{
        Rule{ .lhs = 47, .rhs = 53 },
        Rule{ .lhs = 97, .rhs = 13 },
        Rule{ .lhs = 97, .rhs = 61 },
    };
    try std.testing.expectEqualSlices(Rule, &expected_ordering_rules, sut.ordering_rules.items);

    try std.testing.expectEqual(2, sut.updates.items.len);
    try std.testing.expectEqualSlices(u32, &.{ 75,47,61,53,29 }, sut.updates.items[0].items);
    try std.testing.expectEqualSlices(u32, &.{ 75,29,13 }, sut.updates.items[1].items);
}

test "sum of middle parts" {
    const sut = try parse(std.testing.allocator, example_input);
    defer sut.deinit();
    try std.testing.expectEqual(143, sumOfMiddleUpdates(sut));
}

test "sum or sorted middle parts" {
    const sut = try parse(std.testing.allocator, example_input);
    defer sut.deinit();
    const result = try sumOfMiddlesOfFixedUpdates(std.testing.allocator, sut);
    try std.testing.expectEqual(123, result);
}
