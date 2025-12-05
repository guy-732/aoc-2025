use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;

#[aoc_generator(day05)]
fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut iter = input
        .lines()
        .map(str::trim)
        .skip_while(|line| line.is_empty());

    let ranges = iter
        .take_while_ref(|line| !line.is_empty())
        .map(|line| {
            let (lower, upper) = line.split_once('-').expect("Could not split range");

            (
                lower.parse().expect("Could not parse number"),
                upper.parse().expect("Could not parse number"),
            )
        })
        .collect_vec();

    let ids = iter
        .filter(|line| !line.is_empty())
        .map(str::parse)
        .try_collect()
        .expect("Could not parse number");

    (ranges, ids)
}

fn id_in_any_range(ranges: &[(u64, u64)], id: u64) -> bool {
    for &(lower, upper) in ranges {
        if (lower..=upper).contains(&id) {
            return true;
        }
    }

    false
}

#[aoc(day05, part1)]
fn part1(input: &(Vec<(u64, u64)>, Vec<u64>)) -> usize {
    input
        .1
        .iter()
        .filter(|&&id| id_in_any_range(&input.0, id))
        .count()
}

fn optimize_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    // BinaryHeap to the rescue
    let mut heap: BinaryHeap<_> = ranges.iter().copied().map(Reverse).collect();
    let mut result = vec![];

    while let Some(Reverse((lower, upper))) = heap.pop() {
        let Some((_, last_upper)) = result.last_mut() else {
            result.push((lower, upper));
            continue;
        };

        if lower - 1 <= *last_upper {
            *last_upper = upper.max(*last_upper);
            continue;
        }

        result.push((lower, upper));
    }

    result
}

#[aoc(day05, part2)]
fn part2(input: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    optimize_ranges(&input.0)
        .into_iter()
        // .inspect(|range| print!("{range:?} -> "))
        .map(|(lower, upper)| upper + 1 - lower)
        // .inspect(|res| println!("{res}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 14);
    }
}
