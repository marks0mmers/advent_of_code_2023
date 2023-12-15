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
    winning_numbers: Vec<u32>,
    possible_numbers: Vec<u32>,
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
                    .parse::<u32>()
                    .expect(format!("Failed to parse number for {}", num_str).as_str())
            })
            .collect::<Vec<_>>();

        let possible_numbers = possible_str
            .split_whitespace()
            .map(|num_str| {
                num_str
                    .parse::<u32>()
                    .expect(format!("Failed to parse number for {}", num_str).as_str())
            })
            .collect::<Vec<_>>();

        Self {
            winning_numbers,
            possible_numbers,
        }
    }

    fn num_matches(&self) -> u32 {
        self.possible_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count() as u32
    }
}

fn part_1() -> u32 {
    let input = fs::read_to_string("input.txt").expect("failed to open input.txt");

    let pile = Pile::from(input.as_str());
    pile.cards
        .iter()
        .map(|card| {
            if card.num_matches() > 0 {
                2u32.pow(card.num_matches() - 1)
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1());
}
