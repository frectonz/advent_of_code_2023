use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{ops::Range, str::FromStr};

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.split("\n\n").collect::<Vec<_>>();

        let seeds = input[0]
            .trim_start_matches("seeds: ")
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0];
                let end = chunk[0] + chunk[1];
                start..end
            })
            .collect();

        let mappings = input.into_iter().skip(1).map(Map::make).collect();

        Ok(Self {
            seeds,
            maps: mappings,
        })
    }
}

#[derive(Debug)]
struct Map {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn make(input: &str) -> Self {
        let mappings = input
            .lines()
            .skip(1)
            .map(|line| {
                let mut map = line.split_whitespace().map(|l| l.parse().unwrap());
                (
                    map.next().unwrap(),
                    map.next().unwrap(),
                    map.next().unwrap(),
                )
            })
            .map(|(dest_start, src_start, range)| {
                (
                    src_start..(src_start + range),
                    (dest_start..(dest_start + range)),
                )
            })
            .collect::<Vec<_>>();

        Self { mappings }
    }

    fn translate(&self, src: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(src_range, _)| src_range.contains(&src));

        let Some((src_range, dest_range)) = valid_mapping else {
            return src;
        };

        let offset = src - src_range.start;

        dest_range.start + offset
    }
}

fn main() {
    let almanac = include_str!("input.txt");
    let almanac = almanac.parse::<Almanac>().expect("failde to parse almanac");

    let Almanac { seeds, maps } = almanac;

    let count = seeds.iter().map(|range| range.end - range.start).sum();
    let min_loc = seeds
        .into_par_iter()
        .flat_map(|range| range.clone())
        .progress_count(count)
        .map(|seed| maps.iter().fold(seed, |seed, map| map.translate(seed)))
        .min()
        .unwrap();

    println!("Minimum location: {min_loc}");
}
