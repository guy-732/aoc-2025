use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn apply(self, num: i32) -> i32 {
        match self {
            Self::Left => -num,
            Self::Right => num,
        }
    }
}

#[aoc_generator(day01)]
fn parse(input: &str) -> Vec<(Direction, i32)> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|l| {
            if let Some(num) = l.strip_prefix('L') {
                return (Direction::Left, num.parse().expect("Failed to parse num"));
            }

            if let Some(num) = l.strip_prefix('R') {
                return (Direction::Right, num.parse().expect("Failed to parse num"));
            }

            panic!("Line {:?}", l);
        })
        .collect_vec()
}

#[aoc(day01, part1)]
fn part1(input: &[(Direction, i32)]) -> usize {
    let mut count = 0;
    let mut pos = 50;

    for &(dir, num) in input {
        pos += dir.apply(num);
        pos %= 100;
        if pos == 0 {
            count += 1;
        }
    }

    count
}

#[aoc(day01, part2)]
fn part2(input: &[(Direction, i32)]) -> usize {
    let mut count = 0;
    let mut pos = 50;

    for &(dir, num) in input {
        let clamped_num = num % 100;
        count += num as usize / 100;
        let is_zero = pos == 0;
        pos += dir.apply(clamped_num);
        if pos < 0 {
            count += if is_zero { 0 } else { 1 };
            pos += 100;
        } else if pos >= 100 {
            count += if is_zero { 0 } else { 1 };
            pos -= 100;
        } else if pos == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }
}
