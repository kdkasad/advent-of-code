use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

struct Problem;

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap()))
}

impl<'a> Solution<'a> for Problem {
    type Output = u64;

    fn part1(input: &'a str) -> Self::Output {
        let mut sum = 0;
        for bank in parse_input(input) {
            let bank: Vec<u32> = bank.collect();
            let mut first_digit_i = 0;
            for i in 1..(bank.len() - 1) {
                if bank[i] > bank[first_digit_i] {
                    first_digit_i = i;
                }
            }
            let first_digit = bank[first_digit_i];
            let second_digit = *bank[(first_digit_i + 1)..].iter().max().unwrap();
            sum += (first_digit * 10 + second_digit) as u64;
        }
        sum
    }

    fn part2(input: &'a str) -> Self::Output {
        let mut sum = 0;
        for bank in parse_input(input) {
            let bank: Vec<u32> = bank.collect();
            let mut chosen_indices = Vec::with_capacity(12);
            for i in 0..12 {
                let search_start = if i > 0 { chosen_indices[i - 1] + 1 } else { 0 }; // inclusive
                let search_end = bank.len() - (11 - i); // exclusive
                let (max_i, _) = bank
                    .iter()
                    .copied()
                    .enumerate()
                    .skip(search_start)
                    .take(search_end - search_start)
                    .max_by_key(|&(i, val)| (val, -(i as i32)))
                    .unwrap();
                chosen_indices.push(max_i);
            }
            for (i, bi) in chosen_indices.into_iter().enumerate() {
                sum += (bank[bi] as u64) * 10u64.pow(12 - i as u32 - 1);
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
