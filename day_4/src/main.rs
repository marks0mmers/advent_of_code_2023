use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    fn from(input: &str) -> Self {
        let mut cards: Vec<Card> = Vec::new();

        for line in input.lines() {
            cards.push(Card::from(line));
        }

        Self { cards }
    }
}

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    possible_numbers: HashSet<usize>,
}

impl Card {
    fn from(line: &str) -> Self {
        let (_id_str, rest_of_line) = line.split_once(":").expect("Failed to split line on colon");

        let (winning_str, possible_str) = rest_of_line
            .split_once("|")
            .expect("Failed to split numbers by |");

        let winning_numbers = winning_str
            .split_whitespace()
            .map(|num_str| {
                num_str
                    .parse::<usize>()
                    .expect(format!("Failed to parse number for {}", num_str).as_str())
            })
            .collect::<HashSet<_>>();

        let possible_numbers = possible_str
            .split_whitespace()
            .map(|num_str| {
                num_str
                    .parse::<usize>()
                    .expect(format!("Failed to parse number for {}", num_str).as_str())
            })
            .collect::<HashSet<_>>();

        Self {
            winning_numbers,
            possible_numbers,
        }
    }

    fn num_matches(&self) -> usize {
        self.possible_numbers
            .intersection(&self.winning_numbers)
            .count()
    }
}

fn part_1(pile: &Pile) -> usize {
    pile.cards
        .iter()
        .map(|card| {
            if card.num_matches() > 0 {
                2usize.pow((card.num_matches() - 1) as u32)
            } else {
                0
            }
        })
        .sum()
}

fn part_2(pile: &Pile) -> usize {
    let mut counts = vec![1usize; pile.cards.len()];
    let len = counts.len() - 1;

    for (i, card) in pile.cards.iter().enumerate() {
        let j = if card.num_matches() > len {
            len
        } else {
            card.num_matches() + i
        };

        for k in i + 1..j + 1 {
            counts[k] += counts[i];
        }
    }

    counts.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to open input.txt");

    let pile = Pile::from(input.as_str());

    println!("Part 1: {}", part_1(&pile));
    println!("Part 2: {}", part_2(&pile));
}
