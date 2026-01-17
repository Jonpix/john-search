use crate::grid::types::Coord;

pub fn manhattan_heuristic(start: Coord, finish: Coord) -> usize {
    ((start.x - finish.x).abs() + (start.y - finish.y).abs()) as usize
}
pub fn zero_heuristic(_: Coord, _: Coord) -> usize {
    0
}

pub fn chebyshev_heuristic(start: Coord, goal: Coord) -> usize {
    let dx = start.x.abs_diff(goal.x);
    let dy = start.y.abs_diff(goal.y);

    // Movement model:
    // - Orthogonal move cost = 1
    // - Diagonal move cost   = 1
    //
    // With equal costs, the shortest path is:
    // - move diagonally as much as possible
    // - then finish with straight moves
    //
    // Total cost = max(dx, dy)

    let straight_move_cost = 1;
    let diagonal_move_cost = 1;

    let diagonal_steps = dx.min(dy);
    let straight_steps = dx.max(dy) - diagonal_steps;

    diagonal_steps * diagonal_move_cost
        + straight_steps * straight_move_cost
}
