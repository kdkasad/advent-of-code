use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> impl Iterator<Item = Point> {
    input.lines().map(|line| {
        let (l, r) = line.split_once(',').unwrap();
        Point {
            x: l.parse().unwrap(),
            y: r.parse().unwrap(),
        }
    })
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = usize;

    fn part1(input: &'a str) -> Self::Output {
        let points: Vec<Point> = parse_input(input).collect();
        points
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                points[..i].iter().map(|b| {
                    let side_x = usize::abs_diff(a.x, b.x) + 1;
                    let side_y = usize::abs_diff(a.y, b.y) + 1;
                    side_x * side_y
                })
            })
            .max()
            .unwrap()
    }

    fn part2(input: &'a str) -> Self::Output {
        0
    }
}

fn main() {
    if std::io::stdin().is_terminal() {
        eprintln!("Waiting for input...");
    }

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let result = Problem::part1(&input);
    println!("Part 1: {}", result);
    let result = Problem::part2(&input);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::Problem;
    use advent_of_code::Solution as _;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_sample1() {}

    #[test]
    fn part2_sample1() {}
}
