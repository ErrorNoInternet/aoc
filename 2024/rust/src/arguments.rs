use argh::FromArgs;

/// Advent of Code solutions
#[derive(FromArgs)]
pub struct Args {
    /// day to solve for
    #[argh(option, short = 'd')]
    pub day: usize,

    /// file to read puzzle input from
    #[argh(option, short = 'i')]
    pub input: Option<String>,
}
