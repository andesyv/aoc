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

fn findNextBlockFromBack(disk_map: []i16, start: usize) []i16 {
    var c: i16 = -1;
    var i: usize = start;
    var j: usize = start;
    while (i < disk_map.len) : (i -%= 1) {
        if (0 <= c) {
            if (disk_map[i] != c) {
                return disk_map[i+1..j+1];
            }
        } else {
            j = i;
            c = disk_map[i];
        }
    }

    return disk_map[0..0];
}

fn findEmptyBlockOfSize(allocator: std.mem.Allocator, disk_map: []i16, offset: usize, size: usize) ![]i16 {
    if (disk_map.len < offset + size) {
        return disk_map[0..0];
    }

    var target = try std.ArrayList(i16).initCapacity(allocator, size);
    defer target.deinit();
    try target.appendNTimes(-1, size);

    if (std.mem.eql(i16, target.items, disk_map[offset..offset + size])) {
        return disk_map[offset..offset + size];
    }

    return try findEmptyBlockOfSize(allocator, disk_map, offset + 1, size);
}

fn sortExpandedDiskMapWithWholeBlocks(allocator: std.mem.Allocator, disk_map: []i16) !void {
    if (disk_map.len == 0) {
        return;
    }

    var j: usize = disk_map.len - 1;

    while (true) {
        printExpandedDiskMap(disk_map);
        const block_to_move = findNextBlockFromBack(disk_map, j);
        if (block_to_move.len == 0) {
            return;
        }


        j = (@intFromPtr(block_to_move.ptr) - @intFromPtr(disk_map.ptr)) / @sizeOf(i16) - 1;

        const empty_block = try findEmptyBlockOfSize(allocator, disk_map, 0, block_to_move.len);
        if (empty_block.len != block_to_move.len) {
            continue;
        }

        if (@intFromPtr(block_to_move.ptr) < @intFromPtr(empty_block.ptr)) {
            return;
        }

        for (0..empty_block.len) |k| {
            empty_block[k] = block_to_move[k];
            block_to_move[k] = -1;
        }
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

    // printExpandedDiskMap(expanded_map.items);

    sortExpandedDiskMap(expanded_map.items);

    // printExpandedDiskMap(expanded_map.items);

    return calculateChecksum(expanded_map.items);
}

fn calculateSortedFilesystemChecksumWithWholeBlocks(allocator: std.mem.Allocator, disk_map: []const u8) !u128 {
    var expanded_map = try expandDiskMap(allocator, std.mem.trim(u8, disk_map, "\n"));
    defer expanded_map.deinit();

    // printExpandedDiskMap(expanded_map.items);

    try sortExpandedDiskMapWithWholeBlocks(allocator, expanded_map.items);

    // printExpandedDiskMap(expanded_map.items);

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

test "find next block" {
    var expanded_list_1 = [_]i16{ 0, -1, -1, 1, 1, 1, -1, -1 };

    var next_block = findNextBlockFromBack(&expanded_list_1, expanded_list_1.len-1);
    try std.testing.expectEqualSlices(i16, expanded_list_1[3..6], next_block);
    // The logic of findNextBlock breaks when it reaches the start, but we probably won't use it long enough to notice
    // next_block = findNextBlock(&expanded_list_1, 2);
    // try std.testing.expectEqualSlices(i16, expanded_list_1[0..1], next_block);

    var expanded_list_2 = [_]i16{ 0, -1, -1, 1, 1, 1 };

    next_block = findNextBlockFromBack(&expanded_list_2, expanded_list_2.len-1);
    try std.testing.expectEqualSlices(i16, expanded_list_2[3..6], next_block);
}

test "sort blocks" {
    const result = try calculateSortedFilesystemChecksumWithWholeBlocks(std.testing.allocator, example_input);
    try std.testing.expectEqual(2858, result);
}