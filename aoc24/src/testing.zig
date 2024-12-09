const std = @import("std");

fn my_cool_FUnction() void {
    std.debug.print("This function is not properly formatted", .{});
}

pub fn main() void {
    std.debug.print("Hello, Advent of Code!\n", .{});
}

test "comp" {
    try std.testing.expect('0' < '9' and '0' < '1');
    const string = "0123456789";

    try std.testing.expectEqual(0, string[0] - '0');
    try std.testing.expectEqual(1, string[1] - '0');
    try std.testing.expectEqual(2, string[2] - '0');
    try std.testing.expectEqual(3, string[3] - '0');
    try std.testing.expectEqual(4, string[4] - '0');
    try std.testing.expectEqual(5, string[5] - '0');
    try std.testing.expectEqual(6, string[6] - '0');
    try std.testing.expectEqual(7, string[7] - '0');
    try std.testing.expectEqual(8, string[8] - '0');
    try std.testing.expectEqual(9, string[9] - '0');
}

// test "slice testing" {
//     const arr = [_]u32{ 1, 3, 4 };
//     const slice = arr[0.. :4];
//     try std.testing.expect(slice.len == 4);
// }
