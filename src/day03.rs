use itertools::Itertools;

fn parse_line(input_line: &[u8]) -> Vec<u8> {
    input_line
        .iter()
        .filter_map(|&b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            _ => None,
        })
        .collect_vec()
}

fn first_position_max(line: &[u8]) -> usize {
    let mut idx = 0;

    for (i, v) in line.iter().enumerate().skip(1) {
        if line[idx] < *v {
            idx = i;
        }
    }

    idx
}

fn part1_best_pair_in_line(line: &[u8]) -> u32 {
    let first_idx = first_position_max(&line[..(line.len() - 1)]);
    let second_num = *line[(first_idx + 1)..].iter().max().expect("Empty line???");

    (line[first_idx] * 10 + second_num) as u32
}

fn part2_best_joltage(line: &[u8]) -> u64 {
    let mut sum = 0;
    let mut current_idx = 0;
    for i in (0..12).rev() {
        let line_segment = &line[current_idx..(line.len() - i)];
        let new_max = first_position_max(line_segment);

        sum *= 10;
        sum += line_segment[new_max] as u64;
        current_idx += new_max + 1;
    }

    sum
}

#[aoc_generator(day03)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line.as_bytes()))
        .filter(|line| !line.is_empty())
        .collect_vec()
}

#[aoc(day03, part1)]
fn part1(input: &[Vec<u8>]) -> u32 {
    input.iter().map(|line| part1_best_pair_in_line(line)).sum()
}

#[aoc(day03, part2)]
fn part2(input: &[Vec<u8>]) -> u64 {
    input.iter().map(|line| part2_best_joltage(line)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &[u8] = b"987654321111111";
    const EXAMPLE2: &[u8] = b"811111111111119";
    const EXAMPLE3: &[u8] = b"234234234234278";
    const EXAMPLE4: &[u8] = b"818181911112111";

    #[test]
    fn part1_example1() {
        assert_eq!(part1_best_pair_in_line(&parse_line(EXAMPLE1)), 98);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1_best_pair_in_line(&parse_line(EXAMPLE2)), 89);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1_best_pair_in_line(&parse_line(EXAMPLE3)), 78);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1_best_pair_in_line(&parse_line(EXAMPLE4)), 92);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2_best_joltage(&parse_line(EXAMPLE1)), 987654321111);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2_best_joltage(&parse_line(EXAMPLE2)), 811111111119);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2_best_joltage(&parse_line(EXAMPLE3)), 434234234278);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2_best_joltage(&parse_line(EXAMPLE4)), 888911112111);
    }
}
