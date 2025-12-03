use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[aoc_generator(day02)]
fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(str::trim)
        .map(|range| {
            let (l, r) = range.split_once('-').expect("Invalid range");
            (
                l.trim().parse().expect("Cannot parse u64"),
                r.trim().parse().expect("Cannot parse u64"),
            )
        })
        .collect_vec()
}

#[aoc(day02, part1)]
fn part1(input: &[(u64, u64)]) -> u64 {
    input.iter().copied().map(part1_check_range).sum()
}

fn part1_check_range(range: (u64, u64)) -> u64 {
    (range.0..=range.1)
        .into_par_iter()
        .filter(|&n| !part1_is_valid(n))
        .sum()
}

fn part1_is_valid(num: u64) -> bool {
    let v = num as f64;
    let size = v.log10().floor() as u32 + 1;

    if !size.is_multiple_of(2) {
        return true;
    }

    let size = size / 2;

    let middle = u64::pow(10, size);
    let lower = num % middle;
    let upper = num / middle;

    lower != upper
}

#[aoc(day02, part2)]
fn part2(input: &[(u64, u64)]) -> u64 {
    input.iter().copied().map(part2_check_range).sum()
}

fn part2_check_range(range: (u64, u64)) -> u64 {
    (range.0..=range.1)
        .into_par_iter()
        .filter(|&n| !part2_is_valid(n))
        .sum()
}

fn part2_is_valid(num: u64) -> bool {
    let v = num as f64;
    let size = v.log10().floor() as u32 + 1;

    'outer: for i in 2..=size {
        if !size.is_multiple_of(i) {
            continue;
        }

        let fraction = size / i;
        let parts_divider = u64::pow(10, fraction);

        let expected = num % parts_divider;
        let mut n = num;
        for _ in 1..i {
            n /= parts_divider;
            if n % parts_divider != expected {
                continue 'outer;
            }
        }

        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&[(11, 22)]), 33);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&[(95, 115)]), 99);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&[(998, 1012)]), 1010);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(&[(1188511880, 1188511890)]), 1188511885);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1(&[(222220, 222224)]), 222222);
    }

    #[test]
    fn part1_example6() {
        assert_eq!(part1(&[(1698522, 1698528)]), 0);
    }

    #[test]
    fn part1_example7() {
        assert_eq!(part1(&[(446443, 446449)]), 446446);
    }

    #[test]
    fn part1_example8() {
        assert_eq!(part1(&[(38593856, 38593862)]), 38593859);
    }

    #[test]
    fn part1_examples_rest() {
        assert_eq!(
            part1(&parse(
                "565653-565659,824824821-824824827,2121212118-2121212124"
            )),
            0
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2_check_range((11, 22)), 11 + 22);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2_check_range((95, 115)), 99 + 111);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2_check_range((998, 1012)), 999 + 1010);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2_check_range((1188511880, 1188511890)), 1188511885);
    }

    #[test]
    fn part2_example5() {
        assert_eq!(part2_check_range((222220, 222224)), 222222);
    }

    #[test]
    fn part2_example6() {
        assert_eq!(part2_check_range((1698522, 1698528)), 0);
    }

    #[test]
    fn part2_example7() {
        assert_eq!(part2_check_range((446443, 446449)), 446446);
    }

    #[test]
    fn part2_example8() {
        assert_eq!(part2_check_range((38593856, 38593862)), 38593859);
    }

    #[test]
    fn part2_example9() {
        assert_eq!(part2_check_range((565653, 565659)), 565656);
    }

    #[test]
    fn part2_example10() {
        assert_eq!(part2_check_range((824824821, 824824827)), 824824824);
    }

    #[test]
    fn part2_example11() {
        assert_eq!(part2_check_range((2121212118, 2121212124)), 2121212121);
    }
}
