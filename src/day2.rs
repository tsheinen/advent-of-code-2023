use std::{collections::HashMap, ops::{Add, BitOr}, str::FromStr};

use aoc_parse::{parser, prelude::*};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[repr(usize)]
enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
struct ColorSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl Add for ColorSet {
    type Output = ColorSet;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl BitOr for ColorSet {
    type Output = ColorSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }
}


impl FromStr for ColorSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(", ")
            .map(|color| {
                color
                    .split_once(" ")
                    .expect("valid split_once for count color")
            })
            .map(|(count, color)| {
                (
                    count.parse::<usize>().expect("count should be valid"),
                    color,
                )
            })
            .map(|(count, color)| match color {
                "red" => ColorSet {
                    red: count,
                    ..Default::default()
                },
                "green" => ColorSet {
                    green: count,
                    ..Default::default()
                },
                "blue" => ColorSet {
                    blue: count,
                    ..Default::default()
                },
                e => panic!("invalid color {:?}", e),
            })
            .fold(ColorSet::default(), |sum, next| sum + next))
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green,
            x => panic!("invalid color {:?}", x),
        })
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    shown: Vec<ColorSet>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = Regex::new("Game (?<id>[0-9]+): (?<rest>.*)")
            .unwrap()
            .captures(s)
        else {
            panic!("invalid haystack: {:?}", s)
        };
        Ok(Game {
            id: captures["id"].parse().unwrap(),
            shown: captures["rest"]
                .split("; ")
                .map(|round| round.parse().unwrap())
                .collect(),
        })
    }
}

impl Game {
    fn is_possible_part_1(&self) -> bool {
        self.shown.iter().all(|ColorSet { red, blue, green }| {
            *red <= 12 && *blue <= 14 && *green <= 13
        })
    }

    fn minimum_cubes_part_2(&self) -> usize {
        let maximized = self.shown.iter().copied().reduce(|sum, next| sum | next).expect("game didnt contain at least colorset");
        maximized.red * maximized.blue * maximized.green

    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(Game::from_str)
        .map(|g| g.expect("game is invalid"))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> String {
    input
        .into_iter()
        .filter(|game| game.is_possible_part_1())
        .map(|game| game.id)
        .sum::<usize>()
        .to_string()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> String {
    input
    .into_iter()
    .map(|game| game.minimum_cubes_part_2())
    .sum::<usize>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            "8"
        );
        assert_eq!(part1(&parse(include_str!("../input/2023/day2.txt").trim_end())), "2810");
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            "2286"
        );
        assert_eq!(part2(&parse(include_str!("../input/2023/day2.txt").trim_end())), "69110");

    }
}
