pub struct Day01;

impl super::Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let mut left_nums = [0u32; 1000];
        let mut right_nums = [0u32; 1000];

        for (i, line) in input.lines().enumerate() {
            let mut space_split = line.split(' ');
            left_nums[i] = space_split.next().unwrap().parse().unwrap();
            right_nums[i] = space_split.next_back().unwrap().parse().unwrap();
        }

        left_nums.sort_unstable();
        right_nums.sort_unstable();

        left_nums
            .iter()
            .zip(right_nums)
            .fold(0, |acc, (l, r)| acc + l.abs_diff(r))
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut left_nums = [0u32; 1000];
        let mut right_nums = [0u32; 1000];

        for (i, line) in input.lines().enumerate() {
            let mut space_split = line.split(' ');
            left_nums[i] = space_split.next().unwrap().parse().unwrap();
            right_nums[i] = space_split.next_back().unwrap().parse().unwrap();
        }

        left_nums
            .iter()
            .fold(0, |acc: usize, &l| {
                acc + l as usize * right_nums.iter().filter(|&&r| r == l).count()
            })
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 1;

    #[test]
    fn part1() {
        assert_eq!(
            "11",
            get_solution(1, DAY, include_str!("examples/d01e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "31",
            get_solution(2, DAY, include_str!("examples/d01e1.txt"))
        );
    }
}
