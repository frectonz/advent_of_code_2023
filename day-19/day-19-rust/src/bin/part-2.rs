use std::{collections::HashMap, ops::RangeInclusive};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::{map, rest},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct WorkflowName(String);

type Workflows = HashMap<WorkflowName, Workflow>;

#[derive(Debug)]
struct System {
    workflows: Workflows,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    end: Transition,
}

impl Workflow {
    fn normalize(&mut self) {
        self.rules.push(Rule::End(self.end.clone()))
    }
}

#[derive(Debug)]
enum Rule {
    Normal { cond: Condition, result: Transition },
    End(Transition),
}

impl Rule {
    fn execute(&self, part: &Part) -> (ApplyResult, &Transition) {
        match self {
            Rule::Normal { cond, result } => (cond.apply(part), &result),
            Rule::End(result) => (ApplyResult::Passed, result),
        }
    }
}

#[derive(Debug)]
struct Condition {
    left: Rating,
    comp: Comparison,
    right: u64,
}

enum ApplyResult {
    Split { pass: Part, fails: Part },
    Passed,
    Failed,
}

impl Condition {
    fn apply(&self, part: &Part) -> ApplyResult {
        let left = part.get(&self.left);

        if left.contains(&self.right) {
            match self.comp {
                Comparison::LessThan => {
                    let new_low = *left.start()..=(self.right - 1);
                    let low = part.with(&self.left, new_low);

                    let new_high = self.right..=*left.end();
                    let high = part.with(&self.left, new_high);

                    ApplyResult::Split {
                        pass: low,
                        fails: high,
                    }
                }
                Comparison::GreaterThan => {
                    let new_low = *left.start()..=self.right;
                    let low = part.with(&self.left, new_low);

                    let new_high = (self.right + 1)..=*left.end();
                    let high = part.with(&self.left, new_high);

                    ApplyResult::Split {
                        pass: high,
                        fails: low,
                    }
                }
            }
        } else {
            if self.comp.apply(left, self.right) {
                ApplyResult::Passed
            } else {
                ApplyResult::Failed
            }
        }
    }
}

fn process_part(part: Part, workflows: &Workflows, next_transition: &Transition) -> u64 {
    use ApplyResult::*;
    use Transition::*;
    match next_transition {
        Pointer(name) => {
            let current_workflow = workflows.get(name).unwrap();

            let mut current_part = part;
            let mut sum = 0;
            for rule in current_workflow.rules.iter() {
                match rule.execute(&current_part) {
                    (Split { pass, fails }, next) => {
                        sum += process_part(pass, workflows, next);
                        current_part = fails;
                    }
                    (Passed, next) => {
                        sum += process_part(current_part.clone(), workflows, next);
                        break;
                    }
                    (Failed, _) => {}
                }
            }

            sum
        }
        Accepted => {
            (part.x.end() - part.x.start() + 1)
                * (part.m.end() - part.m.start() + 1)
                * (part.a.end() - part.a.start() + 1)
                * (part.s.end() - part.s.start() + 1)
        }
        Rejected => 0,
    }
}

#[derive(Debug)]
enum Rating {
    X,
    M,
    A,
    S,
}

impl Part {
    fn get(&self, rating: &Rating) -> &RangeInclusive<u64> {
        use Rating::*;
        match rating {
            X => &self.x,
            M => &self.m,
            A => &self.a,
            S => &self.s,
        }
    }

    fn with(&self, rating: &Rating, range: RangeInclusive<u64>) -> Self {
        use Rating::*;
        let mut new = self.clone();

        match rating {
            X => new.x = range,
            M => new.m = range,
            A => new.a = range,
            S => new.s = range,
        }

        new
    }
}

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan,
}
impl Comparison {
    fn apply(&self, left: &RangeInclusive<u64>, right: u64) -> bool {
        use Comparison::*;
        match self {
            LessThan => left.end() < &right,
            GreaterThan => left.start() > &right,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Transition {
    Pointer(WorkflowName),
    Rejected,
    Accepted,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Part {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

fn parse_system(input: &str) -> IResult<&str, System> {
    map(
        separated_pair(parse_workflows, pair(line_ending, line_ending), rest),
        |(workflows, _)| System { workflows },
    )(input)
}

fn parse_workflows(input: &str) -> IResult<&str, Workflows> {
    map(separated_list1(line_ending, parse_workflow), |workflows| {
        workflows.into_iter().collect()
    })(input)
}

fn parse_workflow(input: &str) -> IResult<&str, (WorkflowName, Workflow)> {
    pair(
        map(alpha1, |n: &str| WorkflowName(n.to_owned())),
        delimited(
            complete::char('{'),
            map(
                separated_pair(parse_rules, complete::char(','), parse_transition),
                |(rules, end)| {
                    let mut w = Workflow { rules, end };
                    w.normalize();
                    w
                },
            ),
            complete::char('}'),
        ),
    )(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(complete::char(','), parse_rule)(input)
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(parse_cond, complete::char(':'), parse_transition),
        |(cond, result)| Rule::Normal { cond, result },
    )(input)
}

fn parse_cond(input: &str) -> IResult<&str, Condition> {
    use Comparison::*;
    alt((
        map(
            separated_pair(parse_rating, complete::char('<'), complete::u64),
            |(left, right)| Condition {
                left,
                comp: LessThan,
                right,
            },
        ),
        map(
            separated_pair(parse_rating, complete::char('>'), complete::u64),
            |(left, right)| Condition {
                left,
                comp: GreaterThan,
                right,
            },
        ),
    ))(input)
}

fn parse_rating(input: &str) -> IResult<&str, Rating> {
    use Rating::*;
    alt((
        map(tag("x"), |_| X),
        map(tag("m"), |_| M),
        map(tag("a"), |_| A),
        map(tag("s"), |_| S),
    ))(input)
}

fn parse_transition(input: &str) -> IResult<&str, Transition> {
    use Transition::*;
    alt((
        map(tag("A"), |_| Accepted),
        map(tag("R"), |_| Rejected),
        map(alpha1, |n: &str| Pointer(WorkflowName(n.to_owned()))),
    ))(input)
}

fn main() {
    let (_, system) = parse_system(include_str!("input.txt")).expect("failed to parse");

    let answer = process_part(
        Part {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
        &system.workflows,
        &Transition::Pointer(WorkflowName("in".to_owned())),
    );

    println!("Answer: {answer}");
}
