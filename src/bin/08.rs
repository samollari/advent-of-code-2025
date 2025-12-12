use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

type Coord3D = (usize, usize, usize);

fn parse_input(input: &str) -> Vec<Coord3D> {
    input
        .lines()
        .map(|line| {
            let mut coord_parts = line.trim().splitn(3, ',');
            let x_part = coord_parts.next().unwrap();
            let y_part = coord_parts.next().unwrap();
            let z_part = coord_parts.next().unwrap();
            assert_eq!(coord_parts.next(), None);

            (
                x_part.parse().unwrap(),
                y_part.parse().unwrap(),
                z_part.parse().unwrap(),
            )
        })
        .collect()
}

fn get_sorted_connections(
    coords: &Vec<Coord3D>,
) -> impl Iterator<Item = ((Coord3D, Coord3D), usize)> {
    coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let (a_x, a_y, a_z) = a;
            let (b_x, b_y, b_z) = b;

            let dist = ((*a_x as isize - *b_x as isize).pow(2)
                + (*a_y as isize - *b_y as isize).pow(2)
                + (*a_z as isize - *b_z as isize).pow(2))
            .isqrt() as usize;

            ((*a, *b), dist)
        })
        .sorted_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b))
}

fn connect_to_circuits(
    (a_coord, b_coord): &(Coord3D, Coord3D),
    circuits: &mut Vec<HashSet<Coord3D>>,
) {
    let existing_a_circuit = circuits.iter().find_position(|set| set.contains(a_coord));
    let existing_b_circuit = circuits.iter().find_position(|set| set.contains(b_coord));

    let both_coords = [*a_coord, *b_coord];

    match (existing_a_circuit, existing_b_circuit) {
        (None, None) => circuits.push(HashSet::from(both_coords)),
        (None, Some((idx, _))) | (Some((idx, _)), None) => circuits[idx].extend(both_coords),
        (Some((a_idx, _)), Some((b_idx, b_set))) => {
            if a_idx != b_idx {
                let b_set = b_set.clone();
                circuits[a_idx].extend(b_set);
                circuits[a_idx].extend(both_coords);
                circuits.swap_remove(b_idx);
            }
        }
    }
}

fn get_pt1_ans(input: &str, num_connections: usize) -> Option<u64> {
    let coords = parse_input(input);
    let connections = get_sorted_connections(&coords).take(num_connections);

    let mut circuits: Vec<HashSet<Coord3D>> = vec![];

    for (points, _) in connections {
        connect_to_circuits(&points, &mut circuits);
    }

    Some(
        circuits
            .iter()
            .map(|set| set.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>() as u64,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    get_pt1_ans(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse_input(input);
    let connections = get_sorted_connections(&coords);

    let mut circuits: Vec<HashSet<Coord3D>> = vec![];
    let mut last_connected_coords: Option<(Coord3D, Coord3D)> = None;

    for (points, _) in connections {
        connect_to_circuits(&points, &mut circuits);

        if circuits.len() == 1 && circuits[0].len() == coords.len() {
            last_connected_coords = Some(points);
            break;
        }
    }

    last_connected_coords.map(|((a_x, _, _), (b_x, _, _))| (a_x * b_x) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = get_pt1_ans(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
