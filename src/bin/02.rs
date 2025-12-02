advent_of_code::solution!(2);

fn sum_invalid_ids<F>(input: &str, is_invalid: F) -> Option<u64>
where
    F: Fn(&usize) -> bool,
{
    Some(
        input
            .trim()
            .split(',')
            .map(|part| part.split_once('-').unwrap())
            .map(|(left, right)| {
                (
                    left.parse::<usize>().unwrap(),
                    right.parse::<usize>().unwrap(),
                )
            })
            .flat_map(|(start, end)| (start..=end).filter(&is_invalid))
            .sum::<usize>() as u64,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    sum_invalid_ids(input, |val| {
        let val = val.to_string();
        let (left, right) = val.split_at(val.len() / 2);
        *left == *right
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    sum_invalid_ids(input, |val| {
        let val = val.to_string();
        let str_len = val.len();
        let charvec: Vec<_> = val.chars().collect();

        (2..=str_len).any(|repetition_count| {
            if str_len % repetition_count != 0 {
                return false;
            }

            let chunk_size = str_len / repetition_count;
            let mut chunk_iter = charvec.chunks(chunk_size);
            let first_chunk_str: String = chunk_iter.next().unwrap().iter().collect();
            chunk_iter.all(|chunk| chunk.iter().collect::<String>() == first_chunk_str)
        })
    })
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
        assert_eq!(result, Some(4174379265));
    }
}
