pub struct Day05;

fn merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for &(start, end) in ranges {
        if let Some((_, last_end)) = merged.last_mut()
            && start <= *last_end
        {
            *last_end = (*last_end).max(end);
            continue;
        }
        merged.push((start, end));
    }
    merged
}

impl super::Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let sections = input.split_once("\n\n").unwrap();

        let mut fresh: Vec<(u64, u64)> = sections
            .0
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .map(|split| (split.0.parse().unwrap(), split.1.parse().unwrap()))
            .collect();
        fresh.sort_by_key(|&(start, _)| start);
        let merged = merge_ranges(&fresh);

        sections
            .1
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .fold(0, |acc, available| {
                acc + u64::from(
                    merged
                        .iter()
                        .any(|range| available >= range.0 && available <= range.1),
                )
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut fresh: Vec<(u64, u64)> = input
            .split_once("\n\n")
            .unwrap()
            .0
            .lines()
            .map(|line| line.split_once('-').unwrap())
            .map(|split| (split.0.parse().unwrap(), split.1.parse().unwrap()))
            .collect();
        fresh.sort_by_key(|&(start, _)| start);

        merge_ranges(&fresh)
            .iter()
            .fold(0, |acc, range| acc + (range.1 - range.0) + 1)
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 5;

    #[test]
    fn part1() {
        assert_eq!(
            "3",
            get_solution(1, DAY, include_str!("examples/d05e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "14",
            get_solution(2, DAY, include_str!("examples/d05e1.txt"))
        );
    }
}
