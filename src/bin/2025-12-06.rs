use std::io::{IsTerminal, Read};

use advent_of_code::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: Operation,
}

fn parse_input_1(input: &str) -> Vec<MathProblem> {
    let mut row_wise_matrix: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    for line in input.lines() {
        match line.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                // Number line
                let row = line
                    .split_ascii_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
                row_wise_matrix.push(row);
            }
            Some(_) => {
                // Operation row
                operations = line
                    .split_ascii_whitespace()
                    .map(|c| match c {
                        "+" => Operation::Add,
                        "*" => Operation::Mul,
                        _ => panic!("Invalid operation"),
                    })
                    .collect();
            }
            None => panic!("Empty line"),
        }
    }
    transpose(row_wise_matrix, operations)
}

fn transpose(number_rows: Vec<Vec<u64>>, operations: Vec<Operation>) -> Vec<MathProblem> {
    let mut number_columns: Vec<Vec<u64>> =
        vec![Vec::with_capacity(number_rows.len()); number_rows[0].len()];
    for row in number_rows {
        for (i, num) in row.into_iter().enumerate() {
            number_columns[i].push(num);
        }
    }
    std::iter::zip(number_columns, operations)
        .map(|(numbers, operation)| MathProblem { numbers, operation })
        .collect()
}

fn parse_input_2(input: &str) -> Vec<MathProblem> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut problems = Vec::new();
    let mut numbers = Vec::new();
    for col in (0..grid[0].len()).rev() {
        let mut num = 0;
        let mut op = None;
        for row in &grid {
            match row[col] {
                ' ' => (),
                c @ '0'..='9' => {
                    let digit = c.to_digit(10).unwrap() as u64;
                    num = num * 10 + digit;
                }
                '+' => op = Some(Operation::Add),
                '*' => op = Some(Operation::Mul),
                c => panic!("unexpected character '{c}'"),
            }
        }
        if num != 0 {
            numbers.push(num);
        }
        if let Some(operation) = op {
            let problem = MathProblem {
                numbers: std::mem::take(&mut numbers),
                operation,
            };
            problems.push(problem);
        }
    }
    problems
}

struct Problem;
impl<'a> Solution<'a> for Problem {
    type Output = u64;

    fn part1(input: &'a str) -> Self::Output {
        let mut total = 0;
        for column in parse_input_1(input) {
            let do_op = match column.operation {
                Operation::Add => |a, b| a + b,
                Operation::Mul => |a, b| a * b,
            };
            total += column.numbers.into_iter().reduce(do_op).unwrap_or(0);
        }
        total
    }

    fn part2(input: &'a str) -> Self::Output {
        let mut total = 0;
        for problem in parse_input_2(input) {
            let do_op = match problem.operation {
                Operation::Add => |a, b| a + b,
                Operation::Mul => |a, b| a * b,
            };
            total += problem.numbers.into_iter().reduce(do_op).unwrap_or(0);
        }
        total
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
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";
        let out = Problem::part2(input);
        assert_eq!(3263827, out);
    }
}
