use std::fmt::{self, Write};

use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Free,
    Splitter,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Tile::Free => '.',
            Tile::Splitter => '^',
        })
    }
}

#[aoc_generator(day07)]
fn parse(input: &str) -> (ndarray::Array2<Tile>, Position) {
    let mut start = (0, 0);
    let arrays = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .copied()
                .enumerate()
                .map(|(col, b)| match b {
                    b'.' => Tile::Free,
                    b'^' => Tile::Splitter,
                    b'S' => {
                        start = (row, col);
                        Tile::Free
                    }
                    _ => panic!("{:?} was not in ('.', '^', 'S')", b as char),
                })
                .collect_vec()
        })
        .collect_vec();

    let height = arrays.len();
    let len = arrays[0].len();
    let array = arrays
        .into_iter()
        .inspect(|line| {
            if line.len() != len {
                panic!("Lines with differing length in input");
            }
        })
        .flatten()
        .collect_vec();

    (
        ndarray::Array2::from_shape_vec((height, len), array).expect("Could not create ndarray"),
        start,
    )
}

fn part1_logic(
    array: &ndarray::Array2<Tile>,
    init_pos: Position,
    beams: &mut FnvHashSet<Position>,
    active_splitters: &mut FnvHashSet<Position>,
) {
    let mut positions = vec![init_pos];
    beams.insert(init_pos);

    while let Some(mut pos) = positions.pop() {
        loop {
            pos.0 += 1;
            if !beams.insert(pos) {
                break;
            }

            let Some(&tile) = array.get(pos) else {
                break;
            };

            if tile == Tile::Splitter {
                let splits = [(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)];
                active_splitters.insert(pos);

                for split in splits {
                    if beams.insert(split) {
                        positions.push(split);
                    }
                }

                break;
            }
        }
    }
}

fn part2_logic(
    array: &ndarray::Array2<Tile>,
    pos: Position,
    timelines: &mut FnvHashMap<Position, u64>,
) -> u64 {
    if let Some(&cached) = timelines.get(&pos) {
        return cached;
    }

    let res = if let Some(&tile) = array.get(pos) {
        match tile {
            Tile::Free => part2_logic(array, (pos.0 + 1, pos.1), timelines),
            Tile::Splitter => {
                part2_logic(array, (pos.0, pos.1 - 1), timelines)
                    + part2_logic(array, (pos.0, pos.1 + 1), timelines)
            }
        }
    } else {
        1
    };

    timelines.insert(pos, res);
    res
}

#[aoc(day07, part1)]
fn part1(input: &(ndarray::Array2<Tile>, Position)) -> usize {
    let mut active_splitters = FnvHashSet::default();

    part1_logic(
        &input.0,
        input.1,
        &mut FnvHashSet::default(),
        &mut active_splitters,
    );

    active_splitters.len()
}

#[aoc(day07, part2)]
fn part2(input: &(ndarray::Array2<Tile>, Position)) -> u64 {
    part2_logic(&input.0, input.1, &mut FnvHashMap::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 40);
    }
}
