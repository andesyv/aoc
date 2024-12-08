const std = @import("std");

const Map = struct {
    tokens: []const u8,
    width: u32,
    height: u32,

    fn getCharAtPos(self: @This(), x: u32, y: u32) ?u8 {
        if (self.width <= x) {
            return null;
        }

        const pos = @as(usize, self.width + 1) * y + x;
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

// Ideas for part 2:
// Option 1: Brute force it. Run the simulation for all positions until we detect loops.
// Option 2: Notice how that every time an obstacle causes a loop in the example,
// the guard has already passed through previously, with a direction counter-clockwise
// to the direction when reaching the obstacle. Maybe I can determine when to place an
// obstacle based on:
//  - If the guard will cross a path parallel to one he has already crossed.
//  - And if placing an obstacle after the parallel path will make him traverse in his
//    previously traversed direction.
//
//
// Conclusion: While I believe my idea was good, it failed in practice. And to make matters worse, as a last resort
// I attempted to just brute force it and as Zig is so incredibly fast, it still only took around 1-2 seconds to compute
// the correct answer. Lesson learned: Don't be "clever"

const Ray = struct { pos: Pos, direction: Direction, };

fn raysAlign(test_ray: Ray, ray_to_align_to: Ray, map: Map) !bool {
    if (test_ray.direction != ray_to_align_to.direction) {
        return false;
    }

    if (test_ray.pos.x == ray_to_align_to.pos.x and test_ray.pos.y == ray_to_align_to.pos.y) {
        return true;
    }

    // Check other direction
    if (switch(test_ray.direction) {
        Direction.North, Direction.South => test_ray.pos.x != ray_to_align_to.pos.x,
        Direction.East, Direction.West => test_ray.pos.y != ray_to_align_to.pos.y,
    }) {
        return false;
    }

    // b.pos = a.dir * t + a.pos
    // (b.pos - a.pos) / a.dir = t
    const t = switch(test_ray.direction) {
        Direction.North, Direction.South => try std.math.divExact(i64, @as(i64, ray_to_align_to.pos.y) - @as(i64, test_ray.pos.y), directionY(test_ray.direction)),
        Direction.East, Direction.West => try std.math.divExact(i64, @as(i64, ray_to_align_to.pos.x) - @as(i64, test_ray.pos.x), directionX(test_ray.direction)),
    };
    if (t == 0) {
        return true;
    }
    // If ray is behind, they don't align
    // Nvm, they should align after all...
    // if (t < 0) {
    //     return false;
    // }

    // We also need to check intermediate paths in case there's an obstacle in the way
    // (You'd think "for (0..t)" would work, however, Zig is currently limited to usize...)
    var delta_t: i64 = 0;
    while (delta_t < t) : (delta_t += 1) {
        const x = switch (test_ray.direction) { Direction.North, Direction.South => test_ray.pos.x, else => std.math.cast(u32, directionX(test_ray.direction) * delta_t + test_ray.pos.x) };
        const y = switch (test_ray.direction) { Direction.East, Direction.West => test_ray.pos.y, else => std.math.cast(u32, directionY(test_ray.direction) * delta_t + test_ray.pos.y) };
        if (x == null or y == null) {
            return false;
        }

        const maybe_char = map.getCharAtPos(x.?, y.?);
        if (maybe_char == null) {
            return false;
        } else if (maybe_char.? == '#') {
            return false;
        }
    }

    return true;
}

// I wish Zig had "any" / "all" / "fold" patterns :/
fn anyRayAligns(test_ray: Ray, rays: []const Ray, map: Map) !bool {
    for (rays) |ray| {
        if (try raysAlign(test_ray, ray, map)) {
            return true;
        }
    }
    return false;
}

fn verifyObstacle(obstacle: Pos, guard_paths: []const Ray, map: Map) bool {
    const start_pos = findStartPos(map);
    if (start_pos == null) {
        return false;
    }
    if (guard_paths.len < 1) {
        return false;
    }

    var current_pos = guard_paths[0].pos;
    var current_dir = guard_paths[0].direction;
    // A maximum of 10000 iterations, just to be safe
    for (0..10000) |_| {
        const new_x = wrappedAdd(current_pos.x, directionX(current_dir));
        const new_y = wrappedAdd(current_pos.y, directionY(current_dir));

        const char = if (obstacle.x == new_x and obstacle.y == new_y) '#' else map.getCharAtPos(new_x, new_y);
        if (char == null) {
            return false;
        }

        if (char.? == '#') {
            current_dir = rotate(current_dir);
        } else {
            current_pos = Pos{ .x = new_x, .y = new_y, };
        }

        // for (guard_paths) |path| {
        //     if (path.direction == current_dir and path.pos.x == current_pos.x and path.pos.y == current_pos.y) {
        //         return true;
        //     }
        // }
    }

    return true;
}

fn getCountOfPotentialObstaclesToCreateLoops(allocator: std.mem.Allocator, map: Map) !usize {
    // Most of the logic is the same as in traversePositions. However, instead of keeping
    // track of positions, we keep track of "paths" in the form of rays.
    var guard_paths = std.ArrayList(Ray).init(allocator);
    defer guard_paths.deinit();
    var obstacle_positions = std.AutoHashMap(Pos, void).init(allocator);
    defer obstacle_positions.deinit();

    const start_pos = findStartPos(map);
    if (start_pos == null) {
        return error.FailedToFindStartPos;
    }
    var current_pos = start_pos.?;

    var current_dir = Direction.North;
    try guard_paths.append(Ray{ .pos = current_pos, .direction = current_dir });

    // Traverse path twice.
    // First time, making note of all the different guard paths.
    // Second time, looking for possible obstructions in known paths

    // First traversal:
    while (true) {
        const new_x = wrappedAdd(current_pos.x, directionX(current_dir));
        const new_y = wrappedAdd(current_pos.y, directionY(current_dir));

        const char = map.getCharAtPos(new_x, new_y);
        if (char == null) {
            break;
        }

        if (char.? == '#') {
            current_dir = rotate(current_dir);
            // Every time we rotate, we add a new path
            try guard_paths.append(Ray{ .pos = current_pos, .direction = current_dir, });
        } else {
            current_pos = Pos{ .x = new_x, .y = new_y, };
        }
    }

    current_pos = start_pos.?;
    current_dir = Direction.North;

    // Second traversal:
    while (true) {
        const new_x = wrappedAdd(current_pos.x, directionX(current_dir));
        const new_y = wrappedAdd(current_pos.y, directionY(current_dir));

        const char = map.getCharAtPos(new_x, new_y);
        if (char == null) {
            break;
        }

        if (char.? == '#') {
            current_dir = rotate(current_dir);
        } else {
            // If the next spot is not an obstacle, we imagine what would happen if it was.
            // const theoretical_path = Ray{ .pos = current_pos, .direction = rotate(current_dir), };
            current_pos = Pos{ .x = new_x, .y = new_y, };

            // if (try anyRayAligns(theoretical_path, guard_paths.items, map)) {
                // Note: It turns out only checking for if rays align is not enough. We additionally do need to check
                // if we loop.
                if (verifyObstacle(current_pos, guard_paths.items, map)) {
                    // std.debug.print("Possible obstruction: {any}\n", .{current_pos});
                    try obstacle_positions.put(current_pos, {});
                }
            // }
        }
    }

    // std.debug.print("Current map:\n", .{});
    // for (0..map.height) |y| {
    //     for (0..map.width) |x| {
    //         if (obstacle_positions.contains(Pos{ .x = @as(u32, @intCast(x)), .y = @as(u32, @intCast(y)) })) {
    //             std.debug.print("O", .{});
    //         } else {
    //             const char = map.getCharAtPos(@as(u32, @intCast(x)), @as(u32, @intCast(y)));
    //             std.debug.print("{c}", .{char.?});
    //         }
    //     }
    //     std.debug.print("\n", .{});
    // }

    return obstacle_positions.count();
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

    const potential_obstacles_for_loops = getCountOfPotentialObstaclesToCreateLoops(allocator, map) catch |err| {
        std.debug.panic("Failed to calculate distinct obstacles: {any}\n", .{err});
    };
    std.fmt.format(stdout.writer(), "Count of potential obstacles to create a loop: {d}\n", .{potential_obstacles_for_loops}) catch |err| {
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

test "ray alignment" {
    const input = ".....\n.....\n.....\n.....\n.....";
    const map = parse(input);

    // Directions are different, rays should not align
    var a = Ray{ .pos = Pos{ .x = 0, .y = 0 }, .direction = Direction.East, };
    var b = Ray{ .pos = Pos{ .x = 4, .y = 0 }, .direction = Direction.West, };
    try std.testing.expect(!try raysAlign(a, b, map));

    // Directions are same, should align
    b = Ray{ .pos = Pos{ .x = 4, .y = 0 }, .direction = Direction.East, };
    try std.testing.expect(try raysAlign(a, b, map));

    // Ray should align if behind and in front
    a = Ray{ .pos = Pos{ .x = 3, .y = 1 }, .direction = Direction.North, };
    b = Ray{ .pos = Pos{ .x = 3, .y = 4 }, .direction = Direction.North, };
    try std.testing.expect(try raysAlign(a, b, map));
    try std.testing.expect(try raysAlign(b, a, map));

    // Positions don't align, rays should not align
    b = Ray{ .pos = Pos{ .x = 4, .y = 0 }, .direction = Direction.North, };
    try std.testing.expect(!try raysAlign(a, b, map));

    // This time, the rays don't align because there's an obstacle in the way
    const map_with_obstacle = parse("..#..\n.....\n.....\n.....\n.....");
    a = Ray{ .pos = Pos{ .x = 0, .y = 0 }, .direction = Direction.East, };
    b = Ray{ .pos = Pos{ .x = 4, .y = 0 }, .direction = Direction.East, };
    try std.testing.expect(try raysAlign(a, b, map));
    try std.testing.expect(!try raysAlign(a, b, map_with_obstacle));
}

test "different potential looping obstructions" {
    const map = parse(example_input);
    const result = try getCountOfPotentialObstaclesToCreateLoops(std.testing.allocator, map);
    try std.testing.expectEqual(6, result);
}