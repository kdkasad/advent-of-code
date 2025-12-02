use std::{
    io::{IsTerminal, Read},
    ops::RangeInclusive,
};

use advent_of_code::Solution;

struct Problem;

fn input_to_ranges(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input.trim().split(',').map(|field| {
        let (start, end) = field.split_once('-').unwrap();
        start.parse().unwrap()..=end.parse().unwrap()
    })
}

fn is_invalid_1(num: u64) -> bool {
    let s = format!("{num}");
    if !s.len().is_multiple_of(2) {
        return false;
    }
    let (left, right) = s.split_at(s.len() / 2);
    left == right
}

fn is_invalid_2(num: u64) -> bool {
    let s = format!("{num}");
    for len in 1..=(s.len() / 2) {
        if !s.len().is_multiple_of(len) {
            continue;
        }
        let mut chunks = s.as_bytes().chunks(len);
        if chunks
            .next()
            .is_some_and(|first| chunks.all(|chunk| chunk == first))
        {
            return true;
        }
    }
    false
}

impl<'a> Solution<'a> for Problem {
    type Output = u64;

    fn part1(input: &'a str) -> Self::Output {
        let ranges = input_to_ranges(input);
        let mut sum = 0;
        for range in ranges {
            for num in range {
                if is_invalid_1(num) {
                    sum += num;
                }
            }
        }
        sum
    }

    fn part2(input: &'a str) -> Self::Output {
        let ranges = input_to_ranges(input);
        let mut sum = 0;
        for range in ranges {
            for num in range {
                if is_invalid_2(num) {
                    println!("{num}");
                    sum += num;
                }
            }
        }
        sum
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
