use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Ash,
    Rock,
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((
        map(complete::char('.'), |_| Item::Ash),
        map(complete::char('#'), |_| Item::Rock),
    ))(input)
}

#[derive(Debug)]
struct Pattern {
    rows: Vec<Vec<Item>>,
    cols: Vec<Vec<Item>>,
}

impl Pattern {
    fn mirror_score(&self) -> usize {
        let mirror_point = |list2d: &Vec<Vec<Item>>| {
            list2d
                .iter()
                .enumerate()
                .find(|(i, _)| {
                    let (left, right) = list2d.split_at(*i);
                    let mut left = left.to_owned();
                    left.reverse();

                    if left.is_empty() || right.is_empty() {
                        false
                    } else {
                        left.into_iter().zip(right).all(|(l, r)| l == *r)
                    }
                })
                .map(|(i, _)| i)
        };

        let row = mirror_point(&self.rows).map(|i| i * 100);
        let col = mirror_point(&self.cols);

        row.or(col).expect("couldn;'t find mirroring col or row")
    }
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    map(separated_list1(line_ending, many1(parse_item)), |rows| {
        Pattern {
            cols: transpose2(rows.clone()),
            rows,
        }
    })(input)
}

#[derive(Debug)]
struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    fn from_str(input: &str) -> Self {
        parse_patterns(input).expect("failed to parse").1
    }

    fn mirror_score(&self) -> usize {
        self.patterns.iter().map(Pattern::mirror_score).sum()
    }
}

fn parse_patterns(input: &str) -> IResult<&str, Patterns> {
    map(
        separated_list1(pair(line_ending, line_ending), parse_pattern),
        |patterns| Patterns { patterns },
    )(input)
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() {
    let patterns = Patterns::from_str(include_str!("input.txt"));
    println!("Answer: {}", patterns.mirror_score());
}
