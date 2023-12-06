use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1},
    combinator,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Race {
    duration: u64,
    record_distance: u64,
}

fn parse_race(input: &str) -> IResult<&str, Race> {
    let values = || {
        delimited(
            multispace1,
            separated_list1(multispace1, complete::u64),
            multispace1,
        )
    };
    use std::fmt::Write;
    let values = || {
        combinator::map_res(values(), |nums: Vec<u64>| {
            nums.into_iter()
                .fold(String::new(), |mut acc, n| {
                    let _ = write!(acc, "{n}");
                    acc
                })
                .parse::<u64>()
        })
    };

    let (input, duration) = preceded(tag("Time:"), values())(input)?;
    let (input, record_distance) = preceded(tag("Distance:"), values())(input)?;

    Ok((
        input,
        Race {
            duration,
            record_distance,
        },
    ))
}

fn main() {
    let input = include_str!("input.txt");
    let (_, race) = parse_race(input).expect("failed to parse races");

    let answer = (0..race.duration)
        .map(|duration| {
            let time_in_race = race.duration - duration;

            time_in_race * duration
        })
        .filter(|distance| *distance > race.record_distance)
        .count();

    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use crate::{parse_race, Race};

    #[test]
    fn test() {
        let test = include_str!("test.txt");
        let (_, race) = parse_race(test).expect("failed to parse races");

        assert_eq!(
            race,
            Race {
                duration: 71530,
                record_distance: 940200
            },
        )
    }
}
