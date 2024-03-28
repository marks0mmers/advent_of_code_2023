use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| {
                    i32::from_str_radix(num_str, 10)
                        .expect(format!("Invalid Input: {} is not a number", num_str).as_str())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn build_iterations(line: Vec<i32>) -> Vec<Vec<i32>> {
    let mut iterations = vec![line.clone()];
    let mut current = line;
    while current.iter().any(|n| *n != 0) {
        let next = current[..current.len() - 1]
            .iter()
            .zip(current[1..].iter())
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();
        iterations.push(next.clone());
        current = next;
    }
    iterations
}

fn next_in_sequence(iterations: Vec<Vec<i32>>) -> i32 {
    let mut next = *iterations
        .last()
        .expect("iterations is empty")
        .last()
        .expect("sequence is empty without reaching 0");
    for current in iterations.iter().rev().skip(1) {
        let last_of_current = current.last().expect("empty current");
        next += last_of_current;
    }
    next
}

fn prev_in_sequence(iterations: Vec<Vec<i32>>) -> i32 {
    let mut next = *iterations
        .last()
        .expect("iterations is empty")
        .first()
        .expect("sequence is empty");
    for current in iterations.iter().rev().skip(1) {
        let first_of_current = current.first().expect("empty current");
        next = -next + first_of_current;
    }
    next
}

fn part_1(input: &str) -> i32 {
    let input = parse_input(input);
    input
        .iter()
        .map(|row| {
            let iterations = build_iterations(row.clone());
            next_in_sequence(iterations)
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let input = parse_input(input);
    input
        .iter()
        .map(|row| {
            let iterations = build_iterations(row.clone());
            prev_in_sequence(iterations)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    println!("Part 1: {}", part_1(input.as_str()));
    println!("Part 2: {}", part_2(input.as_str()));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 2);
    }
}
