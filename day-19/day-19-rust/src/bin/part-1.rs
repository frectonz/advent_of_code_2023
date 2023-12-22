use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct WorkflowName(String);

type Workflows = HashMap<WorkflowName, Workflow>;

#[derive(Debug)]
struct System {
    workflows: Workflows,
    parts: Vec<Part>,
}

impl System {
    fn accepted_parts(&self) -> HashSet<&Part> {
        let mut set = HashSet::new();

        let workflow = self.workflows.get(&WorkflowName("in".to_owned())).unwrap();

        for part in self.parts.iter() {
            let transition = workflow.execute(part, &self.workflows);

            if transition == Transition::Accepted {
                set.insert(part);
            }
        }

        set
    }

    fn sum_rating_of_accepted_parts(&self) -> u32 {
        self.accepted_parts()
            .into_iter()
            .map(|p| p.x + p.m + p.a + p.s)
            .sum()
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    end: Transition,
}

impl Workflow {
    fn execute(&self, part: &Part, workflows: &Workflows) -> Transition {
        use Transition::*;

        for rule in self.rules.iter() {
            match rule.execute(part) {
                Some(Accepted) => return Accepted,
                Some(Rejected) => return Rejected,
                Some(Pointer(name)) => {
                    let workflow = workflows.get(name).unwrap();
                    return workflow.execute(part, workflows);
                }
                None => {}
            };
        }

        match &self.end {
            Accepted => Accepted,
            Rejected => Rejected,
            Pointer(name) => {
                let workflow = workflows.get(name).unwrap();
                workflow.execute(part, workflows)
            }
        }
    }
}

#[derive(Debug)]
struct Rule {
    cond: Condition,
    result: Transition,
}

impl Rule {
    fn execute(&self, part: &Part) -> Option<&Transition> {
        if self.cond.apply(part) {
            Some(&self.result)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Condition {
    left: Rating,
    comp: Comparison,
    right: u32,
}

impl Condition {
    fn apply(&self, part: &Part) -> bool {
        let left = self.left.get(part);
        self.comp.apply(left, self.right)
    }
}

#[derive(Debug)]
enum Rating {
    X,
    M,
    A,
    S,
}
impl Rating {
    fn get(&self, part: &Part) -> u32 {
        use Rating::*;
        match self {
            X => part.x,
            M => part.m,
            A => part.a,
            S => part.s,
        }
    }
}

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan,
}
impl Comparison {
    fn apply(&self, left: u32, right: u32) -> bool {
        use Comparison::*;
        match self {
            LessThan => left < right,
            GreaterThan => left > right,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Transition {
    Pointer(WorkflowName),
    Rejected,
    Accepted,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_system(input: &str) -> IResult<&str, System> {
    map(
        separated_pair(parse_workflows, pair(line_ending, line_ending), parse_parts),
        |(workflows, parts)| System { workflows, parts },
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
                |(rules, end)| Workflow { rules, end },
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
        |(cond, result)| Rule { cond, result },
    )(input)
}

fn parse_cond(input: &str) -> IResult<&str, Condition> {
    use Comparison::*;
    alt((
        map(
            separated_pair(parse_rating, complete::char('<'), complete::u32),
            |(left, right)| Condition {
                left,
                comp: LessThan,
                right,
            },
        ),
        map(
            separated_pair(parse_rating, complete::char('>'), complete::u32),
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

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, parse_part)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    delimited(
        complete::char('{'),
        map(
            tuple((
                parse_key('x'),
                parse_key('m'),
                parse_key('a'),
                parse_key('s'),
            )),
            |(x, m, a, s)| Part { x, m, a, s },
        ),
        complete::char('}'),
    )(input)
}

fn parse_key(c: char) -> impl Fn(&str) -> IResult<&str, u32> {
    move |input: &str| {
        map(
            terminated(
                separated_pair(complete::char(c), complete::char('='), complete::u32),
                opt(complete::char(',')),
            ),
            |(_, n)| n,
        )(input)
    }
}

fn main() {
    let (_, system) = parse_system(include_str!("input.txt")).expect("failed to parse");
    let answer = system.sum_rating_of_accepted_parts();
    println!("Answer: {answer}");
}
