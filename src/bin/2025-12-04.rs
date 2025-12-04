use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Paper,
    Empty,
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Paper,
                    c => panic!("Invalid character '{c}' in input"),
                })
                .collect()
        })
        .collect()
}

/// Returns an iterator over the neighbors of cell (i, j) in the grid.
fn neighbors(grid: &[Vec<Cell>], i: usize, j: usize) -> impl Iterator<Item = Cell> {
    [-1, 0, 1]
        .into_iter()
        .flat_map(|di| [-1, 0, 1].into_iter().map(move |dj| (di, dj)))
        .filter(|&(di, dj)| !(di == 0 && dj == 0))
        .map(move |(di, dj)| (i.wrapping_add_signed(di), j.wrapping_add_signed(dj)))
        .filter_map(|(y, x)| grid.get(y).and_then(|row| row.get(x).copied()))
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = usize;

    fn part1(input: &'a str) -> Self::Output {
        let grid = parse_input(input);
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &cell)| cell == Cell::Paper)
                    .map(move |(j, _)| (i, j))
            })
            .map(|(i, j)| {
                if neighbors(&grid, i, j)
                    .filter(|&neighbor| neighbor == Cell::Paper)
                    .count()
                    < 4
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn part2(input: &'a str) -> Self::Output {
        let mut grid = parse_input(input);
        let mut changed = true;
        let mut count = 0;
        while changed {
            changed = false;
            let removable_positions = grid
                .iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|&(_, &cell)| cell == Cell::Paper)
                        .map(move |(j, _)| (i, j))
                })
                .filter(|&(i, j)| {
                    neighbors(&grid, i, j)
                        .filter(|&neighbor| neighbor == Cell::Paper)
                        .count()
                        < 4
                })
                .collect::<Vec<_>>();
            for (i, j) in removable_positions {
                changed = true;
                count += 1;
                grid[i][j] = Cell::Empty;
            }
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
