use std::{collections::HashSet, hash::RandomState};

advent_of_code::solution!(4);

type BasicCoord = (usize, usize);

fn parse_input(input: &str) -> (HashSet<BasicCoord>, BasicCoord) {
    let mut rolls = HashSet::new();

    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.trim().lines().enumerate() {
        height = y + 1;
        width = line.len();
        for (x, char) in line.chars().enumerate() {
            match char {
                '@' => {
                    rolls.insert((x, y));
                }
                '.' => {}
                _ => panic!(),
            }
        }
    }
    (rolls, (width, height))
}

fn get_neighbors((x, y): BasicCoord, (width, height): BasicCoord) -> Vec<BasicCoord> {
    let left_available = x > 0;
    let right_available = x < width - 1;
    let top_available = y > 0;
    let bottom_available = y < height - 1;

    let mut neighbors = Vec::new();
    if left_available {
        neighbors.push((x - 1, y));
    }
    if right_available {
        neighbors.push((x + 1, y));
    }
    if top_available {
        neighbors.push((x, y - 1));
    }
    if bottom_available {
        neighbors.push((x, y + 1));
    }

    if left_available && top_available {
        neighbors.push((x - 1, y - 1));
    }
    if top_available && right_available {
        neighbors.push((x + 1, y - 1));
    }
    if right_available && bottom_available {
        neighbors.push((x + 1, y + 1));
    }
    if bottom_available && left_available {
        neighbors.push((x - 1, y + 1));
    }

    neighbors
}

fn roll_accessible(coord: BasicCoord, rolls: &HashSet<BasicCoord>, dimensions: BasicCoord) -> bool {
    get_neighbors(coord, dimensions)
        .iter()
        .filter(|neighbor_coord| rolls.contains(neighbor_coord))
        .count()
        < 4
}

fn find_accessible_rolls(
    rolls: &HashSet<BasicCoord>,
    dimensions: BasicCoord,
) -> impl Iterator<Item = BasicCoord> {
    rolls
        .iter()
        .filter(move |coord| roll_accessible(**coord, rolls, dimensions))
        .map(|coord| *coord)
}

fn find_rolls_to_remove(
    rolls: &HashSet<BasicCoord>,
    dimensions: BasicCoord,
    removed_last_round: &Option<Vec<BasicCoord>>,
) -> Vec<BasicCoord> {
    match removed_last_round {
        Some(last_removed) => {
            let neighbors_to_check: HashSet<_, RandomState> = HashSet::from_iter(
                last_removed
                    .iter()
                    .flat_map(|coord| get_neighbors(*coord, dimensions))
                    .filter(|coord| rolls.contains(coord)),
            );

            neighbors_to_check
                .iter()
                .filter(|coord| roll_accessible(**coord, rolls, dimensions))
                .map(|coord| *coord)
                .collect()
        }
        None => find_accessible_rolls(rolls, dimensions).collect(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rolls, dimensions) = parse_input(input);

    Some(find_accessible_rolls(&rolls, dimensions).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut rolls, dimensions) = parse_input(input);

    let mut total_removed = 0;
    let mut removed_last_round: Option<Vec<_>> = None;

    loop {
        let removed_this_round = find_rolls_to_remove(&rolls, dimensions, &removed_last_round);
        for coord in &removed_this_round {
            rolls.remove(coord);
        }

        let removed_count = removed_this_round.len();
        total_removed += removed_count;

        removed_last_round = Some(removed_this_round);

        if removed_count == 0 {
            break;
        }
    }

    Some(total_removed as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
