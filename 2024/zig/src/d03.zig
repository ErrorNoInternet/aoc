const regex = @import("regex");
const std = @import("std");
const utils = @import("utils.zig");

pub fn p1(contents: []const u8) !u32 {
    var result: u32 = 0;

    const allocator = std.heap.page_allocator;
    var re = try regex.Regex.compile(allocator, "mul\\((\\d+),(\\d+)\\)");
    var start: usize = 0;

    while (true) {
        const capture = try re.captures(contents[start..]);
        if (capture == null) {
            while (contents[start] != '\n') {
                start += 1;
            }
            start += 1;
            if (start == contents.len)
                break;
            continue;
        }

        start += capture.?.boundsAt(0).?.upper;
        result += try std.fmt.parseInt(u32, capture.?.sliceAt(1).?, 10) * try std.fmt.parseInt(u32, capture.?.sliceAt(2).?, 10);
    }

    return result;
}

pub fn p2(contents: []const u8) !u32 {
    var result: u32 = 0;

    const allocator = std.heap.page_allocator;
    var re = try regex.Regex.compile(allocator, "don't\\(\\)|do\\(\\)|mul\\((\\d+),(\\d+)\\)");
    var start: usize = 0;
    var should_do = true;

    while (true) {
        const capture = try re.captures(contents[start..]);
        if (capture == null) {
            while (contents[start] != '\n') {
                start += 1;
            }
            start += 1;
            if (start == contents.len)
                break;
            continue;
        }
        start += capture.?.boundsAt(0).?.upper;

        if (std.mem.eql(u8, capture.?.sliceAt(0).?, "don't()")) {
            should_do = false;
        } else if (std.mem.eql(u8, capture.?.sliceAt(0).?, "do()")) {
            should_do = true;
        } else if (should_do) {
            result += try std.fmt.parseInt(u32, capture.?.sliceAt(1).?, 10) * try std.fmt.parseInt(u32, capture.?.sliceAt(2).?, 10);
        }
    }

    return result;
}

test "d03p1" {
    try std.testing.expect(161 == try p1(@embedFile("examples/d03e1.txt")));
}

test "d03p2" {
    try std.testing.expect(48 == try p2(@embedFile("examples/d03e2.txt")));
}

pub fn main() !void {
    const contents = try utils.get_contents(3);
    std.debug.print("{d}\n", .{try p1(contents)});
    std.debug.print("{d}\n", .{try p2(contents)});
}
