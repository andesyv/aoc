const std = @import("std");

// The puzzle input is 20k characters. As IDs as stored sequentially, this means the potential largest entry needs to
// be able to store a number with 20k digits. Which is within the limit of a u16 (2¹⁶ = 65536). However, we also need
// to store an "empty" version of an ID for empty space. Zig packs pointers and slices efficiently, utilizing every bit
// available. This is also planned for optionals, but it's not here yet: https://github.com/ziglang/zig/issues/104
// So I'll do the packing myself here by using negative numbers as empty space, as the sign also only takes up 1 bit
// which leaves us with 2¹⁵ = which is still more than enough.
fn expandDiskMap(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(i16) {
    var list = try std.ArrayList(i16).initCapacity(allocator, input.len * 2);

    var file = true;
    var id: i16 = 0;
    for (input) |c| {
        const count = c - '0';
        for (0..count) |_| {
            try list.append(if (file) try (std.math.cast(i16, id) orelse error.IntegerOverflow) else @as(i16, -1));
        }

        if (file) {
            id += 1;
        }
        file = !file;
    }

    return list;
}

fn sortExpandedDiskMap(disk_map: []i16) void {
    if (disk_map.len == 0) {
        return;
    }

    // We have two indices i and j, that goes from left to right and right to left, respectfully.
    var i: usize = 0;
    var j: usize = disk_map.len - 1;

    while (i < j) {
        while (i < j and 0 <= disk_map[i]) : (i += 1) {}
        while (i < j and disk_map[j] < 0) : (j -= 1) {}
        if (i >= j) {
            return;
        }

        disk_map[i] = disk_map[j];
        disk_map[j] = -1; // It saves exactly 1 CPU instruction to directly write -1 instead of swapping
        // std.mem.swap(i16, &disk_map[i], &disk_map[j]);
    }
}

fn calculateChecksum(disk_map: []const i16) !u128 {
    var sum: u128 = 0;

    for (disk_map, 0..) |id, pos| {
        // When we reach an empty spot, we're done.
        if (id < 0) {
            return sum;
        }

        sum += @as(u128, @intCast(id)) * try (std.math.cast(u128, pos) orelse error.IntegerOverflow);
    }

    return sum;
}

fn calculateSortedFilesystemChecksum(allocator: std.mem.Allocator, disk_map: []const u8) !u128 {
    var expanded_map = try expandDiskMap(allocator, std.mem.trim(u8, disk_map, "\n"));
    defer expanded_map.deinit();

    sortExpandedDiskMap(expanded_map.items);

    return calculateChecksum(expanded_map.items);
}

fn printExpandedDiskMap(expanded_disk_map: []const i16) void {
    std.debug.print("Expanded disk map:\n", .{});
    for (expanded_disk_map) |c| {
        if (c < 0) {
            std.debug.print(".", .{});
        } else {
            std.debug.print("{d}", .{c});
        }
    }
    std.debug.print("\n", .{});
}

pub fn main() void {
    const input = @import("inputs").input_9;
    const allocator = std.heap.page_allocator;

    const checksum = calculateSortedFilesystemChecksum(allocator, input) catch |err| {
        std.debug.panic("Calculating checksum of filesystem failed with this error: {any}\n", .{err});
    };

    const stdout = std.io.getStdOut();
    std.fmt.format(stdout.writer(), "Checksum of filesystem: {d}\n", .{checksum}) catch |err| {
        std.debug.panic("Writing to stdout failed with the following error: {any}\n", .{err});
    };
}

const example_input = "2333133121414131402";

test "expand disk map" {
    const sut = try expandDiskMap(std.testing.allocator, "12345");
    defer sut.deinit();

    // print_expanded_disk_map(sut.items);

    const expected = [_]i16{ 0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2 };
    try std.testing.expectEqualSlices(i16, &expected, sut.items);
}

test "sort expanded disk map" {
    var sut = [_]i16{ 0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2 };
    sortExpandedDiskMap(&sut);

    const expected = [_]i16{ 0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1 };
    try std.testing.expectEqualSlices(i16, &expected, &sut);
}

test "example filesystem checksum" {
    const result = try calculateSortedFilesystemChecksum(std.testing.allocator, example_input);
    try std.testing.expectEqual(1928, result);
}
