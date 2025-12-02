advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let range_extents = input
        .trim()
        .split(',')
        .map(|part| part.split_once('-').unwrap())
        .map(|(left, right)| {
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        });

    Some(
        range_extents
            .flat_map(|(start, end)| {
                (start..=end).filter(|val| {
                    let val = val.to_string();
                    let (left, right) = val.split_at(val.len() / 2);
                    *left == *right
                })
            })
            .sum::<usize>() as u64,
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
