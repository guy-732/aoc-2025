use std::fmt::{self, Write};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

const ALL_NEIGHBOURS: [Position; 8] = [
    Position(-1, -1),
    Position(-1, 0),
    Position(-1, 1),
    Position(0, -1),
    Position(0, 1),
    Position(1, -1),
    Position(1, 0),
    Position(1, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Tile {
    #[default]
    Free,
    Roll,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Tile::Free => '.',
            Tile::Roll => '@',
        })
    }
}

#[aoc_generator(day04)]
fn parse(input: &str) -> ndarray::Array2<Tile> {
    let arrays = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&b| match b {
                    b'.' => Tile::Free,
                    b'@' => Tile::Roll,
                    _ => panic!("Byte {:?} is neither '.' nor '@'", b as char),
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

    ndarray::Array2::<Tile>::from_shape_vec((height, len), array).expect("Could not create ndarray")
}

fn get_position(grid: &ndarray::Array2<Tile>, position: Position) -> Tile {
    if position.0 < 0 || position.1 < 0 {
        return Tile::Free;
    }

    grid.get((position.0 as usize, position.1 as usize))
        .copied()
        .unwrap_or_default()
}

fn remove_roll(grid: &mut ndarray::Array2<Tile>, position: Position) {
    grid[(position.0 as usize, position.1 as usize)] = Tile::Free;
}

fn part1_check_roll(grid: &ndarray::Array2<Tile>, roll_position: Position) -> bool {
    if get_position(grid, roll_position) != Tile::Roll {
        return false;
    }

    ALL_NEIGHBOURS
        .iter()
        .filter(|&&offset| get_position(grid, roll_position + offset) == Tile::Roll)
        .count()
        < 4
}

#[aoc(day04, part1)]
fn part1(input: &ndarray::Array2<Tile>) -> usize {
    let mut count = 0;
    for i in 0..input.shape()[0] as isize {
        for j in 0..input.shape()[1] as isize {
            if part1_check_roll(input, Position(i, j)) {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day04, part2)]
fn part2(input: &ndarray::Array2<Tile>) -> usize {
    let mut grid = input.clone();
    let mut removed = 0;
    let mut has_removed = true;

    while has_removed {
        has_removed = false;

        for i in 0..grid.shape()[0] as isize {
            for j in 0..grid.shape()[1] as isize {
                if part1_check_roll(&grid, Position(i, j)) {
                    has_removed = true;
                    removed += 1;
                    remove_roll(&mut grid, Position(i, j));
                }
            }
        }
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 43);
    }
}
