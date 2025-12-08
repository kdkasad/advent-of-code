use std::{
    cmp::Reverse,
    io::{IsTerminal, Read},
};

use advent_of_code::Solution;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    /// Calculates the square of the Euclidean distance between this point and the one given.
    pub fn distance_squared(&self, other: &Point) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Point> {
    input.lines().map(|line| {
        let mut parts = line.split(',').map(|part| part.parse().unwrap());
        Point {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        }
    })
}

/// Constructs a weighted graph from the given point cloud.
/// Returns an edge list.
fn construct_graph(points: &[Point]) -> Vec<(usize, usize, u64)> {
    let mut graph = Vec::with_capacity(points.len().pow(2));
    for i in 1..points.len() {
        for j in 0..i {
            let d2 = points[i].distance_squared(&points[j]);
            graph.push((i, j, d2));
        }
    }
    graph
}

struct DisjointSets {
    entries: Vec<(usize, usize)>,
    size: usize,
}

impl DisjointSets {
    pub fn new(size: usize) -> DisjointSets {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push((i, 1));
        }
        DisjointSets {
            entries: data,
            size,
        }
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let pi = self.find(i);
        let pj = self.find(j);
        if pi != pj {
            let pi_size = self.entries[pi].1;
            let pj_size = self.entries[pj].1;
            if pi_size < pj_size {
                self.entries[pi].0 = pj;
                self.entries[pj].1 += pi_size;
            } else {
                self.entries[pj].0 = pi;
                self.entries[pi].1 += pj_size;
            }
            self.size -= 1
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.entries[i].0 != i {
            self.entries[i].0 = self.find(self.entries[i].0);
        }
        self.entries[i].0
    }
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = u64;

    fn part1(input: &'a str) -> Self::Output {
        let points: Vec<Point> = parse_input(input).collect();
        let mut edges = construct_graph(&points);
        edges.sort_unstable_by_key(|&(_, _, weight)| weight);
        let mut circuits = DisjointSets::new(points.len());
        for &(i, j, _) in &edges[..1000] {
            circuits.union(i, j);
        }
        let mut circuits = circuits.entries;
        circuits.sort_unstable_by_key(|&(_, size)| Reverse(size));
        let result: usize = circuits.into_iter().map(|(_, size)| size).take(3).product();
        result as u64
    }

    fn part2(input: &'a str) -> Self::Output {
        let points: Vec<Point> = parse_input(input).collect();
        let mut edges = construct_graph(&points);
        edges.sort_unstable_by_key(|&(_, _, weight)| weight);
        let mut circuits = DisjointSets::new(points.len());
        let mut edges = edges.into_iter();
        for (i, j, _) in edges.by_ref() {
            circuits.union(i, j);
            if circuits.size == 2 {
                break;
            }
        }
        let next_valid_edge = edges
            .find(|&(i, j, _)| circuits.find(i) != circuits.find(j))
            .unwrap();
        let (i, j, _) = next_valid_edge;
        points[i].x * points[j].x
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
        let input = include_str!("../../inputs/2025-12-08-sample.txt");
        let result = Problem::part1(input);
        assert_eq!(40, result);
    }

    #[test]
    fn part2_sample1() {
        let input = include_str!("../../inputs/2025-12-08-sample.txt");
        let result = Problem::part2(input);
        assert_eq!(25572, result);
    }
}
