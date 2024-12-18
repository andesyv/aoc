const std = @import("std");

// Rules:
//  - find min path
//  - Reindeer starts at S facing east
//  - moving in cardinal direction = 1 point
//  - Rotating 90 degrees = 1000 points

const Pos = struct {
    x: u32,
    y: u32,
};

const Direction = enum(u3) {
    North,
    East,
    South,
    West,
};

const Map = struct {
    tokens: []const u8,
    width: u32,
    height: u32,
    start: Pos,
    goal: Pos,

    fn getCharAtPos(self: @This(), pos: Pos) ?u8 {
        if (self.width <= pos.x) {
            return null;
        }

        const i = (self.width + 1) * pos.y + pos.x;
        if (self.tokens.len <= i) {
            return null;
        }

        return self.tokens[i];
    }

    fn isWall(self: @This(), pos: Pos) bool {
        return if (getCharAtPos(self, pos) == '#') true else false;
    }

    fn getNextPos(self: @This(), conf: Configuration) ?Pos {
        const new_pos = switch (conf.dir) {
            Direction.North => Pos{ .x = conf.pos.x, .y = conf.pos.y -% 1 },
            Direction.East => Pos{ .x = conf.pos.x + 1, .y = conf.pos.y },
            Direction.South => Pos{ .x = conf.pos.x, .y = conf.pos.y + 1 },
            Direction.West => Pos{ .x = conf.pos.x -% 1, .y = conf.pos.y },
        };

        return if (new_pos.x < self.width and new_pos.y < self.height) new_pos else null;
    }

    fn print(self: @This(), traversed_configurations: *std.AutoHashMap(Configuration, ?usize)) void {
        std.debug.print("\n", .{});
        for (0..self.height) |y| {
            for (0..self.width) |x| {
                const pos = Pos{ .x = @as(u32, @intCast(x)), .y = @as(u32, @intCast(y)) };

                if (std.meta.eql(pos, self.start)) {
                    std.debug.print("S", .{});
                } else if (std.meta.eql(pos, self.goal)) {
                    std.debug.print("E", .{});
                } else {
                    const optimal_configuration = getBestTraversedConfiguration(&[_]Configuration{
                        Configuration{ .pos = pos, .dir = Direction.East },
                        Configuration{ .pos = pos, .dir = Direction.South },
                        Configuration{ .pos = pos, .dir = Direction.West },
                        Configuration{ .pos = pos, .dir = Direction.North },
                    }, traversed_configurations);
                    if (optimal_configuration) {
                        std.debug.print("{s}", .{
                            switch (optimal_configuration.?.dir) {
                                Direction.East => ">",
                                Direction.South => "v",
                                Direction.West => "<",
                                Direction.North => "^",
                            }
                        });
                        // std.debug.print("X", .{});
                    } else if (self.isWall(pos)) {
                        std.debug.print("#", .{});
                    } else {
                        std.debug.print(".", .{});
                    }
                }
            }
            std.debug.print("\n", .{});
        }
    }
};

fn parse(input: []const u8) !Map {
    const trimmed = std.mem.trim(u8, input, "\n");
    const newline_pos = std.mem.indexOf(u8, trimmed, "\n");
    const width = newline_pos orelse trimmed.len;
    // Extrapolate height based on the grid width:
    const height = trimmed.len / width;

    const start_i = std.mem.indexOf(u8, trimmed, "S");
    if (start_i == null) {
        return error.MissingStart;
    }

    const start_y = start_i.? / (width + 1);
    const start_x = start_i.? - (width + 1) * start_y;

    const end_i = std.mem.indexOf(u8, trimmed, "E");
    if (end_i == null) {
        return error.MissingEnd;
    }

    const end_y = end_i.? / (width + 1);
    const end_x = end_i.? - (width + 1) * end_y;

    return Map{
        .tokens = trimmed,
        .width = @as(u32, @intCast(width)),
        .height = @as(u32, @intCast(height)),
        .start = Pos{ .x = @as(u32, @intCast(start_x)), .y = @as(u32, @intCast(start_y)) },
        .goal = Pos{ .x = @as(u32, @intCast(end_x)), .y = @as(u32, @intCast(end_y)) },
    };
}

fn getBestTraversedConfiguration(configurations: []const Configuration, traversed_configurations: *std.AutoHashMap(Configuration, ?usize)) ?Configuration {
    const current = configurations[0];

    if (configurations.len == 1) {
        return if (traversed_configurations.*.contains(current)) current else null;
    }

    const other = getBestTraversedConfiguration(configurations[1..], traversed_configurations);

    if (traversed_configurations.*.get(current)) |entry| {
        if (entry) |cost| {
            if (cost < traversed_configurations.*.get(other) orelse std.math.maxInt(usize)) {
                return current;
            }
        }
    }

    return other;
}

fn rotateCW(dir: Direction) Direction {
    return switch (dir) {
        Direction.North => Direction.East,
        Direction.East => Direction.South,
        Direction.South => Direction.West,
        Direction.West => Direction.North,
    };
}

fn rotateCCW(dir: Direction) Direction {
    return switch (dir) {
        Direction.North => Direction.West,
        Direction.West => Direction.South,
        Direction.South => Direction.East,
        Direction.East => Direction.North,
    };
}

const Configuration = struct {
    pos: Pos,
    dir: Direction,

    fn withPos(self: @This(), pos: Pos) Configuration {
        return Configuration {
            .pos = pos,
            .dir = self.dir,
        };
    }

    fn withDir(self: @This(), dir: Direction) Configuration {
        return Configuration {
            .pos = self.pos,
            .dir = dir,
        };
    }
};

// If successfull, returns the relative score cost of traversal from the current configuration
fn traverse(conf: Configuration, map: Map, memoized_results: *std.AutoHashMap(Configuration, ?usize)) !?usize {
    // std.debug.print("Traverse: pos = {any}, dir = {any}\n", .{ pos, dir });
    const found_goal = std.meta.eql(conf.pos, map.goal);
    if (found_goal) {
        return 0;
    }

    // Can't traverse a wall
    if (map.isWall(conf.pos)) {
        return null;
    }

    // Also can't traverse backwards (or it at least doesn't make sense to do so)
    if (memoized_results.contains(conf)) {
        return memoized_results.get(conf).?;
    }

    // Preemptively append the current configuration to prevent infinite loops down the line
    try memoized_results.*.put(conf, null);

    var best_score: ?usize = null;

    // First try moving, as this will always incur the cheapest cost.
    const next_pos = map.getNextPos(conf);
    if (next_pos != null) {
        var candidate = try traverse(conf.withPos(next_pos.?), map, memoized_results);
        if (candidate) |*score| {
            score.* += 1;

            if (score.* < best_score orelse std.math.maxInt(usize)) {
                best_score = score.*;
            }
        }
    }

    // Then try rotating left and right
    var candidate = try traverse(conf.withDir(rotateCW(conf.dir)), map, memoized_results);
    if (candidate) |*score| {
        score.* += 1000;

        if (score.* < best_score orelse std.math.maxInt(usize)) {
            best_score = score.*;
        }
    }

    candidate = try traverse(conf.withDir(rotateCCW(conf.dir)), map, memoized_results);
    if (candidate) |*score| {
        score.* += 1000;

        if (score.* < best_score orelse std.math.maxInt(usize)) {
            best_score = score.*;
        }
    }

    // Make sure to record the current configuration (for memoization)
    try memoized_results.*.put(conf, best_score);

    return best_score;
}

fn calcMinScoreToReachGoal(allocator: std.mem.Allocator, input: []const u8) !usize {
    const map = try parse(input);
    var traversed_configurations = std.AutoHashMap(Configuration, ?usize).init(allocator);
    defer traversed_configurations.deinit();

    const score = try traverse(Configuration{ .pos = map.start, .dir = Direction.East}, map, &traversed_configurations);
    // map.print(&traversed_configurations);
    if (score == null) {
        return error.CouldNotFindGoal;
    }

    return score.?;
}

pub fn main() void {
    std.debug.print("Hello AOC!\n", .{});
}

const example_input_1 =
    \\###############
    \\#.......#....E#
    \\#.#.###.#.###.#
    \\#.....#.#...#.#
    \\#.###.#####.#.#
    \\#.#.#.......#.#
    \\#.#.#####.###.#
    \\#...........#.#
    \\###.#.#####.#.#
    \\#...#.....#.#.#
    \\#.#.#.###.#.#.#
    \\#.....#...#.#.#
    \\#.###.#.#.#.#.#
    \\#S..#.....#...#
    \\###############
; // Minimum score = 7036

const example_input_2 =
    \\#################
    \\#...#...#...#..E#
    \\#.#.#.#.#.#.#.#.#
    \\#.#.#.#...#...#.#
    \\#.#.#.#.###.#.#.#
    \\#...#.#.#.....#.#
    \\#.#.#.#.#.#####.#
    \\#.#...#.#.#.....#
    \\#.#.#####.#.###.#
    \\#.#.#.......#...#
    \\#.#.###.#####.###
    \\#.#.#...#.....#.#
    \\#.#.#.#####.###.#
    \\#.#.#.........#.#
    \\#.#.#.#########.#
    \\#S#.............#
    \\#################
; // Minimum score = 11048

test "parse test" {
    const parse_example =
        \\######
        \\#...E#
        \\#..#.#
        \\#S..##
        \\######
;

    const map = try parse(parse_example);
    try std.testing.expectEqual(6, map.width);
    try std.testing.expectEqual(5, map.height);
    try std.testing.expectEqual(Pos{ .x = 1, .y = 3 }, map.start);
    try std.testing.expectEqual(Pos{ .x = 4, .y = 1 }, map.goal);

    try std.testing.expect(map.isWall(Pos{ .x = 3, .y = 2 }));
}

test "calc min score to reach goal on first example" {
    const score = try calcMinScoreToReachGoal(std.testing.allocator, example_input_1);
    try std.testing.expectEqual(7036, score);
}

test "calc min score to reach goal on second example" {
    const score = try calcMinScoreToReachGoal(std.testing.allocator, example_input_2);
    try std.testing.expectEqual(11048, score);
}
