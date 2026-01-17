use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
pub use crate::grid::types::{Cell, CellType, Coord};
use crate::grid::utils::{coord_from_index, in_bounds, index_from_coord};
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub allow_diagonal: bool,
}

impl Grid {
    pub fn from_seed(
        seed: u64,
        width: usize,
        height: usize,
        mud_pct: u8,   // 0..=100
        wall_pct: u8,  // 0..=100
        allow_diagonal: bool
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
        let last_index = cells.len() - 1;
        let starting_index = rng.random_range(0..last_index);
        let mut finishing_index = rng.random_range(0..last_index);

        while starting_index == finishing_index {
            finishing_index = rng.random_range(0..starting_index);
        }
        cells[starting_index].cell_type = CellType::Start;
        cells[finishing_index].cell_type = CellType::Finish;
        Self { width, height, cells, allow_diagonal }
    }

    pub fn get_start(&self) -> Coord {
        let index = self.cells.iter().position(|cell| cell.cell_type == CellType::Start).unwrap();
        coord_from_index(index, self.width)
    }
    pub fn get_finish(&self) -> Coord {
        let index = self.cells.iter().position(|cell| cell.cell_type == CellType::Finish).unwrap();
        coord_from_index(index, self.width)
    }

    pub fn get_cell_value(&self, c: Coord) -> usize {
        assert!(in_bounds(c, self.width as isize, self.height as isize));
        let index = index_from_coord(c, self.width as isize, self.height as isize);
        self.cells.get(index).unwrap().cell_type.value()
    }
    fn passable(&self, c: Coord) -> bool {
        if !in_bounds(c, self.width as isize, self.height as isize) {
            return false;
        }

        let optional_cell = self.cells.get(index_from_coord(c, self.width as isize, self.height as isize));
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

    
    fn neighbors4(&self, c: Coord) -> impl Iterator<Item = Coord> + '_ {
        let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.into_iter().filter_map(move |offset| {
            self.get_neighbor(c, offset)
        })
    }
    fn neighbors8(&self, c: Coord) -> impl Iterator<Item = Coord> + '_ {
        let offsets: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        offsets.into_iter().filter_map(move |offset| {
            self.get_neighbor(c, offset)
        })
    }

    fn get_neighbor(&self, c: Coord, offset: (isize, isize)) -> Option<Coord> {
        let x = offset.0 + c.x;
        let y = offset.1 + c.y;
        let candidate = Coord { x, y };

        if self.passable(candidate) {
            Some(candidate)
        } else {
            None
        }
    }

    pub fn neighbors_for_pathfinding(&self, c: Coord) -> Box<dyn Iterator<Item=Coord> + '_> {
        if self.allow_diagonal {
            Box::new(self.neighbors8(c))
        } else {
            Box::new(self.neighbors4(c))
        }
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
            allow_diagonal: false
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
            let idx = index_from_coord(c, grid.width as isize, grid.height as isize);
            let back = coord_from_index(idx, grid.width);
            assert_eq!(c, back);
        }
    }

    #[test]
    fn neighbors_corner() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);
        let c = Coord { x: 0, y: 0 };

        let neighbors: Vec<_> = grid.neighbors_for_pathfinding(c).collect();

        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Coord { x: 1, y: 0 }));
        assert!(neighbors.contains(&Coord { x: 0, y: 1 }));
    }

    #[test]
    fn neighbors_edge() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);
        let c = Coord { x: 1, y: 0 };

        let neighbors: Vec<_> = grid.neighbors_for_pathfinding(c).collect();

        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&Coord { x: 0, y: 0 }));
        assert!(neighbors.contains(&Coord { x: 2, y: 0 }));
        assert!(neighbors.contains(&Coord { x: 1, y: 1 }));
    }

    #[test]
    fn neighbors_center() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);
        let c = Coord { x: 1, y: 1 };

        let neighbors: Vec<_> = grid.neighbors_for_pathfinding(c).collect();

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

        let neighbors: Vec<_> = grid.neighbors_for_pathfinding(c).collect();

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
