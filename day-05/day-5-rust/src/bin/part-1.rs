use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default)]
struct Almanac {
    humidity_to_location: HashMap<usize, usize>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.split("\n\n").collect::<Vec<_>>();

        let seeds = input[0]
            .trim_start_matches("seeds: ")
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let seed_to_soil = make_map(input[1], "seed-to-soil map:\n", seeds);

        let soil_to_fertilizer = make_map(
            input[2],
            "soil-to-fertilizer map:\n",
            seed_to_soil.values().copied().collect::<Vec<_>>(),
        );

        let fertilizer_to_water = make_map(
            input[3],
            "fertilizer-to-water map:\n",
            soil_to_fertilizer.values().copied().collect::<Vec<_>>(),
        );

        let water_to_light = make_map(
            input[4],
            "water-to-light map:\n",
            fertilizer_to_water.values().copied().collect::<Vec<_>>(),
        );

        let light_to_temperature = make_map(
            input[5],
            "light-to-temperature map:\n",
            water_to_light.values().copied().collect::<Vec<_>>(),
        );

        let temperature_to_humidity = make_map(
            input[6],
            "temperature-to-humidity map:\n",
            light_to_temperature.values().copied().collect::<Vec<_>>(),
        );

        let humidity_to_location = make_map(
            input[7],
            "humidity-to-location map:\n",
            temperature_to_humidity
                .values()
                .copied()
                .collect::<Vec<_>>(),
        );

        Ok(Self {
            humidity_to_location,
        })
    }
}

impl Almanac {
    fn min_location(&self) -> usize {
        *self.humidity_to_location.values().min().unwrap()
    }
}

fn make_map(input: &str, header: &str, keys: Vec<usize>) -> HashMap<usize, usize> {
    let mut map = input
        .trim_start_matches(header)
        .lines()
        .map(|line| {
            let mut map = line.split_whitespace().map(|l| l.parse::<usize>().unwrap());
            (
                map.next().unwrap(),
                map.next().unwrap(),
                map.next().unwrap(),
            )
        })
        .flat_map(|(dest_start, src_start, range)| {
            let src_range = src_start..(src_start + range);
            let dest_range = dest_start..(dest_start + range);

            keys.iter()
                .copied()
                .filter(move |key| src_range.contains(key))
                .filter_map(move |key| {
                    let value = dest_start + (key - src_start);

                    if dest_range.contains(&value) {
                        Some((key, value))
                    } else {
                        None
                    }
                })
        })
        .filter(|(key, _)| keys.contains(key))
        .collect::<HashMap<_, _>>();

    for key in keys {
        map.entry(key).or_insert(key);
    }

    map
}

fn main() {
    let almanac = include_str!("input.txt");
    let almanac = almanac.parse::<Almanac>().expect("failde to parse almanac");

    dbg!(almanac.min_location());
}
