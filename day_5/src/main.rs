use std::cmp::min;
use std::fs;

struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn from(input: &str) -> Self {
        let seeds_str = input.lines().nth(0).expect("Invalid input");
        let seeds = seeds_str
            .split(":")
            .nth(1)
            .expect("Invalid seed str")
            .split_whitespace()
            .map(|seed_str| {
                seed_str
                    .parse::<usize>()
                    .expect(format!("Invalid number {}", seed_str).as_str())
            })
            .collect::<Vec<_>>();

        let mappings = input
            .split("\n\n")
            .skip(1)
            .map(|mapping_section| Mapping::from(mapping_section.lines().collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        Self { seeds, mappings }
    }

    fn trace_through_for_seeds(&self) -> Vec<usize> {
        let mut mapping = self
            .mappings
            .iter()
            .find(|m| m.from == "seed")
            .expect("Unable to find seed mapping");
        let mut transformed = self
            .seeds
            .iter()
            .map(|seed| mapping.get_dest(seed))
            .collect::<Vec<_>>();

        while let Some(next_mapping) = self.mappings.iter().find(|m| m.from == mapping.to) {
            mapping = next_mapping;
            transformed = transformed
                .iter()
                .map(|step| mapping.get_dest(step))
                .collect();
        }

        transformed
    }

    fn trace_through_seed_pairs(&self) -> Vec<usize> {
        let mut mapping = self
            .mappings
            .iter()
            .find(|m| m.from == "seed")
            .expect("Unable to find seed mapping");
        let mut transformed = self
            .get_seed_ranges()
            .iter()
            .flat_map(|seed| mapping.get_dest_of_pair(seed))
            .collect::<Vec<_>>();

        while let Some(next_mapping) = self.mappings.iter().find(|m| m.from == mapping.to) {
            mapping = next_mapping;
            transformed = transformed
                .iter()
                .flat_map(|step| mapping.get_dest_of_pair(step))
                .collect();
        }

        transformed.iter().map(|pair| pair.0).collect::<Vec<_>>()
    }

    fn get_seed_ranges(&self) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();
        for i in (0..self.seeds.len()).step_by(2) {
            let left = *self.seeds.get(i).expect("Invalid index");
            let right = *self.seeds.get(i + 1).expect("Invalid next index");
            ret.push((left, right));
        }
        ret
    }
}

struct Mapping {
    from: String,
    to: String,
    rows: Vec<MappingRow>,
}

impl Mapping {
    fn from(lines: Vec<&str>) -> Self {
        let (header, lines) = lines.split_first().expect("Lines for mapping are empty");

        let [from, _, to] = header
            .split_whitespace()
            .nth(0)
            .expect(format!("Invalid mapping header, {}", header).as_str())
            .split("-")
            .collect::<Vec<_>>()[..]
        else {
            panic!("Invalid mapping header, {}", header)
        };
        let from = from.to_string();
        let to = to.to_string();

        let rows = lines
            .iter()
            .map(|line| MappingRow::from(line))
            .collect::<Vec<_>>();

        Self { from, to, rows }
    }

    fn get_dest(&self, target: &usize) -> usize {
        let row = self
            .rows
            .iter()
            .find(|row| target >= &row.source_low && target < &row.source_high);

        match row {
            Some(row) => row.dest_low + target - row.source_low,
            None => *target,
        }
    }

    fn get_dest_of_pair(&self, target: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();

        let mut other_values_to_test = Vec::new();

        let (target_start, target_len) = *target;
        let target_end = target_start + target_len;
        let row = self
            .rows
            .iter()
            .find(|row| target_start <= row.source_high && row.source_low <= target_end);
        match row {
            Some(row) => {
                if target_start < row.source_low {
                    other_values_to_test.push((target_start, &row.source_low - target_start - 1));
                }
                if target_end > row.source_high {
                    other_values_to_test
                        .push((row.source_high + 1, target_end - row.source_high - 1))
                }

                let (offset, overlap_start) = if row.source_low < target_start {
                    (target_start - row.source_low, target_start)
                } else {
                    (0, row.source_low)
                };
                let overlap_range = min(target_end, row.source_high) - overlap_start;
                ret.push((row.dest_low + offset, overlap_range));
            }
            None => ret.push((target_start, target_len)),
        }

        ret.append(
            &mut other_values_to_test
                .iter()
                .flat_map(|v| self.get_dest_of_pair(v))
                .collect::<Vec<_>>(),
        );

        ret
    }
}

struct MappingRow {
    source_low: usize,
    source_high: usize,
    dest_low: usize,
}

impl MappingRow {
    fn from(line: &str) -> Self {
        let [dest_start, src_start, len] = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().expect("Failed to parse number"))
            .collect::<Vec<_>>()[..]
        else {
            panic!("Line doesn't contain 3 numbers")
        };

        Self {
            source_low: src_start,
            source_high: src_start + len,
            dest_low: dest_start,
        }
    }
}

fn part_1(almanac: &Almanac) -> usize {
    let transformed = almanac.trace_through_for_seeds();
    *transformed.iter().min().expect("Seed list was empty")
}

fn part_2(almanac: &mut Almanac) -> usize {
    let transformed = almanac.trace_through_seed_pairs();
    *transformed.iter().min().expect("Seed list was empty")
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to open input");
    let mut almanac = Almanac::from(input.as_str());
    println!("Part 1: {}", part_1(&almanac));
    println!("Part 2: {}", part_2(&mut almanac));
}

#[cfg(test)]
mod tests {
    use crate::Almanac;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part_1() {
        let almanac = Almanac::from(INPUT);
        let transformed_values = almanac.trace_through_for_seeds();
        assert_eq!(transformed_values.iter().min().unwrap(), &35);
    }

    #[test]
    fn test_part_2() {
        let almanac = Almanac::from(INPUT);
        let transformed_values = almanac.trace_through_seed_pairs();
        assert_eq!(transformed_values.iter().min().unwrap(), &46);
    }
}
