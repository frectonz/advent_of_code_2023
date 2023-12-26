use itertools::Itertools;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,

    vx: f64,
    vy: f64,
    vz: f64,

    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new((x, y, z): (f64, f64, f64), (vx, vy, vz): (f64, f64, f64)) -> Self {
        Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
            a: vy,
            b: -vx,
            c: vy * x - vx * y,
        }
    }
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    let sep = || tuple((complete::char(','), space1));
    let nums = || {
        map(
            tuple((complete::i64, sep(), complete::i64, sep(), complete::i64)),
            |(x, _, y, _, z)| (x as f64, y as f64, z as f64),
        )
    };

    map(
        separated_pair(nums(), tuple((space1, complete::char('@'), space1)), nums()),
        |(start, dir)| Hailstone::new(start, dir),
    )(input)
}

fn parse_hailstones(input: &str) -> IResult<&str, Vec<Hailstone>> {
    separated_list1(line_ending, parse_hailstone)(input)
}

fn main() {
    let (_, hailstones) = parse_hailstones(include_str!("input.txt")).unwrap();

    let answer = hailstones
        .into_iter()
        .tuple_combinations()
        .filter(|(hs1, hs2)| hs1.a * hs2.b != hs1.b * hs2.a)
        .map(|(hs1, hs2)| {
            let x = (hs1.c * hs2.b - hs2.c * hs1.b) / (hs1.a * hs2.b - hs2.a * hs1.b);
            let y = (hs2.c * hs1.a - hs1.c * hs2.a) / (hs1.a * hs2.b - hs2.a * hs1.b);

            ((x, y), (hs1, hs2))
        })
        .filter(|((x, y), _)| {
            // let range = (7.)..=27.;
            let range = (200000000000000.)..=400000000000000.;
            range.contains(x) && range.contains(y)
        })
        .filter(|((x, y), (hs1, hs2))| {
            [hs1, hs2]
                .into_iter()
                .all(|hs| (x - hs.x) * hs.vx >= 0. && (y - hs.y) * hs.vy >= 0.)
        })
        .count();

    println!("Answer: {answer}");
}
