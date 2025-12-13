use itertools::Itertools;

advent_of_code::solution!(9);

type Coord = (usize, usize);

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_input(input)
        .iter()
        .tuple_combinations()
        .map(|((a_x, a_y), (b_x, b_y))| {
            let w = a_x.abs_diff(*b_x) + 1;
            let h = a_y.abs_diff(*b_y) + 1;
            (w * h) as u64
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
