use std::fs;

struct Race {
    time: usize,
    dist: usize,
}

impl Race {
    fn get_range_of_winning_times(&self) -> (usize, usize) {
        let min = (0..self.time)
            .find(|time_held| {
                let remaining_time = self.time - time_held;
                time_held * remaining_time > self.dist
            })
            .expect("Race impossible to win");
        let max = (0..self.time)
            .rfind(|time_held| {
                let remaining_time = self.time - time_held;
                time_held * remaining_time > self.dist
            })
            .expect("Race impossible to win");
        (min, max)
    }
}

fn read_races_part_1(input: &String) -> Vec<Race> {
    let (times_str, distances_str) = input.split_once("\n").expect("Invalid File Format");

    let times = times_str
        .split_whitespace()
        .skip(1)
        .map(|num_str| num_str.parse::<usize>().expect("Invalid Number"))
        .collect::<Vec<_>>();
    let distances = distances_str
        .split_whitespace()
        .skip(1)
        .map(|num_str| num_str.parse::<usize>().expect("Invalid Number"))
        .collect::<Vec<_>>();
    times
        .iter()
        .enumerate()
        .map(|(i, time)| Race {
            time: *time,
            dist: *distances.get(i).expect("Distances not same size as times"),
        })
        .collect::<Vec<_>>()
}

fn read_race_part_2(input: &String) -> Race {
    let (time_str, distance_str) = input.split_once("\n").expect("Invalid File Format");

    let time = time_str
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, num| {
            acc.push_str(num);
            acc
        })
        .parse::<usize>()
        .expect("Invalid Number");
    let dist = distance_str
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, num| {
            acc.push_str(num);
            acc
        })
        .parse::<usize>()
        .expect("Invalid Number");

    Race { time, dist }
}

fn part_1(races: &Vec<Race>) -> usize {
    races
        .iter()
        .map(|race| {
            let (low, high) = race.get_range_of_winning_times();
            high + 1 - low
        })
        .product()
}

fn part_2(race: &Race) -> usize {
    let (low, high) = race.get_range_of_winning_times();
    high + 1 - low
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open input");
    let races = read_races_part_1(&input);
    let race = read_race_part_2(&input);
    println!("Part 1: {}", part_1(&races));
    println!("Part 2: {}", part_2(&race));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        let races = read_races_part_1(&INPUT.to_string());
        assert_eq!(part_1(&races), 288)
    }

    #[test]
    fn test_part_2() {
        let race = read_race_part_2(&INPUT.to_string());
        assert_eq!(part_2(&race), 71503)
    }
}
