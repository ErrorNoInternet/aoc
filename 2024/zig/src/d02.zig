const std = @import("std");
const utils = @import("utils.zig");

pub fn p1(contents: []const u8) !u32 {
    var result: u32 = 0;

    var line_split = std.mem.tokenizeScalar(u8, contents, '\n');
    lines: while (line_split.next()) |line| {
        var inc = true;
        var dec = true;
        var last_level: ?i32 = null;

        var space_split = std.mem.tokenizeScalar(u8, line, ' ');
        while (space_split.next()) |number| {
            const level = try std.fmt.parseInt(i32, number, 10);

            if (last_level) |last| {
                if (level > last)
                    dec = false;
                if (level < last)
                    inc = false;

                const diff = @abs(level - last);
                if (diff == 0 or diff > 3) continue :lines;
            }

            last_level = level;
        }

        if (inc or dec) result += 1;
    }

    return result;
}

pub fn p2(contents: []const u8) !u32 {
    var result: u32 = 0;

    var line_split = std.mem.tokenizeScalar(u8, contents, '\n');
    while (line_split.next()) |line| {
        var levels: [10]i32 = undefined;
        var len: usize = 0;

        var space_split = std.mem.tokenizeScalar(u8, line, ' ');
        while (space_split.next()) |number| : (len += 1) {
            levels[len] = try std.fmt.parseInt(i32, number, 10);
        }

        skip: for (0..len) |skip_i| {
            var last_level: ?i32 = null;
            var last_diff: i64 = 0;

            for (0.., levels[0..len]) |i, level| {
                if (i == skip_i) continue;

                if (last_level == null) {
                    last_level = level;
                    continue;
                }

                const diff = level - last_level.?;
                if (diff * last_diff < 0 or diff == 0 or @abs(diff) > 3) continue :skip;

                last_level = level;
                last_diff = diff;
            }

            result += 1;
            break;
        }
    }

    return result;
}

test "d02p1" {
    try std.testing.expect(2 == try p1(@embedFile("examples/d02e1.txt")));
}

test "d02p2" {
    try std.testing.expect(4 == try p2(@embedFile("examples/d02e1.txt")));
}

pub fn main() !void {
    const contents = try utils.get_contents(2);
    std.debug.print("{d}\n", .{try p1(contents)});
    std.debug.print("{d}\n", .{try p2(contents)});
}
