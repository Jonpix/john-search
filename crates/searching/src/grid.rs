#[derive(PartialEq)]
pub enum Algorithms { Bfs , Dijkstra, AStarManhattan  }
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScoredCoord {
    pub coord: Coord,
    pub actual_cost: usize,
    pub estimated_total_cost: usize,
}

impl ScoredCoord {
    pub(crate) fn new(coord: Coord, actual_cost: usize, estimated_total_cost: usize) -> Self {
        Self { coord, actual_cost, estimated_total_cost }
    }
}
impl Ord for ScoredCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .estimated_total_cost
            .cmp(&self.estimated_total_cost)
            .then_with(|| self.coord.y.cmp(&other.coord.y))
            .then_with(|| self.coord.x.cmp(&other.coord.x))
    }
}

impl PartialOrd for ScoredCoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CellType {
    Start,
    Finish,
    Wall,
    Normal,
    Mud,
}

impl CellType {
    fn value(&self) -> usize {
        match self {
            CellType::Mud => 2,
            _ => 1
        }
    }
}
#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
}
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
impl Grid {
    pub fn from_seed(
        seed: u64,
        width: usize,
        height: usize,
        mud_pct: u8,   // 0..=100
        wall_pct: u8,  // 0..=100
    ) -> Self {
        assert!(width > 0 && height > 0);
        assert!(mud_pct + wall_pct <= 100, "mud + wall density must be ≤ 100");

        let mut rng = StdRng::seed_from_u64(seed);
        let mut cells = vec![Cell { cell_type: CellType::Normal }; width * height];

        for cell in &mut cells {
            let roll = rng.random_range(0..100);
            *cell = match roll {
                r if r < mud_pct as i32 => Cell { cell_type: CellType::Mud },
                r if r < (mud_pct + wall_pct) as i32 => Cell { cell_type: CellType::Wall },
                _ => Cell { cell_type: CellType::Normal },
            };
        }

        // Force start and finish
        cells[0].cell_type = CellType::Start;
        let last_index = cells.len() - 1;
        cells[last_index].cell_type = CellType::Finish;

        Self { width, height, cells }
    }

    pub fn get_start(&self) -> Coord {
        let index = self.cells.iter().position(|cell| cell.cell_type == CellType::Start).unwrap();
        self.coord_from_index(index)
    }
    pub fn get_finish(&self) -> Coord {
        let index = self.cells.iter().position(|cell| cell.cell_type == CellType::Finish).unwrap();
        self.coord_from_index(index)
    }

    pub(crate) fn index_from_coord(&self, c: Coord) -> usize {
        assert!(self.in_bounds(c));
        (c.y * (self.width as isize) + c.x) as usize
    }
    fn coord_from_index(&self, idx: usize) -> Coord {
        let x = idx % self.width;
        let y = idx / self.width;
        Coord {
            x: x as isize,
            y: y as isize,
        }
    }
    fn in_bounds(&self, c: Coord) -> bool {
        if c.x >= self.width as isize || c.x < 0 {
            return false;
        }

        if c.y >= self.height as isize || c.y < 0 {
            return false;
        }
        true
    }

    pub fn get_cell_value(&self, c: Coord) -> usize {
        assert!(self.in_bounds(c));
        let index = self.index_from_coord(c);
        self.cells.get(index).unwrap().cell_type.value()
    }
    fn passable(&self, c: Coord) -> bool {
        if !self.in_bounds(c) {
            return false;
        }

        let optional_cell = self.cells.get(self.index_from_coord(c));
        if let Some(cell) = optional_cell {
            return match cell.cell_type {
                CellType::Start => true,
                CellType::Finish => true,
                CellType::Wall => false,
                CellType::Normal => true,
                CellType::Mud => true
            };
        }
        false
    }

    pub fn neighbors4(&self, c: Coord) -> impl Iterator<Item = Coord> + '_ {
        let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.into_iter().filter_map(move |offset| {
            let x = offset.0 + c.x;
            let y = offset.1 + c.y;
            let candidate = Coord { x, y };

            if self.passable(candidate) {
                Some(candidate)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_with(
        width: usize,
        height: usize,
        start: Coord,
        finish: Coord,
        walls: &[Coord],
    ) -> Grid {
        let mut cells = vec![
            Cell {
                cell_type: CellType::Normal
            };
            width * height
        ];

        let idx = |c: Coord| (c.y as usize) * width + (c.x as usize);

        cells[idx(start)].cell_type = CellType::Start;
        cells[idx(finish)].cell_type = CellType::Finish;

        for &c in walls {
            cells[idx(c)].cell_type = CellType::Wall;
        }

        Grid {
            width,
            height,
            cells,
        }
    }

    #[test]
    fn index_round_trip() {
        let grid = grid_with(4, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

        let coords = [
            Coord { x: 0, y: 0 },
            Coord { x: 3, y: 0 },
            Coord { x: 0, y: 2 },
            Coord { x: 3, y: 2 },
            Coord { x: 1, y: 1 },
        ];

        for &c in &coords {
            let idx = grid.index_from_coord(c);
            let back = grid.coord_from_index(idx);
            assert_eq!(c, back);
        }
    }

    #[test]
    fn neighbors_corner() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);
        let c = Coord { x: 0, y: 0 };

        let neighbors: Vec<_> = grid.neighbors4(c).collect();

        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Coord { x: 1, y: 0 }));
        assert!(neighbors.contains(&Coord { x: 0, y: 1 }));
    }

    #[test]
    fn neighbors_edge() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);
        let c = Coord { x: 1, y: 0 };

        let neighbors: Vec<_> = grid.neighbors4(c).collect();

        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&Coord { x: 0, y: 0 }));
        assert!(neighbors.contains(&Coord { x: 2, y: 0 }));
        assert!(neighbors.contains(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn neighbors_center() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);
        let c = Coord { x: 1, y: 1 };

        let neighbors: Vec<_> = grid.neighbors4(c).collect();

        assert_eq!(neighbors.len(), 4);
    }

    #[test]
    fn neighbors_respect_walls() {
        let grid = grid_with(
            3,
            3,
            Coord { x: 0, y: 0 },
            Coord { x: 2, y: 2 },
            &[Coord { x: 1, y: 0 }],
        );
        let c = Coord { x: 0, y: 0 };

        let neighbors: Vec<_> = grid.neighbors4(c).collect();

        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0], Coord { x: 0, y: 1 });
    }

    #[test]
    fn start_and_finish_are_passable() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

        assert!(grid.passable(Coord { x: 0, y: 0 }));
        assert!(grid.passable(Coord { x: 2, y: 2 }));
    }

    #[test]
    fn grid_cell_count_matches_dimensions() {
        let grid = grid_with(4, 3, Coord { x: 0, y: 0 }, Coord { x: 3, y: 2 }, &[]);
        assert_eq!(grid.width * grid.height, grid.cells.len());
    }
}
