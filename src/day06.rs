use itertools::{Itertools, zip_eq};
use ndarray::Axis;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
}

impl TryFrom<char> for Operator {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Multiply),
            _ => Err("Operator can only be '+' or '*'"),
        }
    }
}

#[aoc_generator(day06, part1)]
fn parse_p1(input: &str) -> (ndarray::Array2<u64>, ndarray::Array1<Operator>) {
    let mut lines = input.lines().map(str::trim).filter(|line| !line.is_empty());
    let arrays = lines
        .take_while_ref(|line| line.starts_with(|c: char| c.is_ascii_digit()))
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().expect("Could not parse u64"))
                .collect_vec()
        })
        .collect_vec();

    let operators = lines
        .next()
        .expect("No operators line in input")
        .chars()
        .map(Operator::try_from)
        .filter_map(|res| res.ok())
        .collect();

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
        operators,
    )
}

#[aoc_generator(day06, part2)]
fn parse_p2(input: &str) -> (ndarray::Array2<u8>, ndarray::Array1<Operator>) {
    let mut lines = input.lines();
    let arrays = lines
        .take_while_ref(|line| line.trim_start().starts_with(|c: char| c.is_ascii_digit()))
        .map(|line| line.as_bytes().to_owned())
        .collect_vec();

    let operators = lines
        .next()
        .expect("No operators line in input")
        .chars()
        .map(Operator::try_from)
        .filter_map(|res| res.ok())
        .collect();

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
        operators,
    )
}

#[aoc(day06, part1)]
fn part1(input: &(ndarray::Array2<u64>, ndarray::Array1<Operator>)) -> u64 {
    zip_eq(input.0.axis_iter(Axis(1)), input.1.iter())
        .map(|(column, &op)| match op {
            Operator::Add => column.sum(),
            Operator::Multiply => column.product(),
        })
        .sum()
}

fn resolve_numbers(matrix: &ndarray::Array2<u8>) -> Vec<Vec<u64>> {
    let mut iter = matrix.axis_iter(Axis(1));
    let mut result = vec![];

    loop {
        let row = iter
            .by_ref()
            .map_while(|axis| {
                axis.iter()
                    .filter_map(|&d| match d {
                        b'0'..=b'9' => Some((d - b'0') as u64),
                        _ => None,
                    })
                    .reduce(|acc, element| acc * 10 + element)
            })
            .collect_vec();

        if !row.is_empty() {
            result.push(row);
            continue;
        }

        if iter.clone().next().is_none() {
            break result;
        }
    }
}

#[aoc(day06, part2)]
fn part2(input: &(ndarray::Array2<u8>, ndarray::Array1<Operator>)) -> u64 {
    let resolved = resolve_numbers(&input.0);
    resolved
        .into_iter()
        .zip_eq(input.1.iter().copied())
        .map(|(row, op)| match op {
            Operator::Add => row.into_iter().sum::<u64>(),
            Operator::Multiply => row.into_iter().product(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_p1(EXAMPLE)), 4277556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_p2(EXAMPLE)), 3263827);
    }
}
