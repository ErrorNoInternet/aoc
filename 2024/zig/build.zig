const std = @import("std");

pub fn build(b: *std.Build) void {
    const regex = b.dependency("regex", .{});

    const target = b.standardTargetOptions(.{});
    const mode = b.standardOptimizeOption(.{});

    const install_all = b.step("install_all", "Install all days");
    const run_all = b.step("run", "Run all days");

    var day: u32 = 1;
    while (day <= 3) : (day += 1) {
        const dayString = b.fmt("d{:0>2}", .{day});
        const zigFile = b.fmt("src/{s}.zig", .{dayString});

        const exe = b.addExecutable(.{
            .name = dayString,
            .root_source_file = b.path(zigFile),
            .target = target,
            .optimize = mode,
        });
        exe.root_module.addImport("regex", regex.module("regex"));
        const install_cmd = b.addInstallArtifact(exe, .{});

        const build_test = b.addTest(.{
            .root_source_file = b.path(zigFile),
            .target = target,
            .optimize = mode,
        });
        build_test.root_module.addImport("regex", regex.module("regex"));
        const run_test = b.addRunArtifact(build_test);

        {
            const step_key = b.fmt("install_{s}", .{dayString});
            const step_desc = b.fmt("Install {s}", .{dayString});
            const install_step = b.step(step_key, step_desc);
            install_step.dependOn(&install_cmd.step);
            install_all.dependOn(&install_cmd.step);
        }

        {
            const step_key = b.fmt("test_{s}", .{dayString});
            const step_desc = b.fmt("Run tests in {s}", .{zigFile});
            const step = b.step(step_key, step_desc);
            step.dependOn(&run_test.step);
        }

        const run_cmd = b.addRunArtifact(exe);
        if (b.args) |args| {
            run_cmd.addArgs(args);
        }

        const run_desc = b.fmt("Run {s}", .{dayString});
        const run_step = b.step(dayString, run_desc);
        run_step.dependOn(&run_cmd.step);
        run_all.dependOn(&run_cmd.step);
    }

    const test_all = b.step("test", "Run all tests");
    const all_tests = b.addTest(.{
        .root_source_file = b.path("src/tests.zig"),
        .target = target,
        .optimize = mode,
    });
    all_tests.root_module.addImport("regex", regex.module("regex"));
    const run_all_tests = b.addRunArtifact(all_tests);
    test_all.dependOn(&run_all_tests.step);
}
