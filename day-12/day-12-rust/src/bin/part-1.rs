use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

#[derive(Debug)]
struct Field {
    springs: Vec<Record>,
}

impl Field {
    fn from_str(input: &str) -> Self {
        Self {
            springs: input
                .lines()
                .flat_map(parse_record)
                .map(|(_, r)| r)
                .collect(),
        }
    }

    fn sum_of_valid_arrangements(&self) -> usize {
        self.springs
            .par_iter()
            .map(|r| r.valid_arrangements())
            .sum()
    }
}

#[derive(Debug)]
struct Record {
    statuses: Vec<Status>,
    damaged_counts: Vec<u8>,
}

impl Record {
    fn valid_arrangements(&self) -> usize {
        if let Some(index) = self
            .statuses
            .iter()
            .position(|spring| *spring == Status::Unknown)
        {
            // treat unknown spring as damaged
            let mut as_damaged_spring = self.statuses.clone();
            as_damaged_spring[index] = Status::Damaged;
            let as_damaged = Record {
                statuses: as_damaged_spring,
                damaged_counts: self.damaged_counts.to_owned(),
            };

            // treat unknown spring as operational
            let mut as_operational_spring = self.statuses.clone();
            as_operational_spring[index] = Status::Operational;
            let as_operational = Record {
                statuses: as_operational_spring,
                damaged_counts: self.damaged_counts.to_owned(),
            };

            as_damaged.valid_arrangements() + as_operational.valid_arrangements()
        } else if self.is_valid() {
            1
        } else {
            0
        }
    }

    fn is_valid(&self) -> bool {
        self.statuses
            .iter()
            .group_by(|item| *item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Status::Damaged {
                    Some(group.count() as u8)
                } else {
                    None
                }
            })
            .eq(self.damaged_counts.iter().copied())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    map(
        separated_pair(
            many1(alt((
                map(complete::char('.'), |_| Status::Operational),
                map(complete::char('#'), |_| Status::Damaged),
                map(complete::char('?'), |_| Status::Unknown),
            ))),
            space1,
            separated_list1(tag(","), complete::u8),
        ),
        |(statuses, damaged_spring_groups)| Record {
            statuses,
            damaged_counts: damaged_spring_groups,
        },
    )(input)
}

fn main() {
    let field = Field::from_str(include_str!("input.txt"));
    let arrangements = field.sum_of_valid_arrangements();
    println!("Answer: {arrangements}");
}
