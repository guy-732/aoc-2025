use std::{cmp, error::Error, str::FromStr};

use fnv::FnvHashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, other: &Self) -> u64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        let s = (x * x + y * y + z * z) as f64;
        s.sqrt() as u64
    }
}

impl FromStr for Position {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((x, y, z)) = s.split(',').collect_tuple() else {
            return Err(format!("Could not split string {:?} 3 times on ','", s).into());
        };

        Ok(Self::new(x.parse()?, y.parse()?, z.parse()?))
    }
}

#[aoc_generator(day08)]
fn parse(input: &str) -> Result<Vec<Position>, Box<dyn Error>> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::parse)
        .try_collect()
}

fn make_connections(
    junction_box: &[Position],
    connections_limit: usize,
) -> FnvHashMap<usize, Vec<Position>> {
    let mut junction_box_circuits: FnvHashMap<Position, usize> = junction_box
        .iter()
        .copied()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let mut circuits: FnvHashMap<usize, Vec<Position>> = junction_box_circuits
        .iter()
        .map(|(&pos, &id)| (id, vec![pos]))
        .collect();

    let mut candidates = junction_box
        .iter()
        .copied()
        .tuple_combinations()
        .collect_vec();

    candidates.sort_by_key(|(a, b)| a.distance(b));

    for candidate in candidates.into_iter().take(connections_limit) {
        let circuit_id = junction_box_circuits[&candidate.0];
        let other_circuit = junction_box_circuits[&candidate.1];
        if circuit_id == other_circuit {
            continue;
        }

        let other_circuit = circuits
            .remove(&other_circuit)
            .expect("Circuit no longer existed?");
        other_circuit.iter().for_each(|pos| {
            junction_box_circuits.insert(*pos, circuit_id);
        });

        circuits
            .get_mut(&circuit_id)
            .expect("Circuit no longer existed?")
            .extend(other_circuit);
    }

    circuits
}

fn part1_logic(junction_box: &[Position], connections_limit: usize) -> u64 {
    let circuits = make_connections(junction_box, connections_limit);

    circuits
        .into_values()
        .map(|c| c.len() as u64)
        .sorted_by_key(|&c| cmp::Reverse(c))
        .take(3)
        .product()
}

fn part2_logic(junction_box: &[Position]) -> (Position, Position) {
    let mut junction_box_circuits: FnvHashMap<Position, usize> = junction_box
        .iter()
        .copied()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let mut circuits: FnvHashMap<usize, Vec<Position>> = junction_box_circuits
        .iter()
        .map(|(&pos, &id)| (id, vec![pos]))
        .collect();

    let mut candidates = junction_box
        .iter()
        .copied()
        .tuple_combinations()
        .collect_vec();

    candidates.sort_by_key(|(a, b)| a.distance(b));

    for candidate in candidates.into_iter() {
        let circuit_id = junction_box_circuits[&candidate.0];
        let other_circuit = junction_box_circuits[&candidate.1];
        if circuit_id == other_circuit {
            continue;
        }

        if circuits.len() <= 2 {
            return candidate;
        }

        let other_circuit = circuits
            .remove(&other_circuit)
            .expect("Circuit no longer existed?");
        other_circuit.iter().for_each(|pos| {
            junction_box_circuits.insert(*pos, circuit_id);
        });

        circuits
            .get_mut(&circuit_id)
            .expect("Circuit no longer existed?")
            .extend(other_circuit);
    }

    unreachable!("Could not connect everything?")
}

#[aoc(day08, part1)]
fn part1(input: &[Position]) -> u64 {
    part1_logic(input, 1_000)
}

#[aoc(day08, part2)]
fn part2(input: &[Position]) -> i64 {
    let (a, b) = part2_logic(input);
    a.x * b.x
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1_logic(&parse(EXAMPLE).expect("Could not parse"), 10),
            40
        );
    }

    #[test]
    fn part2_example() {
        let (a, b) = part2_logic(&parse(EXAMPLE).expect("Could not parse"));

        assert_eq!(a.x * b.x, 25272, "{:?}", (a, b));
    }
}
