const std = @import("std");

const Grid = struct {
    data: []const u8,
    width: usize,
    height: usize,

    fn getCharAtPos(self: @This(), x: u32, y: u32) ?u8 {
        if (self.width <= x) {
            return null;
        }

        const pos = (self.width + 1) * y + x;
        if (self.data.len <= pos) {
            return null;
        }

        return self.data[pos];
    }
};

fn parseToGrid(input: []const u8) Grid {
    const trimmed = std.mem.trim(u8, input, "\n");
    const newline_pos = std.mem.indexOf(u8, trimmed, "\n");
    const width = newline_pos orelse trimmed.len;
    // Extrapolate height based on the grid width:
    const height = trimmed.len / width;
    return Grid{
        .data = trimmed,
        .width = width,
        .height = height
    };
}

// Zig does not allow for arithmetic between integers of different signness (on purpose)
fn wrappedAdd(a: u32, b: i8) u32 {
    // Note: Only wraps in negative direction. I.e. 0 + (-1) => maxInt(u32), maxInt(u32) + 1 => panic!
    return if (b < 0) a -% @as(u32, @intCast(-b)) else a + @as(u32, @intCast(b));
}

fn findXMAS(grid: Grid, search_slice: []const u8, pos: [2]u32, dir: [2]i8) bool {
    // Base case: We've found the entire string
    if (search_slice.len == 0) {
        return true;
    }

    const maybe_char = grid.getCharAtPos(pos[0], pos[1]);
    if (maybe_char == null) {
        return false;
    }

    if (maybe_char.? != search_slice[0]) {
        return false;
    }

    const x = wrappedAdd(pos[0], dir[0]);
    const y = wrappedAdd(pos[1], dir[1]);
    const new_pos = [_]u32{ x, y };
    return findXMAS(grid, search_slice[1..], new_pos, dir);
}

fn findCountOfAllXMAS(grid: Grid) u32 {
    var count: u32 = 0;

    const dirs = [_][2]i8{
        .{  1,  0 },
        .{  1,  1 },
        .{  0,  1 },
        .{ -1,  1 },
        .{ -1,  0 },
        .{ -1, -1 },
        .{  0, -1 },
        .{  1, -1 },
    };

    const search_slice = "XMAS";

    for (0..grid.width) |x| {
        for (0..grid.height) |y| {
            for (dirs) |dir| {
                const pos = [_]u32{ @as(u32, @intCast(x)), @as(u32, @intCast(y)) };
                if (findXMAS(grid, search_slice, pos, dir)) {
                    // std.debug.print("Found an XMAS at pos {any}, dir: {any}\n", .{ pos, dir });
                    count += 1;
                }
            }
        }
    }

    return count;
}

pub fn main() void {
    // No memory allocation needed for this one! :O
    const input = @import("inputs").input_4;

    const grid = parseToGrid(input);
    const stdout = std.io.getStdOut();
    std.fmt.format(stdout.writer(), "Count of all XMAS words: {d}\n", .{findCountOfAllXMAS(grid)}) catch |err| {
        std.debug.panic("Writing to stdout failed with the following error: {any}\n", .{err});
    };
}

const example_input =
    \\MMMSXXMASM
    \\MSAMXMSMSA
    \\AMXSXMAAMM
    \\MSAMASMSMX
    \\XMASAMXAMM
    \\XXAMMXXAMA
    \\SMSMSASXSS
    \\SAXAMASAAA
    \\MAMMMXMMMM
    \\MXMXAXMASX
;

test "tiny grid" {
    const tiny_grid_input = "XMA";
    const grid = parseToGrid(tiny_grid_input);
    try std.testing.expectEqual(3, grid.width);
    try std.testing.expectEqual(1, grid.height);

    const sut = try (grid.getCharAtPos(0, 0) orelse error.NoValue);
    try std.testing.expectEqual('X', sut);
}

test "medium grid" {
    const medium_grid_input = "XMA\nSXM\nXMX\n";

    const grid = parseToGrid(medium_grid_input);
    try std.testing.expectEqual(3, grid.width);
    try std.testing.expectEqual(3, grid.height);

    var sut = try (grid.getCharAtPos(0, 0) orelse error.NoValue);
    try std.testing.expectEqual('X', sut);

    sut = try (grid.getCharAtPos(1, 1) orelse error.NoValue);
    try std.testing.expectEqual('X', sut);

    sut = try (grid.getCharAtPos(1, 2) orelse error.NoValue);
    try std.testing.expectEqual('M', sut);

    try std.testing.expectEqual(null, grid.getCharAtPos(3, 0));
    try std.testing.expectEqual(null, grid.getCharAtPos(0, 3));
    try std.testing.expectEqual(null, grid.getCharAtPos(3, 3));
}

test "word hunt" {
    const grid = parseToGrid(example_input);
    const result = findCountOfAllXMAS(grid);
    try std.testing.expectEqual(18, result);
}