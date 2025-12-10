use std::io::{IsTerminal, Read};

use advent_of_code::Solution;
use bitvec::prelude::*;

#[derive(Debug, Clone)]
struct Machine {
    expected_lights: BitBox,
    buttons: Box<[Box<[usize]>]>,
    joltage: Box<[u64]>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Machine> {
    input.lines().map(|line| {
        let mut lights = BitVec::new();
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        for part in line.split_whitespace() {
            match part.chars().next().unwrap() {
                '[' => {
                    for c in part.trim_matches(['[', ']']).chars() {
                        lights.push(c == '#');
                    }
                }
                '(' => {
                    let light_indices = part
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|i| i.parse().unwrap())
                        .collect();
                    buttons.push(light_indices);
                }
                '{' => {
                    for num in part.trim_matches(['{', '}']).split(',') {
                        joltage.push(num.parse().unwrap());
                    }
                }
                _ => panic!("Invalid input part \"{part}\""),
            }
        }
        Machine {
            expected_lights: lights.into_boxed_bitslice(),
            buttons: buttons.into_boxed_slice(),
            joltage: joltage.into_boxed_slice(),
        }
    })
}

/// Returns an iterator over all subsets of the elements in the given list.
fn power_set<T>(set: &[T]) -> impl Iterator<Item = impl Iterator<Item = &T> + Clone> {
    assert!(set.len() <= usize::BITS as usize);
    (0..2usize.pow(set.len() as u32)).map(|mask| {
        set.iter()
            .enumerate()
            .filter(move |(i, _)| ((mask >> i) & 1) == 1)
            .map(|(_, item)| item)
    })
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = usize;

    fn part1(input: &'a str) -> Self::Output {
        let mut total = 0;
        for machine in parse_input(input) {
            let mut lights = bitbox![0; machine.expected_lights.len()];
            let min_buttons = power_set(&machine.buttons)
                .filter(|subset| {
                    lights.fill(false);
                    for button in subset.clone() {
                        for &i in button {
                            let bit = lights[i];
                            lights.set(i, !bit);
                        }
                    }
                    lights == machine.expected_lights
                })
                .map(|subset| subset.count())
                .min()
                .unwrap();
            total += min_buttons;
        }
        total
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
    fn part1_sample1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n";
        let actual = Problem::part1(input);
        assert_eq!(2, actual);
    }

    #[test]
    fn part2_sample1() {}
}
