mod arguments;
mod solutions;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: arguments::Args = argh::from_env();

    if args.day < 1 || args.day > 25 {
        eprintln!("day must be between 1 and 25!");
        return ExitCode::FAILURE;
    }

    let path = args
        .input
        .unwrap_or(format!("inputs/d{:0>2}.txt", args.day));
    let input = match std::fs::read_to_string(&path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("couldn't read {path:?}: {error}");
            return ExitCode::FAILURE;
        }
    };

    solutions::run(args.day, &input);
    ExitCode::SUCCESS
}
