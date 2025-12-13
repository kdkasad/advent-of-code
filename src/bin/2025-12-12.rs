use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

#[derive(Debug)]
struct Input {
    shapes: Box<[Shape]>,
    regions: Box<[Region]>,
}

type Shape = [[bool; 3]; 3];

#[derive(Debug)]
struct Region {
    width: u64,
    length: u64,
    presents: Box<[u64]>,
}

fn parse_input(input: &str) -> Input {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let shapes = sections
        .iter()
        .copied()
        .take(sections.len() - 1)
        .map(parse_shape)
        .collect::<Vec<Shape>>()
        .into_boxed_slice();
    let regions = parse_regions(sections.last().unwrap());
    Input { shapes, regions }
}

fn parse_shape(input: &str) -> Shape {
    let mut shape = [[false; 3]; 3];
    for (i, line) in input.lines().skip(1).enumerate() {
        for (j, c) in line.chars().enumerate() {
            shape[i][j] = match c {
                '#' => true,
                '.' => false,
                other => panic!("Invalid character in shape: {}", other),
            }
        }
    }
    shape
}

fn parse_regions(input: &str) -> Box<[Region]> {
    let mut regions = Vec::new();
    for line in input.lines() {
        let (size, rest) = line.split_once(": ").unwrap();
        let (width_str, length_str) = size.split_once('x').unwrap();
        let (width, length) = (width_str.parse().unwrap(), length_str.parse().unwrap());
        let presents = rest
            .trim()
            .split(' ')
            .map(|count| count.parse().unwrap())
            .collect::<Vec<u64>>()
            .into_boxed_slice();
        regions.push(Region {
            width,
            length,
            presents,
        });
    }
    regions.into_boxed_slice()
}

fn region_is_impossible(region: &Region, shapes: &[Shape]) -> bool {
    let region_area = region.width * region.length;
    let shapes_area: u64 = region
        .presents
        .iter()
        .enumerate()
        .map(|(i, count)| count * area_of_shape(&shapes[i]) as u64)
        .sum();
    shapes_area > region_area
}

fn region_is_trivially_possible(region: &Region) -> bool {
    let can_fit_shapes = (region.width / 3) * (region.length / 3);
    can_fit_shapes >= region.presents.iter().sum()
}

fn area_of_shape(shape: &Shape) -> u8 {
    shape
        .iter()
        .map(|row| row.iter().filter(|cell| **cell).count() as u8)
        .sum()
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = usize;

    fn part1(input: &'a str) -> Self::Output {
        let input = parse_input(input);
        let impossible = input
            .regions
            .iter()
            .filter(|region| region_is_impossible(region, &input.shapes))
            .count();
        let easy = input
            .regions
            .iter()
            .filter(|region| region_is_trivially_possible(region))
            .count();
        dbg!(input.regions.len(), impossible, easy);
        easy
    }

    fn part2(_: &'a str) -> Self::Output {
        unreachable!("2025-12-12 has no part 2")
    }
}

fn main() {
    if std::io::stdin().is_terminal() {
        eprintln!("Waiting for input...");
    }

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let result = Problem::part1(&input);
    println!("Solution: {}", result);
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::Problem;
    use advent_of_code::Solution as _;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_sample1() {}
}
