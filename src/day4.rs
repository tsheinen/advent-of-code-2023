#[derive(Debug)]
struct Card {
    id: usize,
    winning: HashSet<isize>,
    actual: HashSet<isize>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = Regex::new(r"Card (?<id>[0-9 ]+): (?<wins>[0-9 ]*?) \| (?<actual>.*)")
            .unwrap()
            .captures(s)
            .unwrap();
        Ok(Self {
            id: cap["id"].trim().parse().unwrap(),
            winning: cap["wins"]
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse().expect("failed to parse"))
                .collect(),
            actual: cap["actual"]
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse().expect("failed to parse"))
                .collect(),
        })
    }
}

impl Card {

    fn wins(&self) -> usize {
        self.winning.intersection(&self.actual).count()
    }

}

use std::{collections::HashSet, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    input.lines().flat_map(Card::from_str).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> String {
    input
        .into_iter()
        .map(Card::wins)
        .filter(|c| *c > 0)
        .map(|c| 2_usize.pow((c - 1) as u32))
        .sum::<usize>()
        .to_string()
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> String {
    fn recurse(all: &[Card], cards: &[Card]) -> usize {
        cards.into_iter().map(|card| {
            let id = card.id;
            recurse(all, &all[id..id+card.wins()])
        }).sum::<usize>() + 1
    }

    (recurse(input, input) - 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )),
            "13"
        );
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day4.txt").trim())),
            "33950"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )),
            "30"
        );
        assert_eq!(
            part2(&parse(include_str!("../input/2023/day4.txt").trim())),
            "14814534"
        );
    }
}
