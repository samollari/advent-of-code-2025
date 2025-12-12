advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let columns = first_line.len();
    let start_pos = first_line.find('S').unwrap();

    let rest_lines = lines.map(|line| {
        line.chars().enumerate().filter_map(|(i, c)| match c {
            '^' => Some(i),
            _ => None,
        })
    });

    let mut beams = vec![false; columns];
    beams[start_pos] = true;

    let mut split_count = 0;

    for line in rest_lines {
        for splitter_idx in line {
            if !beams[splitter_idx] {
                continue;
            }

            split_count += 1;

            beams[splitter_idx] = false;
            if splitter_idx > 0 {
                beams[splitter_idx - 1] = true;
            }
            if splitter_idx < columns - 1 {
                beams[splitter_idx + 1] = true;
            }
        }
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let columns = first_line.len();
    let start_pos = first_line.find('S').unwrap();

    let rest_lines = lines.map(|line| {
        line.chars().enumerate().filter_map(|(i, c)| match c {
            '^' => Some(i),
            _ => None,
        })
    });

    let mut timelines = vec![0; columns];

    for line in rest_lines.rev() {
        for splitter_idx in line {
            let left_timelines = if splitter_idx > 0 {
                let val = timelines[splitter_idx - 1];
                if val != 0 { val } else { 1 }
            } else {
                0
            };
            let right_timelines = if splitter_idx < columns - 1 {
                let val = timelines[splitter_idx + 1];
                if val != 0 { val } else { 1 }
            } else {
                0
            };
            timelines[splitter_idx] = left_timelines + right_timelines;
        }
    }

    Some(timelines[start_pos])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
