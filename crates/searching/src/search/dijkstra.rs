use crate::grid::grid::{ Grid};
use crate::grid::types::Coord;
use crate::PathResult;
use crate::search::core_searching::shortest_path_with_heuristic;
use crate::search::heuristics::zero_heuristic;

pub fn dijkstra(grid: &Grid, start: Coord, goal: Coord) -> Option<PathResult> {
    shortest_path_with_heuristic(grid, start, goal, &zero_heuristic)
}