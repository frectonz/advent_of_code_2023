use itertools::Itertools;
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
}

impl Pattern {
    fn detect_horizontal_fold(&self) -> Option<usize> {
        let result = self
            .rows
            .iter()
            .enumerate()
            .tuple_windows()
            .filter(|((_, line_a), (_, line_b))| {
                line_a == line_b
                    || line_a
                        .iter()
                        .zip(line_b.iter())
                        .filter(|(a, b)| a != b)
                        .count()
                        <= 1
            })
            .find_map(|((index_a, _), (index_b, _))| {
                let lines_a = self.rows[0..=index_a]
                    .iter()
                    .map(|line| line.iter())
                    .rev();
                let lines_b = self.rows[index_b..].iter().map(|line| line.iter());

                (lines_a
                    .flatten()
                    .zip(lines_b.flatten())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1)
                    .then_some(index_a + 1)
            });

        result.map(|i| i * 100)
    }

    pub fn detect_vertical_fold(&self) -> Option<usize> {
        let mut columns_iter_collection =
            self.rows.iter().map(|line| line.iter()).collect::<Vec<_>>();

        let columns = std::iter::from_fn(move || {
            let mut items = vec![];
            for iter in &mut columns_iter_collection {
                match iter.next() {
                    Some(item) => {
                        items.push(item);
                    }
                    None => return None,
                }
            }
            Some(items)
        })
        .collect::<Vec<Vec<_>>>();

        let result = columns
            .iter()
            .enumerate()
            .tuple_windows()
            .filter(|((_, line_a), (_, line_b))| {
                line_a == line_b
                    || line_a
                        .iter()
                        .zip(line_b.iter())
                        .filter(|(a, b)| a != b)
                        .count()
                        <= 1
            })
            .find_map(|((index_a, _), (index_b, _))| {
                let lines_a = columns[0..=index_a].iter().rev();
                let lines_b = columns[index_b..].iter();

                (lines_a
                    .flatten()
                    .zip(lines_b.flatten())
                    .filter(|(a, b)| a != b)
                    .count()
                    == 1)
                    .then_some(index_a + 1)
            });

        result
    }

    fn mirror_score(&self) -> usize {
        self.detect_horizontal_fold()
            .or(self.detect_vertical_fold())
            .expect("couldn't find reflection")
    }
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    map(separated_list1(line_ending, many1(parse_item)), |rows| {
        Pattern { rows }
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

fn main() {
    let patterns = Patterns::from_str(include_str!("input.txt"));

    println!("Answer: {}", patterns.mirror_score());
}
