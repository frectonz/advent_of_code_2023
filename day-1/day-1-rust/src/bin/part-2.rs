use std::str::FromStr;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map, multi::many0,
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

fn digit_as_words(input: &str) -> IResult<&str, u32> {
    alt((
        map(tag("one"), |_: &str| 1),
        map(tag("two"), |_: &str| 2),
        map(tag("three"), |_: &str| 3),
        map(tag("four"), |_: &str| 4),
        map(tag("five"), |_: &str| 5),
        map(tag("six"), |_: &str| 6),
        map(tag("seven"), |_: &str| 7),
        map(tag("eight"), |_: &str| 8),
        map(tag("nine"), |_: &str| 9),
    ))(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, nums) = many0(alt((
        map(digit_as_words, |n: u32| Some(n)),
        map(anychar, |c: char| match c {
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            _ => None,
        }),
    )))(input)?;

    let nums = nums.into_iter().filter_map(|n| n).collect::<Vec<_>>();

    Ok((input, nums))
}

impl FromStr for Calibration {
    type Err = CalibrationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CalibrationParseError::*;

        let (_, nums) = parse_numbers(s).ok().unwrap();

        if nums.len() == 0 {
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
                    ()
                })
                .map(|Calibration { value }| value)
                .ok()
        })
        .fold(0, |acc, b| acc + b);

    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{Calibration, CalibrationParseError::*};

    #[test]
    fn test_parse() {
        let num = "abc".parse::<Calibration>();
        assert_eq!(num, Err(NumbersNotFound));

        let num = "1abc2".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 12 }));

        let num = "pqr3stu8vwx".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 38 }));

        let num = "a1b2c3d4e5f".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 15 }));

        let num = "treb7uchet".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 77 }));

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
    }
}
