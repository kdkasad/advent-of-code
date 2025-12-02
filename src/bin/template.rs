use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = &'a str;

    fn part1(input: &'a str) -> Self::Output {
        ""
    }

    fn part2(input: &'a str) -> Self::Output {
        ""
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
