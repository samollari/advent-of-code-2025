advent_of_code::solution!(3);

use std::collections::VecDeque;

use itertools::{self, Itertools};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_max_joltage<const BATTERIES: usize>(bank: &Vec<u8>) -> u64 {
    let mut digits = VecDeque::from([0u8; BATTERIES]);

    for digit in bank {
        assert!(*digit < 10);
        let found = digits
            .iter()
            .chain(vec![digit])
            .tuple_windows()
            .find_position(|(high, low)| low > high);
        match found {
            Some((drop_index, _)) => {
                digits.remove(drop_index).unwrap();
                digits.push_back(*digit);
            }
            None => {}
        }
    }

    digits.iter().fold(0, |acc, x| acc * 10 + *x as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_input(input).iter().map(find_max_joltage::<2>).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input(input).iter().map(find_max_joltage::<12>).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
