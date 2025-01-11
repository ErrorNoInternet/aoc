const std = @import("std");

pub fn read_path(path: []const u8) ![]u8 {
    return try std.fs.cwd().readFileAlloc(std.heap.page_allocator, path, std.math.maxInt(usize));
}

pub fn read_day(day: u8) ![]u8 {
    if (day < 1 or day > 25) {
        return error.InvalidDay;
    }

    var file_name: [14]u8 = undefined;
    _ = try std.fmt.bufPrint(&file_name, "inputs/d{d:0>2}.txt", .{day});

    return read_path(&file_name);
}

pub fn get_contents(day: u8) ![]u8 {
    var args = std.process.args();
    _ = args.skip();
    return try if (args.next()) |arg| read_path(arg) else read_day(day);
}
