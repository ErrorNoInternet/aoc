const std = @import("std");
const utils = @import("utils.zig");

pub fn p1(contents: []const u8) !u32 {
    var left_nums: [1000]i32 = undefined;
    var right_nums: [1000]i32 = undefined;

    var i: usize = 0;
    var line_split = std.mem.tokenizeScalar(u8, contents, '\n');
    while (line_split.next()) |line| : (i += 1) {
        var space_split = std.mem.tokenizeScalar(u8, line, ' ');
        left_nums[i] = try std.fmt.parseInt(i32, space_split.next().?, 10);
        right_nums[i] = try std.fmt.parseInt(i32, space_split.next().?, 10);
    }

    std.mem.sortUnstable(i32, &left_nums, {}, comptime std.sort.asc(i32));
    std.mem.sortUnstable(i32, &right_nums, {}, comptime std.sort.asc(i32));

    var result: u32 = 0;
    for (left_nums, right_nums) |left_num, right_num| {
        result += @abs(left_num - right_num);
    }
    return result;
}

pub fn p2(contents: []const u8) !u32 {
    var left_nums: [1000]u32 = undefined;
    var right_nums: [1000]u32 = undefined;

    var i: usize = 0;
    var line_split = std.mem.tokenizeScalar(u8, contents, '\n');
    while (line_split.next()) |line| : (i += 1) {
        var space_split = std.mem.tokenizeScalar(u8, line, ' ');
        left_nums[i] = try std.fmt.parseInt(u32, space_split.next().?, 10);
        right_nums[i] = try std.fmt.parseInt(u32, space_split.next().?, 10);
    }

    var result: u32 = 0;
    for (left_nums[0..i]) |left_num| {
        var count: u16 = 0;
        for (right_nums[0..i]) |right_num| {
            if (left_num == right_num) count += 1;
        }
        result += left_num * count;
    }
    return result;
}

test "d01p1" {
    try std.testing.expect(11 == try p1(@embedFile("examples/d01e1.txt")));
}

test "d01p2" {
    try std.testing.expect(31 == try p2(@embedFile("examples/d01e1.txt")));
}

pub fn main() !void {
    const contents = try utils.get_contents(1);
    std.debug.print("{d}\n", .{try p1(contents)});
    std.debug.print("{d}\n", .{try p2(contents)});
}
