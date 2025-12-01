advent_of_code::solution!(1);

#[derive(Debug)]
enum Move {
    Left(u64),
    Right(u64),
}

impl Move {
    fn from_str(str: &str) -> Self {
        let (dir, amt) = str.split_at(1);
        let amt = amt.to_string().parse().unwrap();

        match dir {
            "L" => Move::Left(amt),
            "R" => Move::Right(amt),
            _ => panic!(),
        }
    }
}

const fn do_move(dial_pos: u8, mov: &Move) -> u8 {
    (dial_pos as isize
        + match mov {
            Move::Left(v) => -(*v as isize),
            Move::Right(v) => *v as isize,
        })
    .rem_euclid(100) as u8
}

pub fn part_one(input: &str) -> Option<u64> {
    let moves = input.trim().lines().map(|line| Move::from_str(line.trim()));
    let mut dial_pos = 50;
    let mut zero_count = 0;
    for mov in moves {
        let new_pos = do_move(dial_pos, &mov);
        // println!("{:?}: {} -> {}", mov, dial_pos, new_pos);
        dial_pos = new_pos;
        if dial_pos == 0 {
            zero_count += 1;
        }
    }
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        assert_eq!(do_move(11, &Move::Right(8)), 19);
        assert_eq!(do_move(19, &Move::Left(19)), 0);
        assert_eq!(do_move(0, &Move::Left(1)), 99);
        assert_eq!(do_move(99, &Move::Right(1)), 0);
        assert_eq!(do_move(5, &Move::Left(10)), 95);
        assert_eq!(do_move(95, &Move::Right(5)), 0);
    }

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
