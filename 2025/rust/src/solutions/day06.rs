pub struct Day06;

impl super::Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let lines: Vec<_> = input.lines().collect();
        (0..lines.first().unwrap().len())
            .collect::<Vec<_>>()
            .chunk_by(|&a, &b| {
                let empty_a = lines
                    .iter()
                    .all(|l| l.as_bytes().get(a).unwrap_or(&b' ') == &b' ');
                let empty_b = lines
                    .iter()
                    .all(|l| l.as_bytes().get(b).unwrap_or(&b' ') == &b' ');
                empty_a == empty_b
            })
            .filter(|chunk| {
                !chunk.is_empty()
                    && lines
                        .iter()
                        .any(|l| l.as_bytes().get(chunk[0]).unwrap_or(&b' ') != &b' ')
            })
            .map(|chunk| {
                let start = chunk[0];
                let end = chunk[chunk.len() - 1] + 1;
                let is_add = lines
                    .last()
                    .unwrap()
                    .get(start..end)
                    .unwrap_or("")
                    .contains('+');

                let nums = (0..lines.len() - 1)
                    .map(|row| lines[row].get(start..end).unwrap_or("").trim())
                    .filter_map(|s| s.parse::<u64>().ok());

                if is_add {
                    nums.sum::<u64>()
                } else {
                    nums.product()
                }
            })
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let lines: Vec<_> = input.lines().collect();
        (0..lines.first().unwrap().len())
            .collect::<Vec<_>>()
            .chunk_by(|&a, &b| {
                let empty_a = lines
                    .iter()
                    .all(|l| l.as_bytes().get(a).unwrap_or(&b' ') == &b' ');
                let empty_b = lines
                    .iter()
                    .all(|l| l.as_bytes().get(b).unwrap_or(&b' ') == &b' ');
                empty_a == empty_b
            })
            .filter(|chunk| {
                !chunk.is_empty()
                    && lines
                        .iter()
                        .any(|l| l.as_bytes().get(chunk[0]).unwrap_or(&b' ') != &b' ')
            })
            .map(|chunk| {
                let start = chunk[0];
                let end = chunk[chunk.len() - 1] + 1;
                let is_add = lines
                    .last()
                    .unwrap()
                    .get(start..end)
                    .unwrap_or("")
                    .contains('+');

                let nums = chunk.iter().rev().filter_map(|&c| {
                    let s: String = (0..lines.len() - 1)
                        .map(|r| lines[r].as_bytes().get(c).unwrap_or(&b' '))
                        .filter(|c| !c.is_ascii_whitespace())
                        .map(|c| *c as char)
                        .collect();
                    s.parse::<u64>().ok()
                });

                if is_add {
                    nums.sum::<u64>()
                } else {
                    nums.product()
                }
            })
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 6;

    #[test]
    fn part1() {
        assert_eq!(
            "4277556",
            get_solution(1, DAY, include_str!("examples/d06e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "3263827",
            get_solution(2, DAY, include_str!("examples/d06e1.txt"))
        );
    }
}
