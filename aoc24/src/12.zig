const std = @import("std");

const Pos = struct {
    x: u32,
    y: u32,
};

// const Map = struct {
//     tokens: []const u8,
//     width: u32,
//     height: u32,
//
//     fn getCharAtPos(self: @This(), pos: Pos) ?u8 {
//         if (self.width <= pos.x) {
//             return null;
//         }
//
//         const i = @as(usize, self.width + 1) * pos.y + pos.x;
//         if (self.tokens.len <= i) {
//             return null;
//         }
//
//         return self.tokens[i];
//     }
//
//
// };
//
// fn parseMap(input: []const u8) Map {
//     const trimmed = std.mem.trim(u8, input, "\n");
//     const newline_pos = std.mem.indexOf(u8, trimmed, "\n");
//     const width = newline_pos orelse trimmed.len;
//     // Extrapolate height based on the width:
//     const height = trimmed.len / width;
//     return Map{
//         .tokens = trimmed,
//         .width = @as(u32, @intCast(width)),
//         .height = @as(u32, @intCast(height)),
//     };
// }

const PlotSet = struct {
    data: std.AutoHashMap(u8, std.ArrayList(Pos)),

    fn deinit(self: *@This()) void {
        var iterator = self.*.data.valueIterator();
        while (iterator.next()) |list| {
            list.*.deinit();
        }
        self.*.data.deinit();
    }

    fn getPlot(self: @This(), plot: u8) ?[] const Pos {
        return if (self.data.get(plot)) |list| list.items else null;
    }

    fn getConnectedPlots(self: @This()) !std.ArrayList(std.ArrayList(Pos)) {
        var sets = try std.ArrayList(std.ArrayList(Pos)).initCapacity(self.data.allocator, self.data.count());



        return sets;
    }
};

fn collectPlots(allocator: std.mem.Allocator, input: []const u8) !PlotSet {
    var set = PlotSet{ .data = std.AutoHashMap(u8, std.ArrayList(Pos)).init(allocator) };

    var line_it = std.mem.tokenizeScalar(u8, std.mem.trim(u8, input, "\n"), '\n');
    var y: u32 = 0;
    while (line_it.next()) |line| {
        for (line, 0..) |c, x| {
            const entry = try set.data.getOrPutValue(c, std.ArrayList(Pos).init(allocator));
            try entry.value_ptr.*.append(Pos{ .x = @as(u32, @intCast(x)), .y = y });
        }
        y += 1;
    }

    return set;
}

fn getSurrounding(pos: Pos) [4]Pos {
    return [_]Pos{
        Pos{ .x = pos.x + 1, .y = pos.y },
        Pos{ .x = pos.x, .y = pos.y + 1 },
        Pos{ .x = pos.x -% 1, .y = pos.y },
        Pos{ .x = pos.x, .y = pos.y -% 1 },
    };
}

fn getPerimeterOfPlots(allocator: std.mem.Allocator, plots: []const Pos) !u32 {
    var positions = std.AutoHashMap(Pos, void).init(allocator);
    defer positions.deinit();
    for (plots) |plot| {
        try positions.put(plot, {});
    }

    var possible_max = @as(u32, @intCast(plots.len * 4));
    for (plots) |plot| {
        for (getSurrounding(plot)) |neighbour| {
            if (positions.contains(neighbour)) {
                possible_max -= 1;
            }
        }
    }

    return possible_max;
}

fn getFencingPrice(allocator: std.mem.Allocator, plot_set: PlotSet) !u32 {
    var sum: u32 = 0;

    var it = plot_set.data.valueIterator();
    while (it.next()) |plot| {
        const perimeter = try getPerimeterOfPlots(allocator, plot.items);
        sum += perimeter * @as(u32, @intCast(plot.items.len));
    }

    return sum;
}

pub fn main() void {}

const small_example =
    \\AAAA
    \\BBCD
    \\BBCC
    \\EEEC
;

const medium_example =
    \\OOOOO
    \\OXOXO
    \\OOOOO
    \\OXOXO
    \\OOOOO
;

const big_example =
    \\RRRRIICCFF
    \\RRRRIICCCF
    \\VVRRRCCFFF
    \\VVRCCCJFFF
    \\VVVVCJJCFE
    \\VVIVCCJJEE
    \\VVIIICJJEE
    \\MIIIIIJJEE
    \\MIIISIJEEE
    \\MMMISSJEEE
;

test "perimeter of plots" {
    var plots = try collectPlots(std.testing.allocator, small_example);
    defer plots.deinit();

    var sut = try getPerimeterOfPlots(std.testing.allocator, plots.data.get('A').?.items);
    try std.testing.expectEqual(10, sut);
    sut = try getPerimeterOfPlots(std.testing.allocator, plots.data.get('C').?.items);
    try std.testing.expectEqual(10, sut);

    sut = try getPerimeterOfPlots(std.testing.allocator, plots.data.get('B').?.items);
    try std.testing.expectEqual(8, sut);
    sut = try getPerimeterOfPlots(std.testing.allocator, plots.data.get('E').?.items);
    try std.testing.expectEqual(8, sut);

    sut = try getPerimeterOfPlots(std.testing.allocator, plots.data.get('D').?.items);
    try std.testing.expectEqual(4, sut);

    plots.deinit();
    plots = try collectPlots(std.testing.allocator, medium_example);

    sut = try getPerimeterOfPlots(std.testing.allocator, plots.data.get('O').?.items);
    try std.testing.expectEqual(20 + 16, sut);
    sut = try getPerimeterOfPlots(std.testing.allocator, plots.data.get('X').?.items);
    try std.testing.expectEqual(16, sut);
}

test "small garden fence cost" {
    var plots = try collectPlots(std.testing.allocator, small_example);
    defer plots.deinit();

    const sut = try getFencingPrice(std.testing.allocator, plots);
    try std.testing.expectEqual(140, sut);
}

test "medium garden fence cost" {
    var plots = try collectPlots(std.testing.allocator, medium_example);
    defer plots.deinit();

    const sut = try getFencingPrice(std.testing.allocator, plots);
    try std.testing.expectEqual(772, sut);
}

test "big garden fence cost" {
    var plots = try collectPlots(std.testing.allocator, big_example);
    defer plots.deinit();

    const sut = try getFencingPrice(std.testing.allocator, plots);
    try std.testing.expectEqual(1930, sut);
}
