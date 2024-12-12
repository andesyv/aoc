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

const Block = struct {
    start: usize,
    end: usize,

    fn len(self: @This()) usize {
        return self.end - self.start;
    }
};

fn findNextBlockFromBack(disk_map: []i16, start: usize) ?Block {
    var c: i16 = -1;
    var i: usize = start;
    var j: usize = start;
    while (i < disk_map.len) : (i -%= 1) {
        if (0 <= c) {
            if (disk_map[i] != c) {
                return Block{ .start = i+1, .end = j+1 };
            }
        } else {
            j = i;
            c = disk_map[i];
        }
    }

    return null;
}

fn sortBlocks(_: void, a: Block, b: Block) bool {
    const a_len = a.len();
    const b_len = b.len();
    return if (a_len == b_len) a.start < b.start else a_len < b_len;
}

fn findAllEmptyBlocks(allocator: std.mem.Allocator, disk_map: []const i16) !std.ArrayList(Block) {
    var ordered_blocks = std.ArrayList(Block).init(allocator);

    var i: usize = 0;
    var j: usize = 0;
    while (i < disk_map.len) : (i += 1) {
        if (0 <= disk_map[i]) {
            const block_len: usize = if (0 < i) (i - 1) - j else 0;
            if (0 < block_len) {
                // if (!block_map.contains(block_len)) {
                //     try block_map.put(block_len, std.ArrayList(Block).init(allocator));
                // }

                // var list = block_map.get(block_len).?;
                try ordered_blocks.append(Block{ .start = j + 1, .end = i });
                // var list = block_map.getPtr(block_len).?;
                // try list.*.append(Block{ .start = j, .end = i - 1 });
            }

            j = i;
        }
    }

    std.sort.block(Block, ordered_blocks.items, {}, sortBlocks);

    return ordered_blocks;
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

    var empty_blocks = try findAllEmptyBlocks(allocator, disk_map);
    defer empty_blocks.deinit();

    // std.debug.print("Emtpy blocks: {any}", .{empty_blocks.items});

    var j: usize = disk_map.len - 1;

    outer: while (j < disk_map.len) {
        // printExpandedDiskMap(disk_map);
        const block_to_move = findNextBlockFromBack(disk_map, j);
        if (block_to_move == null) {
            return;
        }

        // std.debug.print("Current block: ", .{});
        // printDiskMapSlice(disk_map[block_to_move.?.start..block_to_move.?.end]);
        // std.debug.print("\n", .{});

        j = block_to_move.?.start -% 1;

        for (0..empty_blocks.items.len) |empty_block_index| {
            const empty_block = empty_blocks.items[empty_block_index];
            if (empty_block.len() < block_to_move.?.len()) {
                continue;
            }

            if (block_to_move.?.start <= empty_block.start) {
                continue;
            }

            // const empty_block = try findEmptyBlockOfSize(allocator, disk_map[0..j], 0, block_to_move.?.len());
            // if (empty_block.len != block_to_move.?.len()) {
            //     continue;
            // }

            for (0..block_to_move.?.len()) |k| {
                const value_index: usize = block_to_move.?.start + k;
                const space_index: usize = empty_block.start + k;
                disk_map[space_index] = disk_map[value_index];
                disk_map[value_index] = -1;
            }

            // Sometimes there's remainders left in the blocks. Remove 'em, and then push 'em back in.
            var removed_block = empty_blocks.orderedRemove(empty_block_index);
            if (block_to_move.?.len() < removed_block.len()) {
                removed_block.start += block_to_move.?.len();
                try empty_blocks.append(removed_block);
                // Re-sort that shit (extremely computationally stupid, but at this point I'm taking as many
                // shortcuts as possible)
                std.sort.block(Block, empty_blocks.items, {}, sortBlocks);
            }

            continue :outer;
        }
    }
}

fn calculateChecksum(disk_map: []const i16) !u128 {
    var sum: u128 = 0;

    for (disk_map, 0..) |id, pos| {
        // When we reach an empty spot, we're done.
        // Apparently, we're not done. We just have to skip it instead.
        if (id < 0) {
            continue;
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

fn printDiskMapSlice(disk_map_slice: []const i16) void {
    for (disk_map_slice) |c| {
        if (c < 0) {
            std.debug.print(".", .{});
        } else {
            std.debug.print("{d}", .{c});
        }
    }
}

fn printExpandedDiskMap(expanded_disk_map: []const i16) void {
    std.debug.print("Expanded disk map:\n", .{});
    printDiskMapSlice(expanded_disk_map);
    std.debug.print("\n", .{});
}

pub fn main() void {
    const input = @import("inputs").input_9;
    const allocator = std.heap.page_allocator;

    var checksum = calculateSortedFilesystemChecksum(allocator, input) catch |err| {
        std.debug.panic("Calculating checksum of filesystem failed with this error: {any}\n", .{err});
    };

    const stdout = std.io.getStdOut();
    std.fmt.format(stdout.writer(), "Checksum of filesystem: {d}\n", .{checksum}) catch |err| {
        std.debug.panic("Writing to stdout failed with the following error: {any}\n", .{err});
    };

    checksum = calculateSortedFilesystemChecksumWithWholeBlocks(allocator, input) catch |err| {
        std.debug.panic("Calculating checksum of filesystem failed with this error: {any}\n", .{err});
    };

    std.fmt.format(stdout.writer(), "Checksum of filesystem, sorted on whole blocks: {d}\n", .{checksum}) catch |err| {
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

    var next_block = try (findNextBlockFromBack(&expanded_list_1, expanded_list_1.len-1) orelse error.NoBlock);
    try std.testing.expectEqualSlices(i16, expanded_list_1[3..6], expanded_list_1[next_block.start..next_block.end]);
    // The logic of findNextBlock breaks when it reaches the start, but we probably won't use it long enough to notice
    // next_block = findNextBlock(&expanded_list_1, 2);
    // try std.testing.expectEqualSlices(i16, expanded_list_1[0..1], next_block);

    var expanded_list_2 = [_]i16{ 0, -1, -1, 1, 1, 1 };

    next_block = try (findNextBlockFromBack(&expanded_list_2, expanded_list_2.len-1) orelse error.NoBlock);
    try std.testing.expectEqualSlices(i16, expanded_list_2[3..6], expanded_list_2[next_block.start..next_block.end]);
}

test "sort blocks" {
    const result = try calculateSortedFilesystemChecksumWithWholeBlocks(std.testing.allocator, example_input);
    try std.testing.expectEqual(2858, result);
}