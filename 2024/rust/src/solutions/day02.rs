use std::cmp::Ordering::{Equal, Greater, Less};

pub struct Day02;

impl super::Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let mut result = 0;

        'lines: for line in input.lines() {
            let (mut dec, mut inc) = (true, true);
            let mut last_level: Option<u32> = None;

            for level in line.split(' ').map(|numbers| numbers.parse().unwrap()) {
                if let Some(last) = last_level {
                    match last.cmp(&level) {
                        Greater => inc = false,
                        Less => dec = false,
                        Equal => (),
                    }

                    let diff = last.abs_diff(level);
                    if diff == 0 || diff > 3 {
                        continue 'lines;
                    }
                }
                last_level = Some(level);
            }

            if inc || dec {
                result += 1;
            }
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut result = 0;

        for levels in input.lines().map(|line| {
            line.split(' ')
                .map(|numbers| numbers.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        }) {
            'skip: for skip_i in 0..levels.len() {
                let mut last_level = None;
                let mut last_diff = 0;

                for (i, level) in levels.iter().enumerate() {
                    if i == skip_i {
                        continue;
                    }

                    if last_level.is_none() {
                        last_level = Some(level);
                        continue;
                    }

                    let diff = level - last_level.unwrap();
                    if diff * last_diff < 0 || diff == 0 || diff.abs() > 3 {
                        continue 'skip;
                    }

                    last_diff = diff;
                    last_level = Some(level);
                }

                result += 1;
                break;
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 2;

    #[test]
    fn part1() {
        assert_eq!(
            "2",
            get_solution(1, DAY, include_str!("examples/d02e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "4",
            get_solution(2, DAY, include_str!("examples/d02e1.txt"))
        );
    }
}
