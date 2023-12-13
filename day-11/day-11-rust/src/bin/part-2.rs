use std::collections::{BTreeMap, HashMap};

use petgraph::{algo::astar, stable_graph::NodeIndex, Graph};

use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Point {
    Space,
    Galaxy,
    ExpansionSpace,
}

impl Point {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Point::Space,
            '#' => Point::Galaxy,
            _ => unreachable!("unknown point: {}", c),
        }
    }

    fn is_space(&self) -> bool {
        use Point::*;
        matches!(self, Space | ExpansionSpace)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

type Node = (Pos, Point);
#[derive(Debug)]
struct Image {
    data: BTreeMap<Pos, Point>,
}

impl Image {
    fn from_str(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|line| line.chars().map(Point::from_char).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut new_rows: Vec<Vec<Point>> = Vec::with_capacity(rows.len());
        for row in rows {
            if row.iter().all(|p| p.is_space()) {
                new_rows.push(row.iter().map(|_| Point::ExpansionSpace).collect());
            } else {
                new_rows.push(row);
            }
        }

        let cols = transpose2(new_rows);
        let mut new_cols: Vec<Vec<Point>> = Vec::with_capacity(cols.len());
        for col in cols {
            if col.iter().all(|p| p.is_space()) {
                new_cols.push(col.iter().map(|_| Point::ExpansionSpace).collect());
            } else {
                new_cols.push(col);
            }
        }

        let data = new_cols
            .into_iter()
            .enumerate()
            .flat_map(|(col, column)| {
                column
                    .into_iter()
                    .enumerate()
                    .map(move |(row, p)| (Pos { row, col }, p))
            })
            .collect();

        Self { data }
    }

    fn adjacent_points(&self, (pos, _): Node) -> Vec<Node> {
        let mut adjacents = Vec::with_capacity(4);

        let north = pos.row.checked_sub(1).map(|row| Pos { row, col: pos.col });
        if let Some(north) = north.and_then(|pos| self.data.get(&pos).map(|p| (pos, p.clone()))) {
            adjacents.push(north)
        }

        let south = Pos {
            row: pos.row + 1,
            col: pos.col,
        };
        if let Some(south) = self.data.get(&south).map(move |p| (south, p.clone())) {
            adjacents.push(south)
        }

        let west = pos.col.checked_sub(1).map(|col| Pos { col, row: pos.row });
        if let Some(west) = west.and_then(|pos| self.data.get(&pos).map(|p| (pos, p.clone()))) {
            adjacents.push(west)
        }

        let east = Pos {
            row: pos.row,
            col: pos.col + 1,
        };
        if let Some(east) = self.data.get(&east).map(|p| (east, p.clone())) {
            adjacents.push(east)
        }

        adjacents
    }

    fn to_graph(&self) -> (Graph<Node, usize>, HashMap<(Pos, Point), NodeIndex>) {
        let mut graph = Graph::<Node, usize>::new();
        let mut node_indexes = HashMap::new();

        for curr in self.data.clone().into_iter() {
            let curr_node = *node_indexes
                .entry(curr.clone())
                .or_insert_with(|| graph.add_node(curr.clone()));

            for adj in self.adjacent_points(curr) {
                let adj_node = node_indexes
                    .entry(adj.clone())
                    .or_insert_with(|| graph.add_node(adj.clone()));

                let weight = match adj.1 {
                    Point::ExpansionSpace => 1_000_000,
                    _ => 1,
                };

                graph.add_edge(curr_node, *adj_node, weight);
            }
        }

        (graph, node_indexes)
    }

    fn shortest_path_between_all_galaxies(&self) -> usize {
        let (graph, node_indexes) = self.to_graph();

        let galaxies = self
            .data
            .iter()
            .filter(|(_, point)| **point == Point::Galaxy)
            .collect::<Vec<_>>();

        let sum = galaxies
            .par_iter()
            .map(|(from_pos, from_point)| {
                let from_idx = node_indexes
                    .get(&((**from_pos).clone(), (**from_point).clone()))
                    .unwrap();

                galaxies
                    .par_iter()
                    .map(|(to_pos, to_point)| {
                        let to_idx = node_indexes
                            .get(&((**to_pos).clone(), (**to_point).clone()))
                            .unwrap();
                        if from_pos != to_pos {
                            let (path, _) = astar(
                                &graph,
                                *from_idx,
                                |finish| finish == *to_idx,
                                |e| *e.weight(),
                                |_| 0,
                            )
                            .unwrap();

                            path
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        sum / 2
    }
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
    let input = include_str!("input.txt");
    let image = Image::from_str(input);
    let sum = image.shortest_path_between_all_galaxies();
    dbg!(sum);
}
