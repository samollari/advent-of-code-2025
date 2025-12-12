use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum ProblemType {
    Addition,
    Multiplication,
}

impl ProblemType {
    fn from(part: &str) -> Option<Self> {
        match part.trim() {
            "+" => Some(ProblemType::Addition),
            "*" => Some(ProblemType::Multiplication),
            _ => None,
        }
    }
}

type Problem = (ProblemType, Vec<usize>);

fn parse_problems_using_normal_math(input: &str) -> Vec<Problem> {
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
            (
                ProblemType::from(part).unwrap(),
                get_operands(&parts_by_line, i),
            )
        })
        .collect()
}

fn get_operands(parts_by_line: &Vec<Vec<&str>>, i: usize) -> Vec<usize> {
    parts_by_line
        .iter()
        .map(|line_parts| line_parts.get(i).unwrap().parse().unwrap())
        .collect()
}

fn calc_problems_sum(problems: Vec<Problem>) -> u64 {
    problems
        .iter()
        .map(|(problem_type, values)| match problem_type {
            ProblemType::Addition => values.iter().sum::<usize>(),
            ProblemType::Multiplication => values.iter().product(),
        })
        .sum::<usize>() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_problems_using_normal_math(input);

    Some(calc_problems_sum(problems))
}

fn parse_problem_strings(input: &str) -> Vec<(ProblemType, Vec<&str>)> {
    let lines = input.lines().collect_vec();
    let all_space_columns = lines
        .iter()
        .map(|line| line.chars().map(|char| char == ' ').collect_vec())
        .reduce(|acc, x| acc.iter().zip_eq(x).map(|(a, b)| *a && b).collect_vec())
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, is_space)| is_space.then_some(i as isize))
        .collect_vec();
    let problem_bounds = [-1isize]
        .iter()
        .chain(all_space_columns.iter())
        .chain([lines[0].len() as isize].iter())
        .tuple_windows()
        .map(|(lower, upper)| ((*lower + 1) as usize)..(*upper as usize))
        .collect_vec();
    problem_bounds
        .iter()
        .map(|range| {
            let mut parts = lines
                .iter()
                .map(|line| line.get(range.clone()).unwrap())
                .collect_vec();
            let op_type_part = parts.pop().unwrap();
            (ProblemType::from(op_type_part).unwrap(), parts)
        })
        .collect()
}

fn parse_problems_using_cephalopod_math(input: &str) -> Vec<Problem> {
    let problem_strings = parse_problem_strings(input);

    problem_strings
        .iter()
        .map(|(problem_type, operand_parts)| -> Problem {
            let mut number_assemblies = vec![vec![]; operand_parts[0].len()];

            for line in operand_parts {
                for (i, c) in line.chars().enumerate() {
                    number_assemblies.get_mut(i).unwrap().push(c);
                }
            }

            let operands = number_assemblies
                .iter()
                .map(|char_vec| char_vec.iter().collect::<String>().trim().parse().unwrap())
                .collect();

            (*problem_type, operands)
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_problems_using_cephalopod_math(input);

    Some(calc_problems_sum(problems))
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
        assert_eq!(result, Some(3263827));
    }
}
