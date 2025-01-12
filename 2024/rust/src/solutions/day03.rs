pub struct Day03;

impl super::Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        regex::Regex::new(r"mul\(\d+,\d+\)")
            .unwrap()
            .find_iter(input)
            .map(|expr| expr.as_str()[4..expr.len() - 1].split_once(",").unwrap())
            .fold(0, |acc, split| {
                acc + split.0.parse::<u32>().unwrap() * split.1.parse::<u32>().unwrap()
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut result = 0;

        let mut should_do = true;
        for expr in regex::Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)")
            .unwrap()
            .find_iter(input)
            .map(|matched| matched.as_str())
        {
            match &expr[0..3] {
                "don" => should_do = false,
                "do(" => should_do = true,
                "mul" => {
                    if should_do {
                        let split = expr[4..expr.len() - 1].split_once(",").unwrap();
                        result += split.0.parse::<u32>().unwrap() * split.1.parse::<u32>().unwrap();
                    }
                }
                _ => unreachable!(),
            }
        }

        result.to_string()
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
