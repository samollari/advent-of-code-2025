use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(5);

fn parse_input(
    input: &str,
) -> (
    impl Iterator<Item = RangeInclusive<u64>>,
    impl Iterator<Item = u64>,
) {
    let (range_part, ingredients_part) = input.trim().split_once("\n\n").unwrap();
    let ranges = range_part.lines().map(|line| {
        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        start..=end
    });
    let ingredients = ingredients_part.lines().map(|line| line.parse().unwrap());
    (ranges, ingredients)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ranges, ingredients) = parse_input(input);

    let fresh_ranges = fresh_ranges.collect_vec();

    Some(
        ingredients
            .filter(|ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
            .count() as u64,
    )
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
