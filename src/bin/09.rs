use std::{collections::HashMap, ops::RangeInclusive};

use itertools::{Itertools, MinMaxResult};

advent_of_code::solution!(9);

type Coord = (usize, usize);

fn parse_input(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    parse_input(input)
        .iter()
        .tuple_combinations()
        .map(|((a_x, a_y), (b_x, b_y))| {
            let w = a_x.abs_diff(*b_x) + 1;
            let h = a_y.abs_diff(*b_y) + 1;
            (w * h) as u64
        })
        .max()
}

fn _get_h_and_v_ranges_for_bbox(
    line: &(&Coord, &Coord),
) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let ((a_x, a_y), (b_x, b_y)) = line;
    let h_range = (*a_x.min(b_x))..=(*a_x.max(b_x));
    let v_range = (*a_y.min(b_y))..=(*a_y.max(b_y));
    (h_range, v_range)
}

// enum LineType {
//     Horizontal(usize),
//     Vertical(usize),
// }

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from(line: &(&Coord, &Coord)) -> Self {
        let ((a_x, a_y), (b_x, b_y)) = line;
        if a_x == b_x {
            // Line is vertical
            if a_y < b_y {
                Direction::South
            } else {
                Direction::North
            }
        } else {
            // Line is horizontal
            if a_x < b_x {
                Direction::East
            } else {
                Direction::West
            }
        }
    }

    // fn normal(self: &Self) -> Self {
    //     match self {
    //         Self::North => Self::East,
    //         Self::East => Self::South,
    //         Self::South => Self::West,
    //         Self::West => Self::North,
    //     }
    // }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Line {
    direction: Direction,
    start_pos: usize,
    len: isize,
    cross_pos: usize,
}

impl Line {
    fn from(line: (&Coord, &Coord)) -> Self {
        let direction = Direction::from(&line);

        let (start_pos, end_pos) = match direction {
            Direction::North | Direction::South => (line.0.1, line.1.1),
            Direction::East | Direction::West => (line.0.0, line.1.0),
        };

        let cross_pos = match direction {
            Direction::North | Direction::South => line.0.0,
            Direction::East | Direction::West => line.0.1,
        };

        let len = end_pos as isize - start_pos as isize;

        Self {
            direction,
            start_pos,
            len,
            cross_pos,
        }
    }

    fn get_range(self: &Self) -> RangeInclusive<usize> {
        let end_pos = (self.start_pos as isize + self.len) as usize;

        let start = self.start_pos.min(end_pos);
        let end = self.start_pos.max(end_pos);

        start..=end
    }

    fn get_interior_range(self: &Self) -> Option<RangeInclusive<usize>> {
        let range = self.get_range();
        let start = range.start() + 1;
        let end = range.end() - 1;

        if start <= end {
            Some(start..=end)
        } else {
            None
        }
    }

    fn perpendicular_to(self: &Self, other: &Self) -> bool {
        match (self.direction, other.direction) {
            (Direction::North | Direction::South, Direction::North | Direction::South)
            | (Direction::East | Direction::West, Direction::East | Direction::West) => false,

            (Direction::North | Direction::South, Direction::East | Direction::West)
            | (Direction::East | Direction::West, Direction::North | Direction::South) => true,
        }
    }

    fn _inline_with(self: &Self, other: &Self) -> bool {
        !self.perpendicular_to(other) && self.cross_pos == other.cross_pos
    }

    fn intersects(self: &Self, other: &Self) -> bool {
        self.perpendicular_to(other)
            && self
                .get_interior_range()
                .map_or(false, |range| range.contains(&other.cross_pos))
            && other
                .get_interior_range()
                .map_or(false, |range| range.contains(&self.cross_pos))
    }

    fn _overlaps(self: &Self, other: &Self) -> bool {
        self._inline_with(other) && {
            let self_range = self.get_range();
            let other_range = other.get_range();
            self_range.contains(other_range.start())
                || self_range.contains(other_range.end())
                || other_range.contains(self_range.start()) | other_range.contains(self_range.end())
        }
    }

    fn _get_repr_at_y(self: &Self, y: usize) -> Vec<(usize, char)> {
        let end_pos = (self.start_pos as isize + self.len) as usize;
        match self.direction {
            Direction::North | Direction::South => {
                if y == self.start_pos || y == end_pos {
                    vec![(self.cross_pos, 'O')]
                } else if self.get_range().contains(&y) {
                    vec![(
                        self.cross_pos,
                        if self.direction == Direction::North {
                            '^'
                        } else {
                            'v'
                        },
                    )]
                } else {
                    vec![]
                }
            }
            Direction::East | Direction::West => {
                if y == self.cross_pos {
                    Vec::from_iter(self.get_range().map(|x| {
                        (
                            x,
                            if x == self.start_pos || x == end_pos {
                                'O'
                            } else {
                                if self.direction == Direction::East {
                                    '>'
                                } else {
                                    '<'
                                }
                            },
                        )
                    }))
                } else {
                    vec![]
                }
            }
        }
    }

    fn iter_coords_except_end(self: &Self) -> impl Iterator<Item = Coord> {
        let range = self.get_range();
        (*range.start()..*range.end()).map(|l| match self.direction {
            Direction::North | Direction::South => (self.cross_pos, l),
            Direction::East | Direction::West => (l, self.cross_pos),
        })
    }
}

// fn line_matching<F>(line_type: LineType) -> F
// where
//     F: Fn(&(&Coord, &Coord)) -> bool,
// {
//     todo!()
// }

fn _lpad(s: &str, n: usize) -> String {
    [&" ".repeat(n - s.len()), s].join("")
}

fn _show_lines(a: &Line, b: &Line, x_bounds: &MinMaxResult<usize>, y_bounds: &MinMaxResult<usize>) {
    let (x_min, x_max) = match x_bounds {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!(),
    };
    let (y_min, y_max) = match y_bounds {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!(),
    };

    let x_min_str = x_min.to_string();
    let x_max_str = x_max.to_string();
    let y_min_str = y_min.to_string();
    let y_max_str = y_max.to_string();

    let y_label_len = y_min_str.len().max(y_max_str.len());
    let y_full_padding = " ".repeat(y_label_len);
    let inner_width = *x_max - *x_min + 1;

    let upper_lower_bound = ["+", &"-".repeat(inner_width), "+"].join("");

    println!(
        "{}{}",
        _lpad(&x_min_str, y_label_len + 2),
        _lpad(&x_max_str, x_max - x_min)
    );
    println!("{}{}", y_full_padding, upper_lower_bound);
    for y in *y_min..=*y_max {
        let mut inner_representation = vec![' '; inner_width];

        for line_repr in [a._get_repr_at_y(y), b._get_repr_at_y(y)] {
            for (x, c) in line_repr {
                let x = x - x_min;
                inner_representation[x] = if inner_representation[x] == ' ' {
                    c
                } else {
                    '#'
                };
            }
        }

        println!(
            "{}|{}|",
            _lpad(
                if y == *y_min {
                    &y_min_str
                } else if y == *y_max {
                    &y_max_str
                } else {
                    ""
                },
                y_label_len
            ),
            inner_representation.iter().collect::<String>()
        );
    }
    println!("{}{}", y_full_padding, upper_lower_bound);
}

fn coord_within_outline(
    coord: Coord,
    outline: &Vec<Line>,
    max_x: usize,
    memo_map: &mut HashMap<Coord, bool>,
) -> bool {
    let memo = memo_map.get(&coord);
    match memo {
        Some(memo_val) => return *memo_val,
        None => {}
    }

    let (x, y) = coord;
    let test_line = Line {
        direction: Direction::East,
        start_pos: x,
        len: (max_x - x + 1) as isize,
        cross_pos: y,
    };

    let lines_crossed = outline
        .iter()
        .filter(|bounds_line| bounds_line.intersects(&test_line))
        .count();

    let in_outline = lines_crossed % 2 == 1;

    memo_map.insert(coord, in_outline);
    in_outline
}

pub fn part_two(input: &str) -> Option<u64> {
    let red_tiles = parse_input(input);
    let usable_area_outline_lines: Vec<_> = red_tiles
        .iter()
        .circular_tuple_windows()
        .map(Line::from)
        .collect();

    let (tile_xs, _tile_ys): (Vec<_>, Vec<_>) = red_tiles.to_owned().into_iter().unzip();
    let max_x = tile_xs.into_iter().max().unwrap();

    // let x_bounds = tile_xs.into_iter().minmax();
    // let y_bounds = tile_ys.into_iter().minmax();

    let mut coord_in_outline_memo = HashMap::new();

    red_tiles
        .iter()
        .tuple_combinations()
        .filter(|bbox_points: &(_, _)| {
            println!("Checking bbox {:?}", bbox_points);

            [
                *bbox_points.0,
                (bbox_points.0.0, bbox_points.1.1),
                *bbox_points.1,
                (bbox_points.1.0, bbox_points.0.1),
            ]
            .iter()
            .tuple_combinations()
            .map(Line::from)
            .flat_map(|line| line.iter_coords_except_end().collect::<Vec<_>>())
            .all(|coord| {
                coord_within_outline(
                    coord,
                    &usable_area_outline_lines,
                    max_x,
                    &mut coord_in_outline_memo,
                )
            })
        })
        .map(|((a_x, a_y), (b_x, b_y))| {
            let w = a_x.abs_diff(*b_x) + 1;
            let h = a_y.abs_diff(*b_y) + 1;
            (w * h) as u64
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
