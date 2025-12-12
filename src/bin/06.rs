use itertools::Itertools;

advent_of_code::solution!(6);

enum Problem {
    Addition(Vec<usize>),
    Multiplication(Vec<usize>),
}

fn parse_input(input: &str) -> Vec<Problem> {
    let mut parts_by_line = input
        .lines()
        .map(|line| line.trim().split_whitespace().collect_vec())
        .collect_vec();
    let problem_type_line_parts = parts_by_line.pop();

    problem_type_line_parts
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, part)| {
            let operands = get_operands(&parts_by_line, i);
            match *part {
                "+" => Problem::Addition(operands),
                "*" => Problem::Multiplication(operands),
                _ => panic!(),
            }
        })
        .collect()
}

fn get_operands(parts_by_line: &Vec<Vec<&str>>, i: usize) -> Vec<usize> {
    parts_by_line
        .iter()
        .map(|line_parts| line_parts.get(i).unwrap().parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_input(input);

    Some(
        problems
            .iter()
            .map(|problem| match problem {
                Problem::Addition(items) => items.iter().sum::<usize>(),
                Problem::Multiplication(items) => items.iter().product(),
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
