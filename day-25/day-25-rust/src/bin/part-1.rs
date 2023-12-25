use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{dot::Dot, Graph, Undirected};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

type Connections = HashSet<(String, String)>;

fn parse_line(input: &str) -> IResult<&str, HashSet<(String, String)>> {
    map(
        separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1)),
        |(start, ends): (&str, Vec<&str>)| {
            ends.into_iter()
                .map(|end| (start.to_owned(), end.to_owned()))
                .collect()
        },
    )(input)
}

fn parse_connections(input: &str) -> IResult<&str, Connections> {
    map(separated_list1(line_ending, parse_line), |lines| {
        lines.into_iter().flatten().collect()
    })(input)
}

fn main() {
    let (_, connections) =
        parse_connections(include_str!("input.txt")).expect("failed to parse file");

    let mut graph = Graph::<String, u32, Undirected>::new_undirected();
    let mut edges = Vec::with_capacity(connections.len());

    let mut node_idxs = HashMap::new();

    for (start, end) in connections {
        edges.push((
            *node_idxs
                .entry(start.to_owned())
                .or_insert_with(|| graph.add_node(start)),
            *node_idxs
                .entry(end.to_owned())
                .or_insert_with(|| graph.add_node(end)),
        ));
    }

    graph.extend_with_edges(edges);

    std::fs::write(
        "original.dot",
        format!(
            "{:?}",
            Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
        ),
    )
    .expect("failed to save graph in dot format");

    let min_cut_res: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    assert_eq!(min_cut, 3);

    std::fs::write(
        "split.dot",
        format!(
            "{:?}",
            Dot::with_attr_getters(
                &graph,
                &[petgraph::dot::Config::EdgeNoLabel],
                &|_, _| String::new(),
                &|_, (idx, _)| if partition.contains(&idx) {
                    "color = red".to_owned()
                } else {
                    String::new()
                }
            )
        ),
    )
    .expect("failed to save graph in dot format");

    let first_split = node_idxs.len() - partition.len();
    let second_split = partition.len();

    let answer = first_split * second_split;

    println!("Answer: {answer}");
}
