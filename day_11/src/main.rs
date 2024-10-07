use std::fs;

type Point = (usize, usize);

type Universe = Vec<Point>;

fn parse_universe(input: &str, expand: usize) -> Universe {
    // fill empty vectors w/ indexes so we can lookup values
    let mut empty_cols = Vec::from_iter(0..input.chars().take_while(|c| *c != '\n').count());
    let mut empty_rows = Vec::new();
    for (row, line) in input.lines().enumerate() {
        if !line.contains("#") {
            empty_rows.push(row);
        }
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                if let Some(idx) = empty_cols.iter().position(|ec| *ec == col) {
                    empty_cols.remove(idx);
                }
            }
        }
    }

    let mut galaxies = Vec::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                let num_empty_cols = empty_cols.iter().take_while(|ec| **ec <= col).count();
                let num_empty_rows = empty_rows.iter().take_while(|er| **er <= row).count();
                galaxies.push((
                    col + num_empty_cols * (expand - 1),
                    row + num_empty_rows * (expand - 1),
                ));
            }
        }
    }
    return galaxies;
}

fn steps_between_points(p0: Point, p1: Point) -> usize {
    p0.0.abs_diff(p1.0) + p0.1.abs_diff(p1.1)
}

fn all_distances(input: &str, expand: usize) -> usize {
    let universe = parse_universe(input, expand);
    let mut total = 0;
    for from_idx in 0..universe.len() {
        for to_idx in from_idx + 1..universe.len() {
            let from = universe[from_idx];
            let to = universe[to_idx];
            let steps = steps_between_points(from, to);
            total += steps;
        }
    }
    return total;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot open input file");
    println!("Part 1: {}", all_distances(input.as_str(), 2));
    println!("Part 2: {}", all_distances(input.as_str(), 1_000_000));
}

#[cfg(test)]
mod tests {
    use crate::all_distances;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1() {
        let result = all_distances(INPUT, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_part_2() {
        let result = all_distances(INPUT, 10);
        assert_eq!(result, 1030);
        let result = all_distances(INPUT, 100);
        assert_eq!(result, 8410);
    }
}
