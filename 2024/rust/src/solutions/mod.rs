seq_macro::seq!(N in 01..=04 {
    #(
        mod day~N;
    )*

    pub fn get_solver(day: usize) -> Box<dyn Solution> {
        match day {
            #(
                N => Box::new(day~N::Day~N),
            )*
            _ => todo!(),
        }
    }
});

pub trait Solution {
    fn part1(&self, _input: &str) -> String {
        "part 1 not yet implemented".to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "part 2 not yet implemented".to_string()
    }
}

pub fn get_solution(part: usize, day: usize, input: &str) -> String {
    let solver = get_solver(day);
    match part {
        1 => solver.part1(input),
        2 => solver.part2(input),
        _ => unreachable!(),
    }
}

pub fn run(day: usize, input: &str) {
    println!(
        "{}\n{}",
        get_solution(1, day, input),
        get_solution(2, day, input)
    );
}
