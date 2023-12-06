use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Race {
    duration: u32,
    record_distance: u32,
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let values = || {
        delimited(
            multispace1,
            separated_list1(multispace1, complete::u32),
            multispace1,
        )
    };

    let (input, durations) = preceded(tag("Time:"), values())(input)?;
    let (input, record_distances) = preceded(tag("Distance:"), values())(input)?;

    let races = record_distances
        .into_iter()
        .zip(durations)
        .map(|(record_distance, duration)| Race {
            duration,
            record_distance,
        })
        .collect();

    Ok((input, races))
}

fn main() {
    let test = include_str!("input.txt");
    let (_, races) = parse_races(test).expect("failed to parse races");

    let answer = races
        .into_iter()
        .map(|race| {
            (0..race.duration)
                .map(|duration| {
                    let time_in_race = race.duration - duration;

                    time_in_race * duration
                })
                .filter(|distance| *distance > race.record_distance)
                .count()
        })
        .product::<usize>();

    dbg!(answer);
}

#[cfg(test)]
mod tests {
    use crate::{parse_races, Race};

    #[test]
    fn test() {
        let test = include_str!("test.txt");
        let (_, races) = parse_races(test).expect("failed to parse races");

        assert_eq!(
            races,
            vec![
                Race {
                    duration: 7,
                    record_distance: 9
                },
                Race {
                    duration: 15,
                    record_distance: 40
                },
                Race {
                    duration: 30,
                    record_distance: 200
                }
            ]
        )
    }
}
