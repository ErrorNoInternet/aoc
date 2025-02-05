pub struct Day03;

impl super::Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        regex::Regex::new(r"mul\((\d+),(\d+)\)")
            .unwrap()
            .captures_iter(input)
            .fold(0, |acc, capture| {
                acc + capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap()
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut should_do = true;
        regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
            .unwrap()
            .captures_iter(input)
            .fold(0, |acc, capture| {
                acc + match &capture[0] {
                    "do()" => {
                        should_do = true;
                        0
                    }
                    "don't()" => {
                        should_do = false;
                        0
                    }
                    _ => {
                        if should_do {
                            capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap()
                        } else {
                            0
                        }
                    }
                }
            })
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 3;

    #[test]
    fn part1() {
        assert_eq!(
            "161",
            get_solution(1, DAY, include_str!("examples/d03e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "48",
            get_solution(2, DAY, include_str!("examples/d03e2.txt"))
        );
    }
}
