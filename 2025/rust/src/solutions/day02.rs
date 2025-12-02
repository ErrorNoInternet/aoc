pub struct Day02;

fn is_valid_part1_id(id: i64) -> bool {
    let len = (id as f64).log10().ceil() as i32;
    if len % 2 != 0 {
        return true;
    }
    let divisor = 10f32.powi(len / 2) as i64;
    let half = id / divisor;
    id != (half * divisor + half)
}

fn is_valid_part2_id(id: i64) -> bool {
    let id = id.to_string();
    for subset_len in (1..=id.len() / 2).rev() {
        let repititions = id.len() / subset_len;
        if !id.len().is_multiple_of(repititions) {
            continue;
        }
        if id == id[0..subset_len].repeat(repititions) {
            return false;
        }
    }
    true
}

impl super::Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let mut sum = 0;
        for segment in input.split(",") {
            let (first, last) = segment.trim().split_once("-").unwrap();
            let (first_id, last_id) = (first.parse().unwrap(), last.parse().unwrap());
            for id in first_id..=last_id {
                if !is_valid_part1_id(id) {
                    sum += id;
                }
            }
        }
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut sum = 0;
        for segment in input.split(",") {
            let (first, last) = segment.trim().split_once("-").unwrap();
            let (first_id, last_id) = (first.parse().unwrap(), last.parse().unwrap());
            for id in first_id..=last_id {
                if !is_valid_part2_id(id) {
                    sum += id;
                }
            }
        }
        sum.to_string()
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
