pub struct Day01;

impl super::Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let mut dial = 50;
        let mut crosses = 0;

        for line in input.lines() {
            let (dir, dist) = line.split_at(1);
            let dist: i32 = dist.parse().unwrap();

            let delta = if dir == "L" { -dist } else { dist };
            dial = (dial + delta).rem_euclid(100);
            crosses += i32::from(dial == 0);
        }

        crosses.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut dial = 50;
        let mut crosses = 0;

        for line in input.lines() {
            let (dir, dist) = line.split_at(1);
            let dist: i32 = dist.parse().unwrap();

            let delta = if dir == "L" { -dist } else { dist };
            crosses += if delta < 0 {
                (100 - dial - delta) / 100 - i32::from(dial == 0)
            } else {
                (dial + delta) / 100
            };
            dial = (dial + delta).rem_euclid(100);
        }

        crosses.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 1;

    #[test]
    fn part1() {
        assert_eq!(
            "3",
            get_solution(1, DAY, include_str!("examples/d01e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "6",
            get_solution(2, DAY, include_str!("examples/d01e1.txt"))
        );
    }
}
