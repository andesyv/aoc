const std = @import("std");

const Map = struct {
    tokens: []const u8,
    width: u32,
    height: u32,

    fn getCharAtPos(self: @This(), x: u32, y: u32) ?u8 {
        if (self.width <= x) {
            return null;
        }

        const pos = (self.width + 1) * y + x;
        if (self.tokens.len <= pos) {
            return null;
        }

        return self.tokens[pos];
    }
};

const Pos = struct { x: u32, y: u32, };

fn parse(input: []const u8) Map {
    const trimmed = std.mem.trim(u8, input, "\n");
    const newline_pos = std.mem.indexOf(u8, trimmed, "\n");
    const width = newline_pos orelse trimmed.len;
    // Extrapolate height based on the width:
    const height = trimmed.len / width;
    return Map{
        .tokens = trimmed,
        .width = @as(u32, @intCast(width)),
        .height = @as(u32, @intCast(height)),
    };
}

fn findStartPos(map: Map) ?Pos {
    const index = std.mem.indexOf(u8, map.tokens, "^");
    if (index == null) {
        return null;
    }

    const i = @as(u32, @intCast(index.?));
    const y = i / (map.width + 1);
    return Pos{ .x = i - y * (map.width + 1), .y = y };
}

fn wrappedAdd(a: u32, b: i8) u32 {
    // Note: Only wraps in negative direction. I.e. 0 + (-1) => maxInt(u32), maxInt(u32) + 1 => panic!
    return if (b < 0) a -% @as(u32, @intCast(-b)) else a + @as(u32, @intCast(b));
}

const Direction = enum(u4) { North, East, South, West, };

fn rotate(dir: Direction) Direction {
    return @enumFromInt((@intFromEnum(dir) + 1) % 4);
}

fn directionX(dir: Direction) i8 {
    return switch (dir) {
        Direction.North, Direction.South => 0,
        Direction.East => 1,
        Direction.West => -1,
    };
}

fn directionY(dir: Direction) i8 {
    return switch (dir) {
        Direction.East, Direction.West => 0,
        Direction.North => -1,
        Direction.South => 1,
    };
}

fn traversePositions(allocator: std.mem.Allocator, map: Map) !std.AutoHashMap(Pos, void) {
    var visited_positions = std.AutoHashMap(Pos, void).init(allocator);
    var current_pos = findStartPos(map);
    // if (current_pos == null) {
    //     return list;
    // }

    var current_dir = Direction.North;

    while (current_pos) |pos| {
        try visited_positions.put(pos, {});

        const new_x = wrappedAdd(pos.x, directionX(current_dir));
        const new_y = wrappedAdd(pos.y, directionY(current_dir));

        const char = map.getCharAtPos(new_x, new_y);
        if (char == null) {
            break;
        }

        if (char.? == '#') {
            current_dir = rotate(current_dir);
        } else {
            current_pos = Pos{ .x = new_x, .y = new_y, };
        }
    }

    return visited_positions;
}

fn getDistinctGuardPositionsForPatrol(allocator: std.mem.Allocator, map: Map) !usize {
    var positions = try traversePositions(allocator, map);
    defer positions.deinit();
    return positions.count();
}

pub fn main() void {
    const input = @import("inputs").input_6;
    const allocator = std.heap.page_allocator;

    const map = parse(input);
    const distinct_positions = getDistinctGuardPositionsForPatrol(allocator, map) catch |err| {
        std.debug.panic("Failed to calculate distinct positions: {any}\n", .{err});
    };

    const stdout = std.io.getStdOut();
    std.fmt.format(stdout.writer(), "Distinct positions for guard: {d}\n", .{distinct_positions}) catch |err| {
        std.debug.panic("Writing to stdout failed with the following error: {any}\n", .{err});
    };
}

const example_input =
    \\....#.....
    \\.........#
    \\..........
    \\..#.......
    \\.......#..
    \\..........
    \\.#..^.....
    \\........#.
    \\#.........
    \\......#...
;

test "find start pos" {
    const sut = Map{ .tokens = "....\n..^.\n....", .width = 4, .height = 3 };
    const result = findStartPos(sut) orelse Pos{ .x = 10, .y = 10 };
    try std.testing.expectEqual(Pos{ .x = 2, .y = 1 }, result);
}

test "distinct guard patrol positions" {
    const map = parse(example_input);
    const result = try getDistinctGuardPositionsForPatrol(std.testing.allocator, map);
    try std.testing.expectEqual(41, result);
}
