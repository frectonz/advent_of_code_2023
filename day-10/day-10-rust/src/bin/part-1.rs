use std::collections::{BTreeMap, HashMap};

use petgraph::{algo::maximum_matching, dot, Graph};

#[derive(Debug)]
struct Grid {
    tiles: BTreeMap<(usize, usize), Tile>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let tiles = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .map(Tile::from_char)
                    .enumerate()
                    .map(move |(col, tile)| ((row, col), tile))
            })
            .flatten()
            .collect::<BTreeMap<_, _>>();

        Self { tiles }
    }

    // fn find_start(&self) -> (usize, usize) {
    //     for ((row, col), tile) in self.tiles.iter() {
    //         match tile {
    //             Tile::Start => return (*row, *col),
    //             _ => continue,
    //         }
    //     }

    //     unreachable!()
    // }

    fn adjacent_tiles(
        &self,
        ((row, col), curr): ((usize, usize), Tile),
    ) -> Vec<((Option<usize>, Option<usize>), Tile)> {
        let mut adjacents = Vec::with_capacity(4);

        let top = row
            .checked_sub(1)
            .and_then(|row| self.tiles.get(&(row, col)))
            .unwrap_or(&Tile::Ground);
        if curr.connected_to_north(top) {
            let key = (row.checked_sub(1), Some(col));
            adjacents.push((key, *top))
        }

        let bottom = self.tiles.get(&(row + 1, col)).unwrap_or(&Tile::Ground);
        if curr.connected_to_south(bottom) {
            let key = (Some(row + 1), Some(col));
            adjacents.push((key, *bottom))
        }

        let left = col
            .checked_sub(1)
            .and_then(|col| self.tiles.get(&(row, col)))
            .unwrap_or(&Tile::Ground);
        if curr.connected_to_west(left) {
            let key = (Some(row), col.checked_sub(1));
            adjacents.push((key, *left))
        }

        let right = self.tiles.get(&(row, col + 1)).unwrap_or(&Tile::Ground);
        if curr.connected_to_east(right) {
            let key = (Some(row), Some(col + 1));
            adjacents.push((key, *right))
        }

        adjacents
    }

    fn process(&self) {
        type Node = ((usize, usize), Tile);
        let mut graph = Graph::<Node, usize>::new();
        let mut node_indexes = HashMap::new();

        for curr in self.tiles.clone().into_iter() {
            let curr_node = node_indexes
                .entry(curr)
                .or_insert_with(|| graph.add_node(curr))
                .clone();

            for ((row, col), tile) in self.adjacent_tiles(curr) {
                if let (Some(row), Some(col)) = (row, col) {
                    let adj = ((row, col), tile);

                    let adj_node = node_indexes
                        .entry(adj)
                        .or_insert_with(|| graph.add_node(adj));

                    graph.add_edge(curr_node, *adj_node, 1);
                }
            }
        }

        let graph_dot = dot::Dot::with_config(&graph, &[dot::Config::EdgeNoLabel]);
        println!("{:?}", graph_dot);

        let path = maximum_matching(&graph);
        dbg!(path.len());
        dbg!(path.nodes().collect::<Vec<_>>());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn from_char(c: char) -> Self {
        use Tile::*;
        match c {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => panic!("unknown char: `{c}`"),
        }
    }

    fn connected_to_north(&self, tile: &Tile) -> bool {
        use Tile::*;
        match (self, tile) {
            (Ground, _) => false,
            (_, Ground) => false,

            (Start, _) => true,
            (_, Start) => true,

            (EastWest, _) => false,
            (_, EastWest) => false,

            (SouthEast, NorthWest) => true,
            (SouthEast, _) => false,

            (SouthWest, NorthEast) => true,
            (SouthWest, _) => false,

            (NorthSouth, NorthSouth) => true,
            (NorthSouth, SouthEast) => true,
            (NorthSouth, SouthWest) => true,
            (NorthSouth, _) => false,

            (NorthEast, NorthSouth) => true,
            (NorthEast, SouthEast) => true,
            (NorthEast, SouthWest) => true,
            (NorthEast, _) => false,

            (NorthWest, NorthSouth) => true,
            (NorthWest, SouthEast) => true,
            (NorthWest, SouthWest) => true,
            (NorthWest, _) => false,
        }
    }

    fn connected_to_south(&self, tile: &Tile) -> bool {
        use Tile::*;
        match (self, tile) {
            (Ground, _) => false,
            (_, Ground) => false,

            (Start, _) => true,
            (_, Start) => true,

            (EastWest, _) => false,
            (_, EastWest) => false,

            (NorthEast, SouthWest) => true,
            (NorthEast, _) => false,

            (NorthWest, SouthEast) => true,
            (NorthWest, _) => false,

            (NorthSouth, NorthSouth) => true,
            (NorthSouth, NorthEast) => true,
            (NorthSouth, NorthWest) => true,
            (NorthSouth, _) => false,

            (SouthEast, NorthSouth) => true,
            (SouthEast, NorthEast) => true,
            (SouthEast, NorthWest) => true,
            (SouthEast, _) => false,

            (SouthWest, NorthSouth) => true,
            (SouthWest, SouthEast) => true,
            (SouthWest, SouthWest) => true,
            (SouthWest, _) => false,
        }
    }

    fn connected_to_west(&self, tile: &Tile) -> bool {
        use Tile::*;
        match (self, tile) {
            (Ground, _) => false,
            (_, Ground) => false,

            (Start, _) => true,
            (_, Start) => true,

            (NorthSouth, _) => false,

            (EastWest, EastWest) => true,
            (EastWest, SouthEast) => true,
            (EastWest, NorthEast) => true,
            (EastWest, _) => false,

            (NorthEast, _) => false,

            (NorthWest, EastWest) => true,
            (NorthWest, NorthEast) => true,
            (NorthWest, SouthEast) => true,
            (NorthWest, _) => false,

            (SouthEast, _) => false,

            (SouthWest, EastWest) => true,
            (SouthWest, NorthEast) => true,
            (SouthWest, SouthEast) => true,
            (SouthWest, _) => false,
        }
    }

    fn connected_to_east(&self, tile: &Tile) -> bool {
        use Tile::*;
        match (self, tile) {
            (Ground, _) => false,
            (_, Ground) => false,

            (Start, _) => true,
            (_, Start) => true,

            (NorthSouth, _) => false,

            (EastWest, EastWest) => true,
            (EastWest, NorthWest) => true,
            (EastWest, SouthWest) => true,
            (EastWest, _) => false,

            (NorthEast, EastWest) => true,
            (NorthEast, NorthWest) => true,
            (NorthEast, SouthWest) => true,
            (NorthEast, _) => false,

            (NorthWest, _) => false,
            (SouthWest, _) => false,

            (SouthEast, EastWest) => true,
            (SouthEast, NorthWest) => true,
            (SouthEast, SouthWest) => true,
            (SouthEast, _) => false,
        }
    }
}

fn main() {
    let grid = Grid::from_str(include_str!("test2.txt"));
    grid.process();
}
