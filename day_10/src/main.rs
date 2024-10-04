use std::{collections::HashMap, fs};

type Point = (i32, i32);

#[derive(Debug)]
enum Tile {
    PipeNS,
    PipeEW,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    Start,
}

impl Tile {
    pub fn contains_char(&self, c: char) -> bool {
        let name = match self {
            Tile::BendNE => "NE",
            Tile::BendNW => "NW",
            Tile::BendSE => "SE",
            Tile::BendSW => "SW",
            Tile::PipeEW => "EW",
            Tile::PipeNS => "NS",
            Tile::Start => "",
        };

        name.contains(c)
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Stop,
}

impl Direction {
    pub fn from(self, loc: &Point) -> Point {
        match self {
            Direction::North => (loc.0, loc.1 - 1),
            Direction::East => (loc.0 + 1, loc.1),
            Direction::South => (loc.0, loc.1 + 1),
            Direction::West => (loc.0 - 1, loc.1),
            Direction::Stop => (loc.0, loc.1),
        }
    }
}

trait Area {
    fn shoestring(&self) -> usize;
}

impl Area for Vec<Point> {
    fn shoestring(&self) -> usize {
        (self.windows(2).fold(0, |acc, matrix| {
            acc + (matrix[0].0 * matrix[1].1) - (matrix[1].0 * matrix[0].1)
        }) / 2) as usize
    }
}

trait Interior {
    fn picks(&self, boundary: usize) -> usize;
}

impl Interior for usize {
    fn picks(&self, boundary: usize) -> usize {
        self + 1 - boundary / 2
    }
}

struct PipeMap {
    pipes: HashMap<Point, Tile>,
    start: Point,
}

impl PipeMap {
    fn new(input: &str) -> Self {
        let mut start = (0, 0);
        let mut pipes = HashMap::new();
        input.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                let point = (col as i32, row as i32);
                let tile = match c {
                    'L' => Tile::BendNE,
                    'J' => Tile::BendNW,
                    'F' => Tile::BendSE,
                    '7' => Tile::BendSW,
                    '-' => Tile::PipeEW,
                    '|' => Tile::PipeNS,
                    'S' => {
                        start = point;
                        Tile::Start
                    }
                    _ => return,
                };
                pipes.insert(point, tile);
            })
        });
        Self { pipes, start }
    }

    fn find_next(&self, current: Point, last: Option<Direction>) -> Point {
        let mut next = None;

        if last.is_none() {
            let check = &[
                (current.0, current.1 - 1, 'S'),
                (current.0 + 1, current.1, 'W'),
                (current.0, current.1 + 1, 'N'),
                (current.0 - 1, current.1, 'E'),
            ];

            for &(col, row, dir) in check {
                if let Some(tile) = self.pipes.get(&(col, row)) {
                    if tile.contains_char(dir) {
                        next = Some((col, row));
                        break;
                    }
                }
            }
        } else {
            next = match self.pipes.get(&current).unwrap() {
                Tile::BendNE => match last.unwrap() {
                    Direction::South => Some(Direction::East.from(&current)),
                    Direction::West => Some(Direction::North.from(&current)),
                    _ => panic!("Invalid move."),
                },
                Tile::BendNW => match last.unwrap() {
                    Direction::South => Some(Direction::West.from(&current)),
                    Direction::East => Some(Direction::North.from(&current)),
                    _ => panic!("Invalid move."),
                },
                Tile::BendSE => match last.unwrap() {
                    Direction::North => Some(Direction::East.from(&current)),
                    Direction::West => Some(Direction::South.from(&current)),
                    _ => panic!("Invalid move."),
                },
                Tile::BendSW => match last.unwrap() {
                    Direction::North => Some(Direction::West.from(&current)),
                    Direction::East => Some(Direction::South.from(&current)),
                    _ => panic!("Invalid move."),
                },
                Tile::PipeEW => match last.unwrap() {
                    Direction::East => Some(Direction::East.from(&current)),
                    Direction::West => Some(Direction::West.from(&current)),
                    _ => panic!("Invalid move."),
                },
                Tile::PipeNS => match last.unwrap() {
                    Direction::North => Some(Direction::North.from(&current)),
                    Direction::South => Some(Direction::South.from(&current)),
                    _ => panic!("Invalid move."),
                },
                Tile::Start => Some(Direction::Stop.from(&current)),
            };
        }

        next.unwrap()
    }
}

fn part_1(input: &str) -> usize {
    let map = PipeMap::new(input);
    let mut last_move = None;
    let mut current = map.start;
    let mut visited: Vec<Point> = Vec::new();

    loop {
        let next = map.find_next(current, last_move);
        visited.push(next);
        let delta = (next.0 - current.0, next.1 - current.1);
        last_move = match delta {
            (0, -1) => Some(Direction::North),
            (1, 0) => Some(Direction::East),
            (0, 1) => Some(Direction::South),
            (-1, 0) => Some(Direction::West),
            (0, 0) => break,
            _ => panic!("Invalid move {:?}.", delta),
        };
        current = next;
    }

    visited.len() / 2
}

fn part_2(input: &str) -> usize {
    let map = PipeMap::new(input);
    let mut last_move = None;
    let mut current = map.start;
    let mut visited: Vec<Point> = vec![map.start];

    loop {
        let next = map.find_next(current, last_move);

        visited.push(next);
        let delta = (next.0 - current.0, next.1 - current.1);
        last_move = match delta {
            (0, -1) => Some(Direction::North),
            (1, 0) => Some(Direction::East),
            (0, 1) => Some(Direction::South),
            (-1, 0) => Some(Direction::West),
            (0, 0) => break,
            _ => panic!("Invalid move {:?}.", delta),
        };
        current = next;
    }
    visited.shoestring().picks(visited.len() - 1)
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot open input file");
    println!("Part 1: {}", part_1(input.as_str()));
    println!("Part 2: {}", part_2(input.as_str()));
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    const INPUT_PART_1: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT_PART_2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT_PART_1);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT_PART_2);
        assert_eq!(result, 10);
        let res2 = part_2(INPUT_PART_1);
        assert_eq!(res2, 1);
    }
}
