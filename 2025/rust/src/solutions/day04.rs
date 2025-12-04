pub struct Day04;

fn get_accessible(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, c)| **c == '@') {
            let mut rolls = 0;
            for offset in [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ] {
                let adjacent = (
                    (x as isize + offset.0) as usize,
                    (y as isize + offset.1) as usize,
                );
                if adjacent.1 < map.len()
                    && adjacent.0 < map[0].len()
                    && map[adjacent.1][adjacent.0] == '@'
                {
                    rolls += 1;
                }
            }
            if rolls < 4 {
                accessible.push((x, y));
            }
        }
    }

    accessible
}

impl super::Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        get_accessible(&map).len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut removed = 0;

        loop {
            let accessible = get_accessible(&map);
            if accessible.is_empty() {
                break;
            }

            for roll in accessible {
                map[roll.1][roll.0] = '.';
                removed += 1;
            }
        }

        removed.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::get_solution;

    const DAY: usize = 4;

    #[test]
    fn part1() {
        assert_eq!(
            "13",
            get_solution(1, DAY, include_str!("examples/d04e1.txt"))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            "43",
            get_solution(2, DAY, include_str!("examples/d04e1.txt"))
        );
    }
}
