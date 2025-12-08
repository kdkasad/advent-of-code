use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = u64;

    fn part1(input: &'a str) -> Self::Output {
        let mut splits = 0;
        let width = input.lines().next().unwrap().len();
        let mut beams: Box<[bool]> = vec![false; width].into_boxed_slice();
        let mut new_beams: Box<[bool]> = vec![false; width].into_boxed_slice();
        for line in input.lines() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    'S' => new_beams[i] = true,
                    '.' => {
                        new_beams[i] |= beams[i];
                    }
                    '^' => {
                        if beams[i] {
                            splits += 1;
                            if i > 0 {
                                new_beams[i - 1] = true;
                            }
                            if i < width - 1 {
                                new_beams[i + 1] = true;
                            }
                        }
                    }
                    _ => panic!("Invalid input character '{c}'"),
                }
            }
            std::mem::swap(&mut beams, &mut new_beams);
            new_beams.iter_mut().for_each(|b| *b = false);
        }
        splits
    }

    fn part2(input: &'a str) -> Self::Output {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let (start_i, start_j, _) = grid
            .iter()
            .enumerate()
            .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, &c)| (i, j, c)))
            .find(|&(_, _, c)| c == 'S')
            .unwrap();
        let mut dp = vec![vec![0u64; grid[0].len()]; grid.len()];
        // Set last row to 1's
        dp.last_mut().unwrap().fill(1);
        for i in (0..(grid.len() - 1)).rev() {
            for j in 0..grid[i].len() {
                match grid[i][j] {
                    '.' | 'S' => dp[i][j] = dp[i + 1][j],
                    '^' => dp[i][j] = dp[i + 1][j - 1] + dp[i + 1][j + 1],
                    c => panic!("Unexpected character '{c}'"),
                }
            }
        }
        dp[start_i][start_j]
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
    fn part2_sample1() {
        let input = include_str!("../../inputs/2025-12-07-sample.txt");
        let result = Problem::part2(input);
        assert_eq!(40, result);
    }
}
