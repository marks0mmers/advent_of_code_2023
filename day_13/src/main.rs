use std::fs;

fn horizontal_reflection_index(pattern: &str, smudge: bool) -> Option<usize> {
    let lines = pattern.lines().collect::<Vec<_>>();
    for (i, pair) in lines.windows(2).enumerate() {
        let (top, bottom) = (pair[0], pair[1]);
        let mut smudge_used = false;
        let is_reflection = match smudge {
            true => {
                let count = top
                    .chars()
                    .zip(bottom.chars())
                    .filter(|(tc, bc)| *tc != *bc)
                    .count();
                if count > 0 {
                    smudge_used = true;
                }
                count <= 1
            }
            false => top == bottom,
        };
        if is_reflection {
            let size = i.min(lines.len() - (i + 2));
            let before = &lines[i - size..i];
            let after = &lines[i + 2..i + 2 + size];
            if before
                .iter()
                .enumerate()
                .all(|(j, row)| match !smudge_used && smudge {
                    true => {
                        let count = row
                            .chars()
                            .zip(after[size - j - 1].chars())
                            .filter(|(tc, bc)| *tc != *bc)
                            .count();
                        if count > 0 {
                            smudge_used = true;
                        }
                        count <= 1
                    }
                    false => *row == after[size - j - 1],
                })
            {
                return Some(i + 1);
            }
        }
    }
    return None;
}

fn vertical_reflection_index(pattern: &str, smudge: bool) -> Option<usize> {
    let width = pattern.chars().take_while(|c| *c != '\n').count();
    for col in 0..width - 1 {
        let mut smudge_used = false;
        let mut is_match = true;
        for line in pattern.lines() {
            let size = col.min(width - (col + 2));
            let before = &line[col..col + 1];
            let after = &line[col + 1..col + 2];
            let is_reflection = match !smudge_used && smudge {
                true => {
                    let count = before
                        .chars()
                        .zip(after.chars())
                        .filter(|(tc, bc)| *tc != *bc)
                        .count();
                    if count > 0 {
                        smudge_used = true;
                    }
                    count <= 1
                }
                false => before == after,
            };
            if is_reflection {
                let before = &line[col - size..col];
                let after = &line[col + 2..col + 2 + size];
                let after = after.chars().rev().collect::<String>();
                is_match = match !smudge_used && smudge {
                    true => {
                        let count = before
                            .chars()
                            .zip(after.chars())
                            .filter(|(tc, bc)| *tc != *bc)
                            .count();
                        if count > 0 {
                            smudge_used = true;
                        }
                        count <= 1
                    }
                    false => before == after,
                };
            } else {
                is_match = false;
                break;
            }
        }
        if is_match {
            return Some(col + 1);
        }
    }
    return None;
}

fn part_1(input: &str, smudge: bool) -> usize {
    let mut total = 0;
    for pattern in input.split("\n\n") {
        if let Some(row) = horizontal_reflection_index(pattern, smudge) {
            if smudge {
                match horizontal_reflection_index(pattern, false) {
                    Some(no_smudge) if no_smudge != row => {
                        total += 100 * row;
                        // println!("{pattern}");
                        // println!("Horizontal (no smudge): {no_smudge}");
                        // println!("Horizontal: {row}\n");
                    }
                    _ => (),
                }
            } else {
                total += 100 * row;
            }
            continue;
        }
        if let Some(col) = vertical_reflection_index(pattern, smudge) {
            if smudge {
                match vertical_reflection_index(pattern, false) {
                    Some(no_smudge) if no_smudge != col => total += col,
                    _ => (),
                }
            } else {
                total += col;
            }
            continue;
        }
    }
    return total;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot open input file");
    println!("Part 1: {}", part_1(&input, false));
    println!("Part 2: {}", part_1(&input, true));
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT, false);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_part_2() {
        let result = part_1(INPUT, true);
        assert_eq!(result, 400);
    }
}
