use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input.split('\n').map(String::from).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> isize {
    input
        .into_iter()
        .filter_map(|line| {
            Some(
                (line.chars().find(|c| c.is_numeric())? as u8 - 0x30) * 10
                    + (line.chars().rev().find(|c| c.is_numeric())? as u8 - 0x30),
            )
        })
        .map(|x| x as isize)
        .sum()
}

fn map_numbers_to_digits(num: &[u8]) -> isize {
    match num {
        b"zero" => 0,
        b"one" => 1,
        b"two" => 2,
        b"three" => 3,
        b"four" => 4,
        b"five" => 5,
        b"six" => 6,
        b"seven" => 7,
        b"eight" => 8,
        b"nine" => 9,
        x if (*x.into_iter().next().expect("match should be non-empty") as char).is_numeric() => {
            x[0] as isize - 0x30
        }
        e => panic!("invalid number {:?}", e),
    }
}

fn check_needles(line: &[u8], check_func: fn(&[u8], &[u8]) -> bool) -> Option<isize> {
    const NEEDLES: [&[u8]; 20] = [
        b"zero", b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
        b"0", b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9",
    ];

    NEEDLES
        .into_iter()
        .filter(|needle| check_func(line, needle))
        .map(|num| map_numbers_to_digits(num))
        .next()
}
fn scan<'a, T: IntoIterator<Item = &'a [u8]>>(iter: T) -> isize {
    iter.into_iter()
        .filter_map(|haystack| check_needles(haystack, |line, needle| line.starts_with(needle)))
        .next()
        .expect("failed to match forward")
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> isize {
    input
        .into_iter()
        .map(|line| line.as_bytes())
        .filter_map(|line| {
            Some(
                scan((0..line.len()).map(|idx| &line[idx..])) * 10
                    + scan((0..line.len()).rev().map(|idx| &line[idx..])),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            )),
            142
        );
        assert_eq!(
            part1(&parse(include_str!("../input/2023/day1.txt").trim_end())),
            54927
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            )),
            281
        );

        assert_eq!(part2(&parse("eighthree")), 83);
        assert_eq!(part2(&parse("sevenine")), 79);
        assert_eq!(
            part2(&parse(include_str!("../input/2023/day1.txt").trim_end())),
            54581
        );
    }
}
