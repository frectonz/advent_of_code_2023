use std::collections::{BTreeMap, HashMap};

use petgraph::{algo::all_simple_paths, Graph};

type Node = ((usize, usize), Tile);

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

    fn process(self) -> isize {
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

        let start = self
            .tiles
            .clone()
            .into_iter()
            .find(|(_, t)| *t == Tile::Start)
            .unwrap();
        let start = node_indexes.get(&start).unwrap();

        let path = all_simple_paths::<Vec<_>, _>(&graph, *start, *start, 3, None)
            .max_by_key(|p| p.len())
            .unwrap();

        let path = path
            .into_iter()
            .flat_map(|node_id| {
                node_indexes
                    .iter()
                    .find(|(_, value)| **value == node_id)
                    .map(|(key, _)| key)
                    .cloned()
            })
            .collect::<Vec<Node>>();

        let area = shoelace_algorithm(&path);
        picks_theorem(area, path.len() as isize)
    }
}

fn picks_theorem(area: isize, boundary: isize) -> isize {
    area - (boundary / 2) + 1
}

fn shoelace_algorithm(vertices: &[Node]) -> isize {
    let mut area: isize = 0;

    for w in vertices.windows(2) {
        let ((row_1, col_1), _) = w[0];
        let ((row_2, col_2), _) = w[1];

        area += (row_1 * col_2) as isize;
        area -= (col_1 * row_2) as isize;
    }

    isize::abs(area) / 2
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
    let grid = Grid::from_str(include_str!("input.txt"));
    println!("Answer {}", grid.process());
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn examples() {
        let grid = Grid::from_str(include_str!("test1.txt"));
        assert_eq!(grid.process(), 1);

        let grid = Grid::from_str(include_str!("test2.txt"));
        assert_eq!(grid.process(), 1);

        let grid = Grid::from_str(include_str!("test3.txt"));
        assert_eq!(grid.process(), 4);

        let grid = Grid::from_str(include_str!("test4.txt"));
        assert_eq!(grid.process(), 8);

        let grid = Grid::from_str(include_str!("test5.txt"));
        assert_eq!(grid.process(), 10);
    }
}
