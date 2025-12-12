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

fn find_range_conflicts(
    range: &RangeInclusive<u64>,
    ranges: &Vec<RangeInclusive<u64>>,
) -> Vec<usize> {
    ranges
        .iter()
        .enumerate()
        .filter_map(|(idx, cmp_range)| {
            let lower_above_min = range.start() >= cmp_range.start();
            let upper_below_max = range.end() <= cmp_range.end();

            if (lower_above_min && upper_below_max) // range fully covered by cmp_range
                || (!lower_above_min && !upper_below_max) // cmp_range fully covered by range
                || (lower_above_min && range.start() <= cmp_range.end()) // range extends upper end of cmp_range
                || (upper_below_max && range.end() >= cmp_range.start())
            // range extends lower end of cmp_range
            {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh_ranges, _) = parse_input(input);

    let mut built_ranges = vec![];

    for range in fresh_ranges {
        let range_conflicts = find_range_conflicts(&range, &built_ranges);
        if range_conflicts.is_empty() {
            built_ranges.push(range);
        } else {
            let all_conflicting_indices = range_conflicts.iter().sorted().rev().collect_vec();

            let start_value = all_conflicting_indices
                .iter()
                .map(|idx| *(built_ranges[**idx].start()))
                .chain([*range.start()])
                .min()
                .unwrap();
            let end_value = all_conflicting_indices
                .iter()
                .map(|idx| *(built_ranges[**idx].end()))
                .chain([*range.end()])
                .max()
                .unwrap();

            for (idx, range) in all_conflicting_indices
                .iter()
                .map(|idx| (**idx, built_ranges[**idx].clone()))
                .collect_vec()
            {
                assert_eq!(built_ranges.remove(idx), range);
            }

            let new_range = start_value..=end_value;

            built_ranges.push(new_range);
        }
    }

    Some(
        built_ranges
            .iter()
            .map(|range| 1 + range.end() - range.start())
            .sum(),
    )
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
        assert_eq!(result, Some(14));
    }
}
