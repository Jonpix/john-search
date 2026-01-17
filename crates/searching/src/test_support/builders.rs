use crate::grid::grid::Grid;
use crate::grid::types::{Cell, CellType, Coord};

pub fn grid_with(
    width: usize,
    height: usize,
    start: Coord,
    finish: Coord,
    walls: &[Coord],
) -> Grid {
    let mut cells = vec![
        Cell {
            cell_type: CellType::Normal,
        };
        width * height
    ];

    let idx = |c: Coord| -> usize {
        assert!(c.x >= 0 && c.y >= 0, "negative coord in test helper: {:?}", c);
        let (x, y) = (c.x as usize, c.y as usize);
        assert!(x < width && y < height, "out of bounds coord in test helper: {:?}", c);
        y * width + x
    };

    cells[idx(start)].cell_type = CellType::Start;
    cells[idx(finish)].cell_type = CellType::Finish;

    for &w in walls {
        cells[idx(w)].cell_type = CellType::Wall;
    }

    Grid { width, height, cells, allow_diagonal: false }
}

pub fn grid_with_mud(
    width: usize,
    height: usize,
    start: Coord,
    finish: Coord,
    walls: &[Coord],
    mud: &[Coord],
) -> Grid {
    let mut cells = vec![Cell { cell_type: CellType::Normal }; width * height];

    let idx = |c: Coord| (c.y as usize) * width + (c.x as usize);

    cells[idx(start)].cell_type = CellType::Start;
    cells[idx(finish)].cell_type = CellType::Finish;

    for &c in walls {
        cells[idx(c)].cell_type = CellType::Wall;
    }

    for &c in mud {
        cells[idx(c)].cell_type = CellType::Mud;
    }

    Grid { width, height, cells, allow_diagonal: false }
}

pub fn assert_path_is_valid(grid: &Grid, path: &[Coord], start: Coord, goal: Coord) {
    assert!(!path.is_empty(), "path should not be empty");
    assert_eq!(path.first().copied(), Some(start), "path must start at start");
    assert_eq!(path.last().copied(), Some(goal), "path must end at goal");

    for pair in path.windows(2) {
        let a = pair[0];
        let b = pair[1];
        assert!(
            grid.neighbors_for_pathfinding(a).any(|n| n == b),
            "invalid step {:?} -> {:?}",
            a,
            b
        );
    }
}
