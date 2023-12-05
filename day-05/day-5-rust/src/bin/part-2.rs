use std::{collections::HashMap, ops::Range, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct URange(Range<usize>);

#[derive(Debug, Default)]
struct Almanac {
    seed_to_soil: HashMap<URange, URange>,
    soil_to_fertilizer: HashMap<URange, URange>,
    fertilizer_to_water: HashMap<URange, URange>,
    water_to_light: HashMap<URange, URange>,
    light_to_temperature: HashMap<URange, URange>,
    temperature_to_humidity: HashMap<URange, URange>,
    humidity_to_location: HashMap<URange, URange>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.split("\n\n").collect::<Vec<_>>();

        let seeds = input[0]
            .trim_start_matches("seeds: ")
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0];
                let end = chunk[0] + chunk[1];
                URange(start..end)
            })
            .collect::<Vec<_>>();

        dbg!(&seeds);

        let seed_to_soil = make_map(input[1], "seed-to-soil map:\n", seeds, false);

        let soil_to_fertilizer = make_map(
            input[2],
            "soil-to-fertilizer map:\n",
            seed_to_soil.values().cloned().collect::<Vec<_>>(),
            false,
        );

        let fertilizer_to_water = make_map(
            input[3],
            "fertilizer-to-water map:\n",
            soil_to_fertilizer.values().cloned().collect::<Vec<_>>(),
            false,
        );

        let water_to_light = make_map(
            input[4],
            "water-to-light map:\n",
            fertilizer_to_water.values().cloned().collect::<Vec<_>>(),
            false,
        );

        let light_to_temperature = make_map(
            input[5],
            "light-to-temperature map:\n",
            water_to_light.values().cloned().collect::<Vec<_>>(),
            true,
        );

        let temperature_to_humidity = make_map(
            input[6],
            "temperature-to-humidity map:\n",
            light_to_temperature.values().cloned().collect::<Vec<_>>(),
            false,
        );

        let humidity_to_location = make_map(
            input[7],
            "humidity-to-location map:\n",
            temperature_to_humidity
                .values()
                .cloned()
                .collect::<Vec<_>>(),
            false,
        );

        Ok(Self {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl Almanac {
    fn min_location(&self) -> URange {
        self.humidity_to_location
            .values()
            .reduce(|acc, loc| if loc.0.start <= acc.0.start { loc } else { acc })
            .unwrap()
            .clone()
    }
}

fn make_map(input: &str, header: &str, keys: Vec<URange>, debug: bool) -> HashMap<URange, URange> {
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
        .map(|(dest_start, src_start, range)| {
            keys.iter()
                .map(|key| key.clone())
                .filter(move |key| {
                    let src_range = src_start..(src_start + range);
                    src_range.contains(&key.0.start) && src_range.contains(&key.0.end)
                })
                .filter_map(move |key| {
                    let src_range = src_start..(src_start + range);
                    let dest_range = dest_start..(dest_start + range);

                    if debug {
                        dbg!(&src_range);
                        dbg!(&dest_range);
                    }

                    let diff = key.0.start - src_range.start;

                    let start = dest_range.start + diff;
                    let end = start + (key.0.end - key.0.start);
                    let value = URange(start..end);

                    if dest_range.contains(&value.0.start) {
                        Some((key, value))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    for key in keys {
        map.entry(key.clone()).or_insert(key);
    }

    map
}

fn main() {
    let almanac = include_str!("test.txt");
    let almanac = almanac.parse::<Almanac>().expect("failde to parse almanac");

    dbg!(&almanac);
}
