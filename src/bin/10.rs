use std::ops::Add;

use itertools::Itertools;

advent_of_code::solution!(10);

struct Machine {
    desired_light_state: u16,
    buttons: Vec<u16>,
    joltage_requirements: Vec<u16>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let mut parts = line.trim().split(' ');

        let lights_part = parts.next();
        let (button_parts, joltage_parts) =
            parts.partition::<Vec<_>, _>(|part| part.starts_with('('));

        assert!(button_parts.len() > 0);
        assert!(joltage_parts.len() == 1);
        let joltage_part = joltage_parts.first();

        let lights_str = lights_part.unwrap();
        let desired_light_state = lights_str[1..lights_str.len() - 1]
            .chars()
            .map(|char| match char {
                '#' => 1,
                '.' => 0,
                _ => panic!(),
            })
            .enumerate()
            .fold(0, |acc, (i, x)| acc | (x << i));

        let buttons = button_parts
            .iter()
            .map(|button_def| {
                button_def[1..button_def.len() - 1]
                    .split(',')
                    .fold(0, |acc, i| acc | 1 << i.parse::<u8>().unwrap())
            })
            .collect();

        let joltage_str = joltage_part.unwrap();
        let joltage_requirements = joltage_str[1..joltage_str.len() - 1]
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        Self {
            desired_light_state,
            buttons,
            joltage_requirements,
        }
    }

    fn find_min_presses_to_configure(self: &Self) -> Option<u64> {
        (0..self.buttons.len())
            .powerset()
            .find(|buttons_to_try| self.buttons_satisfy_lights(buttons_to_try))
            .map(|satisfying_buttons| satisfying_buttons.len() as u64)
    }

    fn buttons_satisfy_lights(self: &Self, buttons_to_press: &Vec<usize>) -> bool {
        let mut lights_val = 0;

        for button_idx in buttons_to_press {
            match self.buttons.get(*button_idx) {
                Some(button_affects) => lights_val ^= button_affects,
                None => return false,
            }
        }

        lights_val == self.desired_light_state
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.trim().lines().map(Machine::from_line).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);

    machines
        .iter()
        .map(Machine::find_min_presses_to_configure)
        .fold_options(0, Add::add)
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
