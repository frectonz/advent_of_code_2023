use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

use petgraph::{algo::all_simple_paths, dot, Graph};

#[derive(Debug)]
struct Grid {
    tiles: BTreeMap<(usize, usize), Tile>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let tiles = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .map(Tile::from_char)
                    .enumerate()
                    .map(move |(col, tile)| ((row, col), tile))
            })
            .collect::<BTreeMap<_, _>>();

        Self { tiles }
    }

    fn adjacent_tiles(
        &self,
        ((row, col), curr): ((usize, usize), Tile),
    ) -> Vec<((usize, usize), Tile)> {
        let mut adjacents = Vec::with_capacity(4);

        let north = row
            .checked_sub(1)
            .and_then(|row| self.tiles.get(&(row, col)))
            .unwrap_or(&Tile::Ground);
        if curr.connected_to_north(north) {
            let key = row.checked_sub(1).map(|row| (row, col));
            adjacents.push((key, *north))
        }

        let south = self.tiles.get(&(row + 1, col)).unwrap_or(&Tile::Ground);
        if curr.connected_to_south(south) {
            let key = Some((row + 1, col));
            adjacents.push((key, *south))
        }

        let west = col
            .checked_sub(1)
            .and_then(|col| self.tiles.get(&(row, col)))
            .unwrap_or(&Tile::Ground);
        if curr.connected_to_west(west) {
            let key = col.checked_sub(1).map(|col| (row, col));
            adjacents.push((key, *west))
        }

        let east = self.tiles.get(&(row, col + 1)).unwrap_or(&Tile::Ground);
        if curr.connected_to_east(east) {
            let key = Some((row, col + 1));
            adjacents.push((key, *east))
        }

        adjacents
            .into_iter()
            .filter(|tile| tile.0.is_some())
            .map(|tile| (tile.0.unwrap(), tile.1))
            .collect()
    }

    fn process(self) {
        type Node = ((usize, usize), Tile);
        let mut graph = Graph::<Node, usize>::new();
        let mut node_indexes = HashMap::new();

        for curr in self.tiles.clone().into_iter() {
            let curr_node = *node_indexes
                .entry(curr)
                .or_insert_with(|| graph.add_node(curr));

            for adj in self.adjacent_tiles(curr) {
                let adj_node = node_indexes
                    .entry(adj)
                    .or_insert_with(|| graph.add_node(adj));

                graph.add_edge(curr_node, *adj_node, 1);
            }
        }

        let graph_dot = dot::Dot::with_config(&graph, &[dot::Config::EdgeNoLabel]);
        fs::write("graph.dot", format!("{:?}", graph_dot)).unwrap();

        let start = self
            .tiles
            .into_iter()
            .find(|(_, t)| *t == Tile::Start)
            .unwrap();
        let start = node_indexes.get(&start).unwrap();

        let path = all_simple_paths::<Vec<_>, _>(&graph, *start, *start, 3, None)
            .map(|p| p.len())
            .max()
            .unwrap();

        println!("Answer: {}", (path - 1) / 2)
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

            (SouthEast, _) => false,
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

            (NorthEast, _) => false,
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
            (SouthWest, NorthEast) => true,
            (SouthWest, NorthWest) => true,
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
            (NorthEast, _) => false,
            (SouthEast, _) => false,

            (EastWest, EastWest) => true,
            (EastWest, SouthEast) => true,
            (EastWest, NorthEast) => true,
            (EastWest, _) => false,

            (NorthWest, EastWest) => true,
            (NorthWest, SouthEast) => true,
            (NorthWest, NorthEast) => true,
            (NorthWest, _) => false,

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
            (NorthWest, _) => false,
            (SouthWest, _) => false,

            (EastWest, EastWest) => true,
            (EastWest, NorthWest) => true,
            (EastWest, SouthWest) => true,
            (EastWest, _) => false,

            (NorthEast, EastWest) => true,
            (NorthEast, NorthWest) => true,
            (NorthEast, SouthWest) => true,
            (NorthEast, _) => false,

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
