use std::{
    collections::HashMap,
    io::{IsTerminal, Read},
    rc::Rc,
};

use advent_of_code::Solution;
use memoize::memoize;

#[derive(Debug)]
struct Graph {
    adj_list: Vec<Vec<usize>>,
    name_to_i: HashMap<String, usize>,
}

fn parse_input(input: &str) -> Graph {
    // Mapping of names to indices
    let mut last_i = 0;
    let mut name_to_i: HashMap<String, usize> = HashMap::from_iter(
        input
            .lines()
            .enumerate()
            .inspect(|(i, _)| last_i = *i)
            .map(|(i, line)| (line.split_once(':').unwrap().0.to_string(), i)),
    );
    // Add out entry to mapping
    name_to_i.insert("out".to_string(), last_i + 1);
    // Construct adjacency list using integer indices instead of names
    let mut adj_list: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            let (_, rest) = line.split_once(':').unwrap();
            rest.trim()
                .split(' ')
                .map(|dst_name| name_to_i[dst_name])
                .collect()
        })
        .collect();
    // Add out entry to adjacency list
    adj_list.push(Vec::new());
    Graph {
        adj_list,
        name_to_i,
    }
}

fn count_all_paths(graph: &[Vec<usize>], start: usize, target: usize) -> u64 {
    if start == target {
        return 1;
    }

    graph[start]
        .iter()
        .copied()
        .map(|next| count_all_paths(graph, next, target))
        .sum::<u64>()
}

#[memoize]
fn count_paths_through(
    graph: Rc<Vec<Vec<usize>>>,
    start: usize,
    target: usize,
    mut needs_1: Option<usize>,
    mut needs_2: Option<usize>,
) -> u64 {
    if needs_1.is_some_and(|i| start == i) {
        needs_1 = None;
    }
    if needs_2.is_some_and(|i| start == i) {
        needs_2 = None;
    }

    if start == target {
        return if needs_1.is_none() && needs_2.is_none() {
            1
        } else {
            0
        };
    }

    graph[start]
        .iter()
        .copied()
        .map(|next| count_paths_through(Rc::clone(&graph), next, target, needs_1, needs_2))
        .sum()
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = u64;

    fn part1(input: &'a str) -> Self::Output {
        let graph = parse_input(input);
        let start = graph.name_to_i["you"];
        let end = graph.name_to_i["out"];
        count_all_paths(&graph.adj_list, start, end)
    }

    fn part2(input: &'a str) -> Self::Output {
        let graph = parse_input(input);
        let start = graph.name_to_i["svr"];
        let end = graph.name_to_i["out"];
        let dac = graph.name_to_i["dac"];
        let fft = graph.name_to_i["fft"];
        count_paths_through(Rc::new(graph.adj_list), start, end, Some(dac), Some(fft))
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
