use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Module {
    t: ModuleType,
    dests: Vec<ModuleName>,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<ModuleName, Pulse>),
    Broadcaster,
}

impl ModuleType {
    fn flip_flop() -> Self {
        Self::FlipFlop(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ModuleName(String);

#[derive(Debug)]
struct Modules(ModuleMap);
type ModuleMap = HashMap<ModuleName, Module>;

impl Modules {
    fn set_conjunctions(&mut self) {
        let mut to_conjunction_dests = Vec::new();

        for (name, module) in self.0.iter() {
            for dest in module.dests.iter() {
                if let Some(ModuleType::Conjunction(_)) = self.0.get(dest).map(|dest| &dest.t) {
                    to_conjunction_dests.push((dest.clone(), name.clone()));
                }
            }
        }

        for (conjunction, input) in to_conjunction_dests {
            let conjunction = self.0.get_mut(&conjunction).unwrap();
            match conjunction.t.clone() {
                ModuleType::Conjunction(mut memory) => {
                    memory.insert(input, Pulse::Low);
                    conjunction.t = ModuleType::Conjunction(memory);
                }
                _ => unreachable!(),
            }
        }
    }

    fn push_button(&mut self) -> (u64, u64) {
        let mut low_pulses = 1;
        let mut high_pulses = 0;

        let broadcaster = self
            .0
            .get(&ModuleName("broadcaster".to_owned()))
            .cloned()
            .unwrap();

        let mut q: VecDeque<(ModuleName, ModuleName, Pulse)> = VecDeque::from_iter(
            broadcaster
                .dests
                .into_iter()
                .map(|dest| (ModuleName("broadcaster".to_owned()), dest, Pulse::Low)),
        );

        while !q.is_empty() {
            let (origin, target, pulse) = q.pop_front().unwrap();

            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }

            if !self.0.contains_key(&target) {
                continue;
            }

            let module = self.0.get_mut(&target).unwrap();

            match module.t.clone() {
                ModuleType::FlipFlop(state) => match pulse {
                    Pulse::Low => {
                        let new_state = !state;
                        module.t = ModuleType::FlipFlop(new_state);

                        let new_pulse = if new_state { Pulse::High } else { Pulse::Low };

                        for dest in module.dests.iter() {
                            q.push_back((target.clone(), dest.clone(), new_pulse.clone()))
                        }
                    }
                    Pulse::High => continue,
                },
                ModuleType::Conjunction(mut memory) => {
                    memory.insert(origin, pulse);

                    let new_pulse = if memory.values().all(|p| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    module.t = ModuleType::Conjunction(memory);

                    for dest in module.dests.iter() {
                        q.push_back((target.clone(), dest.clone(), new_pulse.clone()))
                    }
                }
                ModuleType::Broadcaster => continue,
            }
        }

        (low_pulses, high_pulses)
    }
}

fn parse_module_name(input: &str) -> IResult<&str, ModuleName> {
    map(alpha1, |s: &str| ModuleName(s.to_owned()))(input)
}

fn parse_module(input: &str) -> IResult<&str, (ModuleName, Module)> {
    map(
        separated_pair(
            alt((
                map(tag("broadcaster"), |_| {
                    (
                        ModuleType::Broadcaster,
                        ModuleName("broadcaster".to_owned()),
                    )
                }),
                map(
                    tuple((complete::char('%'), parse_module_name)),
                    |(_, name)| (ModuleType::flip_flop(), name),
                ),
                map(
                    tuple((complete::char('&'), parse_module_name)),
                    |(_, name)| (ModuleType::Conjunction(HashMap::new()), name),
                ),
            )),
            tag(" -> "),
            separated_list1(tag(", "), parse_module_name),
        ),
        |((t, name), dests)| (name, Module { t, dests }),
    )(input)
}

fn parse_modules(input: &str) -> IResult<&str, Modules> {
    map(separated_list1(line_ending, parse_module), |modules| {
        let mut modules = Modules(modules.into_iter().collect());
        modules.set_conjunctions();
        modules
    })(input)
}

fn main() {
    let (_, mut modules) = parse_modules(include_str!("input.txt")).expect("failed to parse");

    let (lows, highs) = (0..1000).fold((0, 0), |(acc_low, acc_high), _| {
        let (lows, highs) = modules.push_button();
        (acc_low + lows, acc_high + highs)
    });

    let answer = lows * highs;

    println!("Answer: {lows} * {highs} = {answer}");
}
