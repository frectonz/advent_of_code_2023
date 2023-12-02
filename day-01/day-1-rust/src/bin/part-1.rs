use std::str::FromStr;

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

impl FromStr for Calibration {
    type Err = CalibrationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CalibrationParseError::*;

        let nums = s
            .chars()
            .filter_map(|ch| ch.to_digit(10))
            .collect::<Vec<_>>();

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
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{Calibration, CalibrationParseError::*};

    #[test]
    fn test_parse() {
        let num = "1abc2".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 12 }));

        let num = "abc".parse::<Calibration>();
        assert_eq!(num, Err(NumbersNotFound));

        let num = "pqr3stu8vwx".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 38 }));

        let num = "treb7uchet".parse::<Calibration>();
        assert_eq!(num, Ok(Calibration { value: 77 }));
    }
}
