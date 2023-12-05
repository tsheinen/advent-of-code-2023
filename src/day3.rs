use core::num;
use std::{collections::HashSet, ops::Add};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use regex::Regex;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn adjacent_points(&self) -> HashSet<Point> {
        iproduct!(-1..=1, -1..=1)
            .filter(|(x, y)| !(*x == 0 && *y == 0))
            .map(|(x, y)| Point { x, y })
            .map(|x| self.clone() + x)
            .collect()
    }

    fn is_adjacent(&self, tail: &Point) -> bool {
        (self.x - tail.x).abs() <= 1 && (self.y - tail.y).abs() <= 1
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Symbol {
    symbol: String,
    point: Point,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Number {
    number: isize,
    points: Vec<Point>,
}

struct Board {
    numbers: HashSet<Number>,
    symbols: HashSet<Symbol>,
}

impl Board {
    fn find_numbers_with_adjacent_points(&self) -> HashSet<Number> {
        self.numbers
            .iter()
            .cloned()
            .filter(|num| {
                let adj_set = num
                    .points
                    .iter()
                    .flat_map(|p| p.adjacent_points())
                    .collect::<HashSet<_>>();
                let symbol_points = self.symbols.iter().map(|s| s.point).collect::<HashSet<_>>();
                let adj = adj_set.intersection(&symbol_points).collect::<HashSet<_>>();
                adj.len() >= 1
            })
            .collect()
    }

    fn part_two(&self) -> isize {
        self.symbols
            .iter()
            .cloned()
            .filter(|sym| sym.symbol == "*")
            .filter_map(|sym| {
                let adj_points = sym.point.adjacent_points();
                let numbers = self
                    .numbers
                    .iter()
                    .filter(|num| num.points.iter().any(|p| adj_points.contains(p)))
                    .collect::<Vec<_>>();
                if numbers.len() == 2 {
                    Some(numbers[0].number * numbers[1].number)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Board {
    let number_regex = Regex::new("([0-9]+)").unwrap();
    let symbol_regex = Regex::new("([^0-9.\\s])").unwrap();
    let (numbers, symbols): (Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            (
                number_regex
                    .find_iter(line)
                    .map(|m| Number {
                        number: m.as_str().parse().expect("parse invalid int"),
                        points: (m.start()..m.end())
                            .map(|x| x as isize)
                            .map(|p| Point {
                                y: idx as isize,
                                x: p,
                            })
                            .collect(),
                    })
                    .collect::<Vec<_>>(),
                symbol_regex
                    .find_iter(line)
                    .map(|m| Symbol {
                        symbol: m.as_str().to_string(),
                        point: Point {
                            y: idx as isize,
                            x: m.start() as isize,
                        },
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unzip();
    Board {
        numbers: numbers.into_iter().flatten().collect(),
        symbols: symbols.into_iter().flatten().collect(),
    }
}

#[aoc(day3, part1)]
fn part1(input: &Board) -> String {
    input
        .find_numbers_with_adjacent_points()
        .into_iter()
        .map(|x| x.number)
        .sum::<isize>()
        .to_string()
}

#[aoc(day3, part2)]
fn part2(input: &Board) -> String {
    input.part_two().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points() {
        assert_eq!(
            (Point { x: 2, y: 0 }).adjacent_points(),
            [
                Point { x: 1, y: -1 },
                Point { x: 2, y: -1 },
                Point { x: 3, y: -1 },
                Point { x: 1, y: 0 },
                Point { x: 3, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            "4361"
        );
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day3.txt").trim())),
            "520135"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )),
            "467835"
        );
    }
}
