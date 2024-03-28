use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

#[derive(Eq)]
struct Hand {
    cards: [usize; 5],
    hand_type: usize,
    bid: usize,
}

impl Hand {
    fn _hand_type(cards: &[usize; 5], jokers: bool) -> usize {
        let mut map = cards.iter().copied().fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        });

        if jokers {
            // if jokers reassign in hashmap then match
            if let Some(num_jokers) = map.remove(&9) {
                match map.iter().max_by_key(|(_, &v)| v) {
                    Some(k) => *map.get_mut(&k.0.to_owned()).unwrap() += num_jokers,
                    None => {
                        map.entry(12).or_insert(5);
                    } // all jokers, pick aces
                };
            }
        }

        let frequencies: Vec<usize> = map.iter().map(|(_, &freq)| freq).sorted().collect();
        match frequencies[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1,
        }
    }

    fn _get_card_label(x: char) -> usize {
        let valid_chars = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        valid_chars.iter().position(|&c| c == x).unwrap()
    }
    pub fn new(line: &str) -> Self {
        let (card_str, bid_str) = line.split_once(' ').unwrap();
        let cards = card_str
            .chars()
            .map(Hand::_get_card_label)
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        Hand {
            cards,
            hand_type: Hand::_hand_type(&cards, false),
            bid: bid_str.parse::<usize>().unwrap(),
        }
    }

    pub fn joker_rescore(&mut self) {
        self.hand_type = Hand::_hand_type(&self.cards, true);
        for c in &mut self.cards {
            *c = if *c != 9 { *c + 1 } else { 0 };
        }
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            // we could eliminate this check
            for (a, b) in zip(self.cards, other.cards) {
                if a != b {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (a, b) in zip(self.cards, other.cards) {
                    if a != b {
                        return a.cmp(&b);
                    }
                }
                Ordering::Equal
            }
        }
    }
}

fn camel_cards(data: &str) -> (usize, usize) {
    let mut hands: Vec<Hand> = data.lines().map(Hand::new).sorted().collect();
    let p1 = hands
        .iter()
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1))
        .sum();
    for h in &mut hands {
        h.joker_rescore();
    }
    let p2 = hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(idx, h)| h.bid * (idx + 1))
        .sum();
    (p1, p2)
}

pub fn day07() -> (usize, usize) {
    let data = fs::read_to_string("input.txt").expect("Failed to open file");
    camel_cards(&data)
}

fn main() {
    let (part_1, part_2) = day07();
    println!("Part 1: {}", part_1);
    print!("Part 2: {}", part_2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_input() {
        let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(camel_cards(data), (6440, 5905));
    }

    #[test]
    fn test_ans() {
        assert_eq!(day07(), (255048101, 253718286))
    }

    #[test]
    fn test_hand_type() {
        assert_eq!(Hand::_hand_type(&[2, 3, 2, 3, 3], false), 5);
        assert_eq!(Hand::_hand_type(&[13; 5], false), 7)
    }

    #[test]
    fn test_hand() {
        let h = Hand::new("32T3K 765");
        assert_eq!(h.hand_type, 2)
    }
}
