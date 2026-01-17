use searching::{a_star_manhattan, Coord};
use searching::test_support::builders::{grid_with, assert_path_is_valid};

#[test]
fn a_star_manhattan_finds_path_on_open_grid() {
    let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

    let start = Coord { x: 0, y: 0 };
    let goal = Coord { x: 2, y: 2 };

    let result = a_star_manhattan(&grid, start, goal).expect("expected a path");
    assert_path_is_valid(&grid, result.path(), start, goal);
}
