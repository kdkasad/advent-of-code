use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

struct Problem;

fn input_to_offsets(input: &str) -> impl Iterator<Item = i32> {
    input
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..].parse::<u32>().unwrap(),
            )
        })
        .map(|(dir, distance)| match dir {
            'L' => -(distance as i32),
            'R' => distance as i32,
            other => panic!("Invalid direction {other}"),
        })
}

impl<'a> Solution<'a> for Problem {
    type Output = u32;

    fn part1(input: &'a str) -> Self::Output {
        let mut dial = 50;
        let mut count = 0;
        for offset in input_to_offsets(input) {
            dial = (dial + offset).rem_euclid(100);
            if dial == 0 {
                count += 1;
            }
        }
        count
    }

    fn part2(input: &'a str) -> Self::Output {
        let mut dial: i32 = 50;
        let mut count: u32 = 0;
        for offset in input_to_offsets(input) {
            let full_rotations = (offset / 100).unsigned_abs();
            count += full_rotations;
            let offset_rem = offset % 100;
            let dial_start = dial;
            dial += offset_rem;
            if (dial_start > 0 && dial <= 0) || dial >= 100 {
                count += 1;
            }
            dial = dial.rem_euclid(100);
        }
        count
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
