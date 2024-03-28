use num::integer::gcd;
use std::collections::HashMap;
use std::fs;

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_input<'a>(input: &'a str) -> (&str, HashMap<&str, Node<'a>>) {
    let mut map = HashMap::new();
    let (directions, input) = input.split_once("\n\n").expect("Invalid input file");

    for line in input.lines() {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(key, Node { left, right });
    }

    (directions, map)
}

fn get_cycle_length(
    starting: &str,
    directions: &str,
    map: &HashMap<&str, Node>,
    p1: bool,
) -> usize {
    let mut turns = 0usize;
    let mut current = starting;
    while (p1 && current != "ZZZ") || (!p1 && !current.ends_with("Z")) {
        let direction = directions.as_bytes()[turns % directions.len()] as char;
        let node = map
            .get(current)
            .expect(&*format!("No entry for key: {}", current));
        current = match direction {
            'L' => node.left,
            'R' => node.right,
            _ => panic!("Invalid Direction"),
        };
        turns += 1
    }

    turns
}

fn part_1(input: &str) -> usize {
    let (directions, map) = parse_input(input);

    get_cycle_length("AAA", directions, &map, true)
}

fn part_2(input: &str) -> usize {
    let (directions, map) = parse_input(input);

    let all_starting = map
        .keys()
        .filter_map(|key| if key.ends_with("A") { Some(*key) } else { None })
        .collect::<Vec<_>>();

    let cycles = all_starting
        .iter()
        .map(|starting| get_cycle_length(starting, directions, &map, false))
        .collect::<Vec<_>>();

    lcm(&cycles)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input");
    println!("Part 1: {}", part_1(&*input));
    println!("Part 2: {}", part_2(&*input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 6)
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 6)
    }
}
