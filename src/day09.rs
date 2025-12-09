use geo::{Coord, Intersects, Line, Rect};
use itertools::Itertools;

type Position = Coord<i64>;

fn rect_intersects_no_edge(
    mut rect_a: Coord<i64>,
    mut rect_b: Coord<i64>,
    line: Line<i64>,
) -> bool {
    let rect = Rect::new(rect_a, rect_b);
    if rect.width() < 2 || rect.height() < 2 {
        // they're not gonna be big areas anyways
        return false;
    }

    if rect_a.x < rect_b.x {
        rect_a.x += 1;
        rect_b.x -= 1;
    } else {
        rect_a.x -= 1;
        rect_b.x += 1;
    }

    if rect_a.y < rect_b.y {
        rect_a.y += 1;
        rect_b.y -= 1;
    } else {
        rect_a.y -= 1;
        rect_b.y += 1;
    }

    Rect::new(rect_a, rect_b).intersects(&line)
}

#[aoc_generator(day09)]
fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line
                .split_once(',')
                .unwrap_or_else(|| panic!("Could not parse {:?}", line));

            (
                left.parse()
                    .unwrap_or_else(|err| panic!("Could not parse {:?} ({:?})", line, err)),
                right
                    .parse()
                    .unwrap_or_else(|err| panic!("Could not parse {:?} ({:?})", line, err)),
            )
                .into()
        })
        .collect()
}

#[aoc(day09, part1)]
fn part1(input: &[Position]) -> i64 {
    input
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(l, r)| {
            let rect = Rect::new(l, r);
            (rect.width() + 1) * (rect.height() + 1)
        })
        .max()
        .expect("Empty input")
}

// Yes some of the kept rectangles are not valid... they're too small though so I don't care

#[aoc(day09, part2)]
fn part2(input: &[Position]) -> i64 {
    let last_line = Line::new(input[input.len() - 1], input[0]);

    input
        .iter()
        .copied()
        .tuple_combinations()
        .filter_map(|(l, r)| {
            if input
                .iter()
                .copied()
                .tuple_windows()
                .map(|(c1, c2)| Line::new(c1, c2))
                .chain(std::iter::once(last_line))
                .any(|line| rect_intersects_no_edge(l, r, line))
            {
                None
            } else {
                let rect = Rect::new(l, r);
                Some((rect.width() + 1) * (rect.height() + 1))
            }
        })
        .max()
        .expect("No valid rectangles?")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 24);
    }
}
