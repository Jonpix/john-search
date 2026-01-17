use searching::{bfs, Coord};
use searching::test_support::builders::{grid_with, assert_path_is_valid};

#[test]
fn bfs_start_equals_goal() {
    let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

    let start = Coord { x: 1, y: 1 };
    let result = bfs(&grid, start, start).expect("expected a path");
    assert_eq!(result.path(), &[start]);
}

#[test]
fn bfs_open_grid_shortest_path_len_and_validity() {
    let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

    let start = Coord { x: 0, y: 0 };
    let goal = Coord { x: 2, y: 2 };

    let result = bfs(&grid, start, goal).expect("expected a path");
    assert_path_is_valid(&grid, result.path(), start, goal);

    // Manhattan distance is 4 moves => 5 coords
    assert_eq!(result.path().len(), 5);
}

#[test]
fn bfs_detours_around_wall_shortest_path_len_and_validity() {
    let grid = grid_with(
        3,
        3,
        Coord { x: 0, y: 0 },
        Coord { x: 2, y: 0 },
        &[Coord { x: 1, y: 0 }],
    );

    let start = Coord { x: 0, y: 0 };
    let goal = Coord { x: 2, y: 0 };

    let result = bfs(&grid, start, goal).expect("expected a path");
    assert_path_is_valid(&grid, result.path(), start, goal);

    // Must detour: 4 moves => 5 coords
    assert_eq!(result.path().len(), 5);
}

#[test]
fn bfs_unreachable_returns_none() {
    let grid = grid_with(
        3,
        3,
        Coord { x: 0, y: 0 },
        Coord { x: 2, y: 2 },
        &[Coord { x: 1, y: 2 }, Coord { x: 2, y: 1 }],
    );

    let start = Coord { x: 0, y: 0 };
    let goal = Coord { x: 2, y: 2 };

    let path = bfs(&grid, start, goal);
    assert!(path.is_none());
}

#[test]
fn bfs_exact_path_respects_offset_order_left_right_up_down() {
    // This exact-path test assumes Grid neighbor order: Left, Right, Up, Down
    let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

    let start = Coord { x: 0, y: 0 };
    let goal = Coord { x: 2, y: 2 };

    let result = bfs(&grid, start, goal).expect("expected a path");

    let expected = vec![
        Coord { x: 0, y: 0 },
        Coord { x: 1, y: 0 },
        Coord { x: 2, y: 0 },
        Coord { x: 2, y: 1 },
        Coord { x: 2, y: 2 },
    ];

    assert_eq!(result.into_path(), expected);
}
