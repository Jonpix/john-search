use searching::{bfs, dijkstra, Coord};
use searching::test_support::builders::{grid_with, grid_with_mud, assert_path_is_valid};

#[test]
fn dijkstra_matches_bfs_on_uniform_grid() {
    let grid = grid_with(
        4, 4,
        Coord { x: 0, y: 0 },
        Coord { x: 3, y: 3 },
        &[],
    );

    let start = Coord { x: 0, y: 0 };
    let goal  = Coord { x: 3, y: 3 };

    let bfs_result = bfs(&grid, start, goal).unwrap();
    let result = dijkstra(&grid, start, goal).unwrap();

    assert_eq!(bfs_result.path().len(), result.path().len());
    assert_path_is_valid(&grid, result.path(), start, goal);
    assert_eq!(result.total_cost(), bfs_result.path().len() - 1);
}

#[test]
fn dijkstra_avoids_mud_when_cheaper_path_exists() {
    let grid = grid_with_mud(
        4, 2,
        Coord { x: 0, y: 0 },
        Coord { x: 3, y: 0 },
        &[],
        &[
            Coord { x: 1, y: 0 },
            Coord { x: 2, y: 0 },
        ],
    );

    let start = Coord { x: 0, y: 0 };
    let goal  = Coord { x: 3, y: 0 };

    let bfs_result = bfs(&grid, start, goal).unwrap();
    let dijkstra_result = dijkstra(&grid, start, goal).unwrap();

    assert!(bfs_result.path().len() < dijkstra_result.path().len());
    assert!(dijkstra_result.total_cost() < (bfs_result.path().len() as u32 * 2) as usize);
    assert_path_is_valid(&grid, dijkstra_result.path(), start, goal);
}

#[test]
fn dijkstra_unreachable_returns_none() {
    let grid = grid_with(
        3, 3,
        Coord { x: 0, y: 0 },
        Coord { x: 2, y: 2 },
        &[
            Coord { x: 1, y: 2 },
            Coord { x: 2, y: 1 },
        ],
    );

    let result = dijkstra(&grid, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 });
    assert!(result.is_none());
}

#[test]
fn dijkstra_uses_mud_when_it_is_unavoidable() {
    let grid = grid_with_mud(
        3, 1,
        Coord { x: 0, y: 0 },
        Coord { x: 2, y: 0 },
        &[],
        &[Coord { x: 1, y: 0 }],
    );

    let start = Coord { x: 0, y: 0 };
    let goal  = Coord { x: 2, y: 0 };

    let result = dijkstra(&grid, start, goal).unwrap();

    assert_eq!(result.path().len(), 3);
    assert_eq!(result.total_cost(), 1 + 2); // normal + mud
}
