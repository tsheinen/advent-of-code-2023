use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use rayon::prelude::*;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{rebase} {base} {limit}")]
struct Rule {
    base: u64,
    rebase: u64,
    limit: u64
}

impl Rule {
    fn apply(&self, input: u64) -> Option<u64> {
        if input < self.base || input >= self.base + self.limit {
            None
        } else {
            Some(input - self.base + self.rebase)
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Mapping {
    rules: Vec<Rule>
}

impl Mapping {
    fn apply(&self, input: u64) -> u64 {
        self.rules.iter().filter_map(|r| r.apply(input)).next().unwrap_or(input)
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<u64>, Vec<Mapping>) {
    let mut split = input.split("\n\n");
    let seeds = split.next().unwrap().split(": ").nth(1).expect("seed parse").split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let maps = split.map(|block| {
        let mut block_lines = block.lines();
        block_lines.next().unwrap();
        Mapping { rules: block_lines.map(|l| l.parse::<Rule>().expect("rule to parse")).collect() }
    }).collect::<Vec<_>>();
    (seeds, maps)
}

#[aoc(day5, part1)]
fn part1((seeds, mappings): &(Vec<u64>, Vec<Mapping>)) -> String {
    seeds.into_iter().map(|seed| mappings.iter().fold(*seed, |acc, next| next.apply(acc))).min().unwrap().to_string()
}

#[aoc(day5, part2)]
fn part2((seeds, mappings): &(Vec<u64>, Vec<Mapping>)) -> String {
    seeds.array_chunks().map(|[base, length]| (*base..base+length).into_par_iter().map(|seed| mappings.iter().fold(seed, |acc, next| next.apply(acc))).min()).min().flatten().unwrap().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("seeds: 79 14 55 13

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
56 93 4")), "35");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("seeds: 79 14 55 13

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
56 93 4")), "46");
    }
}