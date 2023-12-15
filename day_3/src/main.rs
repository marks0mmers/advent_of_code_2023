use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Gear {
    numbers: Vec<u32>,
}

fn is_symbol_not_period(char: char) -> bool {
    !char.is_alphanumeric() && char != '.'
}

struct Schematic {
    numbers: Vec<u32>,
    gears: HashMap<(usize, usize), Gear>,
}

impl Schematic {
    fn from(input: &str) -> Option<Self> {
        let number_regex = Regex::new(r"([0-9]+)").ok()?;

        let mut numbers: Vec<u32> = Vec::new();
        let mut gears: HashMap<(usize, usize), Gear> = HashMap::new();

        let lines = input.lines().collect::<Vec<_>>();

        for (line_idx, line) in lines.iter().enumerate() {
            for c in number_regex.captures_iter(line) {
                let m = c.get(0).unwrap();
                let m_range = m.range();
                let number = m.as_str().parse::<u32>().unwrap();
                let mut number_pushed = false;

                let before_idx = if m_range.start > 0 {
                    m.range().start - 1
                } else {
                    m.range().start
                };

                let after_idx = m_range.end;
                let word_range = if after_idx == line.len() {
                    before_idx..after_idx
                } else {
                    before_idx..(after_idx + 1)
                };

                // check the entire line above
                if line_idx > 0 {
                    if let Some(above) = lines.get(line_idx - 1) {
                        if above[word_range.clone()]
                            .chars()
                            .any(|c| is_symbol_not_period(c))
                        {
                            for (i, c) in above[word_range.clone()].char_indices() {
                                let col = word_range.clone().nth(i).unwrap();
                                if c == '*' {
                                    if let Some(gear) = gears.get_mut(&(line_idx - 1, col)) {
                                        gear.numbers.push(number)
                                    } else {
                                        gears.insert(
                                            (line_idx - 1, col),
                                            Gear {
                                                numbers: vec![number],
                                            },
                                        );
                                    }
                                }
                            }
                            if !number_pushed {
                                numbers.push(number);
                                number_pushed = true;
                            }
                        }
                    }
                }

                // check the left
                if let Some(left) = line.chars().nth(before_idx) {
                    if is_symbol_not_period(left) {
                        if left == '*' {
                            if let Some(gear) = gears.get_mut(&(line_idx, before_idx)) {
                                gear.numbers.push(number)
                            } else {
                                gears.insert(
                                    (line_idx, before_idx),
                                    Gear {
                                        numbers: vec![number],
                                    },
                                );
                            }
                        }
                        if !number_pushed {
                            numbers.push(number);
                            number_pushed = true;
                        }
                    }
                }

                // check the right
                if let Some(right) = line.chars().nth(after_idx) {
                    if is_symbol_not_period(right) {
                        if right == '*' {
                            if let Some(gear) = gears.get_mut(&(line_idx, after_idx)) {
                                gear.numbers.push(number)
                            } else {
                                gears.insert(
                                    (line_idx, after_idx),
                                    Gear {
                                        numbers: vec![number],
                                    },
                                );
                            }
                        }
                        if !number_pushed {
                            numbers.push(number);
                            number_pushed = true;
                        }
                    }
                }

                // check the entire below
                if let Some(below) = lines.get(line_idx + 1) {
                    if below[word_range.clone()]
                        .chars()
                        .any(|c| is_symbol_not_period(c))
                    {
                        for (i, c) in below[word_range.clone()].char_indices() {
                            let col = word_range.clone().nth(i).unwrap();
                            if c == '*' {
                                if let Some(gear) = gears.get_mut(&(line_idx + 1, col)) {
                                    gear.numbers.push(number)
                                } else {
                                    gears.insert(
                                        (line_idx + 1, col),
                                        Gear {
                                            numbers: vec![number],
                                        },
                                    );
                                }
                            }
                        }
                        if !number_pushed {
                            numbers.push(number);
                        }
                    }
                }
            }
        }

        Some(Self { numbers, gears })
    }
}

fn part_1(schematic: &Schematic) -> u32 {
    schematic.numbers.iter().sum()
}

fn part_2(schematic: &Schematic) -> u32 {
    let gears = &schematic.gears;
    let mut gears_vec = gears.iter().collect::<Vec<_>>();
    gears_vec.sort_by_key(|x| x.0);
    gears
        .values()
        .filter(|g| g.numbers.len() == 2)
        .map(|g| g.numbers.iter().product::<u32>())
        .sum()
}

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let input = input_string.as_str();

    let schematic = Schematic::from(input).unwrap();

    println!("Part 1: {}", part_1(&schematic));
    println!("Part 2: {}", part_2(&schematic));
}
