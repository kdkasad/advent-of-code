use std::{
    io::{IsTerminal, Read},
    str::FromStr,
};

use advent_of_code::Solution;
use rangemap::RangeInclusiveSet;

/// Type which cannot be constructed
enum Never {}

struct Input {
    fresh_ranges: RangeInclusiveSet<u64>,
    ingredients: Vec<u64>,
}

impl FromStr for Input {
    type Err = Never;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let fresh_ranges: RangeInclusiveSet<u64> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (l, r) = line.split_once('-').unwrap();
                let (min, max) = (l.parse().unwrap(), r.parse().unwrap());
                min..=max
            })
            .collect();
        let ingredients = lines.map(|line| line.parse().unwrap()).collect();
        Ok(Input {
            fresh_ranges,
            ingredients,
        })
    }
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = u64;

    fn part1(input: &'a str) -> Self::Output {
        let Ok(Input {
            fresh_ranges,
            ingredients,
        }) = input.parse();
        let mut fresh_count = 0;
        for ingredient in ingredients {
            if fresh_ranges.contains(&ingredient) {
                fresh_count += 1;
            }
        }
        fresh_count
    }

    fn part2(input: &'a str) -> Self::Output {
        let Ok(Input { fresh_ranges, .. }) = input.parse();
        fresh_ranges
            .into_iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
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
