use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::anychar,
    combinator::{map, map_opt},
    multi::many0,
    IResult,
};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
struct Calibration {
    value: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
enum CalibrationParseError {
    #[error("could not find numbers")]
    NumbersNotFound,
}

fn digit_as_words(input: &str) -> IResult<&str, Vec<u8>> {
    alt((
        // 1
        map(tag("oneight"), |_: &str| vec![1, 8]),
        // 2
        map(tag("twone"), |_: &str| vec![2, 1]),
        // 3
        map(tag("threeight"), |_: &str| vec![3, 8]),
        // 4
        // -
        // 5
        map(tag("fiveight"), |_: &str| vec![5, 8]),
        // 6
        // -
        // 7
        map(tag("sevenine"), |_: &str| vec![7, 9]),
        // 8
        map(tag("eighthree"), |_: &str| vec![8, 3]),
        map(tag("eightwo"), |_: &str| vec![8, 2]),
        // 9
        map(tag("nineight"), |_: &str| vec![9, 8]),
        // regular check
        map(tag("one"), |_: &str| vec![1]),
        map(tag("two"), |_: &str| vec![2]),
        map(tag("three"), |_: &str| vec![3]),
        map(tag("four"), |_: &str| vec![4]),
        map(tag("five"), |_: &str| vec![5]),
        map(tag("six"), |_: &str| vec![6]),
        map(tag("seven"), |_: &str| vec![7]),
        map(tag("eight"), |_: &str| vec![8]),
        map(tag("nine"), |_: &str| vec![9]),
    ))(input)
}

fn char_to_digit(c: &str) -> Option<u8> {
    match c {
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        _ => None,
    }
}

fn digit(input: &str) -> IResult<&str, u8> {
    map_opt(take(1usize), char_to_digit)(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, nums) = many0(alt((
        map(digit_as_words, |n: Vec<u8>| Some(n)),
        map(digit, |n: u8| Some(vec![n])),
        map(anychar, |_| None),
    )))(input)?;

    let nums = nums
        .into_iter()
        .flat_map(|nums| nums.into_iter())
        .flatten()
        .collect::<Vec<_>>();

    Ok((input, nums))
}

impl FromStr for Calibration {
    type Err = CalibrationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CalibrationParseError::*;

        let (_, nums) = parse_numbers(s).ok().unwrap();

        if nums.is_empty() {
            return Err(NumbersNotFound);
        }

        let (start, end) = if nums.len() == 1 {
            (nums.first().unwrap(), nums.first().unwrap())
        } else {
            (nums.first().unwrap(), nums.last().unwrap())
        };

        let value = format!("{start}{end}").parse::<usize>().unwrap();

        Ok(Self { value })
    }
}

fn main() {
    let input = include_str!("input.txt");

    let sum = input
        .lines()
        .filter_map(|line| {
            line.parse::<Calibration>()
                .map_err(|e| {
                    println!("error `{e}` parsing `{line}`");
                })
                .map(|Calibration { value }| value)
                .ok()
        })
        .sum::<usize>();

    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{Calibration, CalibrationParseError::*};

    #[test]
    fn test_parse() {
        let num = "abc".parse::<Calibration>();
        assert_eq!(num, Err(NumbersNotFound));

        // part 1 examples
        let num = "1abc2".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 12 }));

        let num = "pqr3stu8vwx".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 38 }));

        let num = "a1b2c3d4e5f".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 15 }));

        let num = "treb7uchet".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 77 }));

        // part 2 examples
        let num = "two1nine".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 29 }));

        let num = "eightwothree".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 83 }));

        let num = "abcone2threexyz".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 13 }));

        let num = "xtwone3four".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 24 }));

        let num = "4nineeightseven2".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 42 }));

        let num = "zoneight234".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 14 }));

        let num = "7pqrstsixteen".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 76 }));

        let num = "13".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 13 }));

        let num = "nnnineon7nnine".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 99 }));
    }
}
