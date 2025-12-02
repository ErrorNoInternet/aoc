pub struct Day02;

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|range| {
            let (left, right) = range.trim().split_once("-").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

fn sum(ranges: &[(u64, u64)], digits: &[u32], lengths: &[u32]) -> u64 {
    let mut result = 0;

    for (d, l) in digits.iter().zip(lengths) {
        let repetitions = d / l;
        let power = 10_u64.pow(*l);
        let mut step = 0;
        for _ in 0..repetitions {
            step = step * power + 1;
        }
        let invalid_start = step * (power / 10);
        let invalid_end = step * (power - 1);

        for &(start, end) in ranges {
            let lower = start.next_multiple_of(step).max(invalid_start);
            let upper = end.min(invalid_end);
            if lower <= upper {
                let n = (upper - lower) / step;
                let m = n * (n + 1) / 2;
                result += lower * (n + 1) + step * m;
            }
        }
    }

    result
}

impl super::Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        sum(&parse(input), &[2, 4, 6, 8, 10], &[1, 2, 3, 4, 5]).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let result = sum(&parse(input), &[2, 4, 6, 8, 10], &[1, 2, 3, 4, 5])
            + sum(&parse(input), &[3, 5, 6, 7, 9, 10], &[1, 1, 2, 1, 3, 2])
            - sum(&parse(input), &[6, 10], &[1, 1]);
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
            "1227775554",
            get_solution(1, DAY, include_str!("examples/d02e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "4174379265",
            get_solution(2, DAY, include_str!("examples/d02e1.txt"))
        );
    }
}
