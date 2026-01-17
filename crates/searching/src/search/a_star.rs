use crate::grid::grid::{ Grid};
use crate::grid::types::Coord;
use crate::PathResult;
use crate::search::core_searching::shortest_path_with_heuristic;
use crate::search::heuristics::manhattan_heuristic;

pub fn a_star_manhattan(grid: &Grid, start: Coord, goal: Coord) -> Option<PathResult> {
    shortest_path_with_heuristic(grid, start, goal, &manhattan_heuristic)
}