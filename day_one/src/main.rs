use std::fs;
use std::io::Error;
use std::str::CharIndices;

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct DigitIter<'a> {
    line: &'a str,
    chars: CharIndices<'a>,
}

impl<'a> DigitIter<'a> {
    fn new(line: &'a str) -> Self {
        Self {
            line,
            chars: line.char_indices(),
        }
    }
}

impl Iterator for DigitIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((index, ch)) = self.chars.next() else {
                break;
            };
            if let Some(digit) = ch.to_digit(10) {
                return Some(digit);
            }
            for (digit, word) in WORDS.iter().enumerate() {
                if self.line[index..].starts_with(word) {
                    return Some((digit + 1) as u32);
                }
            }
        }
        None
    }
}

fn main() -> Result<(), Error> {
    let file = fs::read_to_string("input.txt")?;

    let value = file
        .lines()
        .filter_map(|line| {
            let mut digit_iter = DigitIter::new(line);
            let Some(first) = digit_iter.next() else {
                return None;
            };
            let last = digit_iter.last().unwrap_or(first);
            Some(first * 10 + last)
        })
        .sum::<u32>();

    println!("{}", value);

    Ok(())
}
