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
                .map(|(_, r)| r.unfold())
                .collect(),
        }
    }

    fn sum_of_valid_arrangements(self) -> u64 {
        self.springs
            .into_par_iter()
            .map(|r| r.valid_arrangements())
            .sum()
    }
}

#[derive(Debug)]
struct Record {
    statuses: Vec<Status>,
    damaged_counts: Vec<usize>,
}

impl Record {
    fn unfold(self) -> Self {
        let statuses = self.statuses.len();
        let statuses = self
            .statuses
            .into_iter()
            .chain([Status::Unknown])
            .cycle()
            .take(statuses * 5 + 4)
            .collect();

        let damaged_counts = self.damaged_counts.len();
        let damaged_counts = self
            .damaged_counts
            .into_iter()
            .cycle()
            .take(damaged_counts * 5)
            .collect();

        Self {
            statuses,
            damaged_counts,
        }
    }

    fn valid_arrangements(mut self) -> u64 {
        // to make the Damaged recursion case simpler
        self.statuses.push(Status::Operational);
        let mut cache = vec![vec![None; self.statuses.len()]; self.damaged_counts.len()];
        count_possible_arangements_inner(&self.statuses, self.damaged_counts.as_slice(), &mut cache)
    }
}

fn count_possible_arangements_inner(
    statuses: &[Status],
    counts: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if counts.is_empty() {
        return if statuses.contains(&Status::Damaged) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        };
    }
    if statuses.len() < counts.iter().sum::<usize>() + counts.len() {
        // Not enough space for remaining numbers
        return 0;
    }
    if let Some(cached) = cache[counts.len() - 1][statuses.len() - 1] {
        return cached;
    }
    let mut arangements = 0;
    if statuses[0] != Status::Damaged {
        // Assume operational
        arangements += count_possible_arangements_inner(&statuses[1..], counts, cache);
    }
    let next_group_size = counts[0];
    if !statuses[..next_group_size].contains(&Status::Operational)
        && statuses[next_group_size] != Status::Damaged
    {
        // Assume damaged
        arangements +=
            count_possible_arangements_inner(&statuses[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][statuses.len() - 1] = Some(arangements);
    arangements
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
            separated_list1(tag(","), map(complete::u64, |n| n as usize)),
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
