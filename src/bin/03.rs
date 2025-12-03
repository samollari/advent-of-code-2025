advent_of_code::solution!(3);

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

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse_input(input);

    Some(
        banks
            .iter()
            .map(|bank| {
                let mut first_digit = 0;
                let mut second_digit = 0;
                for digit in bank {
                    if second_digit > first_digit {
                        first_digit = second_digit;
                        second_digit = *digit;
                    } else if *digit > second_digit {
                        second_digit = *digit;
                    }
                }
                assert!(first_digit < 10);
                assert!(second_digit < 10);

                (first_digit * 10 + second_digit) as u64
            })
            .sum(),
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
