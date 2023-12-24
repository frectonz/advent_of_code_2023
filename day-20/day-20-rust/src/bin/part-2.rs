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

    fn find_rx(&mut self) -> u64 {
        let (feed_name, feed) = self
            .0
            .iter()
            .find(|(_, module)| module.dests.contains(&ModuleName("rx".to_owned())))
            .map(|(key, value)| (key.clone(), value.clone()))
            .unwrap();

        let mut seen = match feed.t.clone() {
            ModuleType::Conjunction(memory) => memory
                .into_keys()
                .map(|key| (key, 0))
                .collect::<HashMap<ModuleName, u64>>(),
            _ => unreachable!(),
        };
        let mut cycle_lengths: HashMap<ModuleName, u64> = HashMap::new();

        let mut presses = 0;
        'top: loop {
            presses += 1;

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

                if !self.0.contains_key(&target) {
                    continue;
                }

                let module = self.0.get_mut(&target).unwrap();

                if target == feed_name && pulse == Pulse::High {
                    seen.entry(origin.clone()).and_modify(|c| *c += 1);
                    cycle_lengths.entry(origin.clone()).or_insert(presses);

                    if seen.values().all(|x| *x > 0) {
                        let mut min_cycle = 1;
                        for c in cycle_lengths.into_values() {
                            min_cycle = lcm(min_cycle, c);
                        }

                        break 'top min_cycle;
                    }
                }

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
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
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
    let answer = modules.find_rx();
    println!("Answer: {answer}");
}
