use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl Game {
    fn new(line: &str) -> Option<Self> {
        let (id, rest) = line.split_once(":")?;
        let id = id.replace("Game ", "").parse::<u32>().ok()?;
        let turns = rest
            .split(";")
            .filter_map(|segment| Turn::new(segment))
            .collect::<Vec<_>>();
        Some(Self { id, turns })
    }
}

#[derive(Debug)]
struct Turn {
    red: u32,
    blue: u32,
    green: u32,
}

impl Turn {
    fn new(segment: &str) -> Option<Self> {
        let regex = Regex::new(r"([0-9]+) ([a-z]+)").ok()?;
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for (_, [count_str, color]) in regex.captures_iter(segment).map(|c| c.extract()) {
            let count = count_str.parse::<u32>().unwrap();
            match color {
                "red" => red = count,
                "blue" => blue = count,
                "green" => green = count,
                val => panic!("Invalid color, {}", val),
            }
        }
        Some(Self { red, blue, green })
    }
}

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn part_1(games: &Vec<Game>) {
    let valid_games = games
        .iter()
        .filter(|game| {
            game.turns
                .iter()
                .all(|turn| turn.blue <= BLUE && turn.green <= GREEN && turn.red <= RED)
        })
        .collect::<Vec<_>>();

    let game_id_sum: u32 = valid_games.iter().map(|game| game.id).sum();

    println!("Part 1: {}", game_id_sum);
}

fn part_2(games: &Vec<Game>) {
    let fewest_die = games
        .iter()
        .map(|game| {
            let mut result = Turn {
                red: 0,
                blue: 0,
                green: 0,
            };
            for turn in &game.turns {
                result.red = result.red.max(turn.red);
                result.blue = result.blue.max(turn.blue);
                result.green = result.green.max(turn.green);
            }
            return result.red * result.green * result.blue;
        })
        .collect::<Vec<_>>();

    println!("Part 2: {}", fewest_die.iter().sum::<u32>());
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let games = input
        .lines()
        .filter_map(|line| Game::new(line))
        .collect::<Vec<_>>();

    part_1(&games);
    part_2(&games);
}
