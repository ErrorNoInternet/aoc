pub struct Day04;

impl super::Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let line_len = TryInto::<isize>::try_into(input.find('\n').unwrap()).unwrap();
        let input = input.as_bytes();

        let search = |i: isize, offset: isize| -> u32 {
            if let Ok(end) = TryInto::<usize>::try_into(i + offset * 3)
                && end < input.len()
            {
                #[allow(clippy::cast_sign_loss)]
                u32::from(
                    input[(i + offset) as usize] == b'M'
                        && input[(i + offset * 2) as usize] == b'A'
                        && input[end] == b'S',
                )
            } else {
                0
            }
        };

        input
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == b'X')
            .fold(0, |acc, (i, _)| {
                let i = TryInto::try_into(i).unwrap();
                acc + search(i, 1)
                    + search(i, -1)
                    + search(i, -line_len)
                    + search(i, line_len)
                    + search(i, -line_len - 1)
                    + search(i, line_len + 1)
                    + search(i, -line_len - 2)
                    + search(i, line_len + 2)
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let line_len = input.find('\n').unwrap();
        let input = input.as_bytes();

        input
            .iter()
            .enumerate()
            .filter(|&(i, &c)| i > line_len && i < input.len() - line_len && c == b'A')
            .fold(0, |acc, (i, _)| {
                acc + u32::from(
                    input[i - line_len] ^ input[i + line_len] == 30
                        && input[i - line_len - 2] ^ input[i + line_len + 2] == 30,
                )
            })
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 4;

    #[test]
    fn part1() {
        assert_eq!(
            "18",
            get_solution(1, DAY, include_str!("examples/d04e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "9",
            get_solution(2, DAY, include_str!("examples/d04e1.txt"))
        );
    }
}
