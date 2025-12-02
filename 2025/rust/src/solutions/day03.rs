pub struct Day03;

impl super::Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let mut total: u64 = 0;

        for line in input.lines() {
            let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();

            let n = digits.len();
            let mut suffix = vec![0u8; n];
            let mut max = 0u8;
            for i in (0..n).rev() {
                suffix[i] = max;
                if digits[i] > max {
                    max = digits[i];
                }
            }

            let mut best = 0u8;
            for i in 0..n - 1 {
                let joltage = digits[i] * 10 + suffix[i];
                if joltage > best {
                    best = joltage;
                }
            }
            total += best as u64;
        }

        total.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut total = 0;

        for line in input.lines() {
            let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
            let mut required = 12;
            let mut result = Vec::with_capacity(required);

            let mut i = 0;
            while required > 0 {
                let mut best = 0;
                let mut pos = i;
                let end = digits.len() - required;
                let mut j = i;
                while j <= end {
                    if digits[j] > best {
                        best = digits[j];
                        pos = j;
                        if best == 9 {
                            break;
                        }
                    }
                    j += 1;
                }
                result.push(best);
                required -= 1;
                i = pos + 1;
            }

            let mut joltage = 0;
            for digit in result {
                joltage = joltage * 10 + digit as u128;
            }
            total += joltage;
        }

        total.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 3;

    #[test]
    fn part1() {
        assert_eq!(
            "357",
            get_solution(1, DAY, include_str!("examples/d03e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "3121910778619",
            get_solution(2, DAY, include_str!("examples/d03e1.txt"))
        );
    }
}
