use std::collections::{HashMap, VecDeque};
use crate::{ PathResult};
use crate::grid::grid::{ Grid};
use crate::grid::types::Coord;

pub fn bfs(grid: &Grid, start: Coord, goal: Coord) -> Option<PathResult> {
    if start == goal {
        return Some(PathResult::new(start));
    }

    let mut to_visit: VecDeque<Coord> = VecDeque::new();
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();
    let mut expanded_order: Vec<Coord> = Vec::new();
    to_visit.push_back(start);
    came_from.insert(start, start);

    while let Some(next_node) = to_visit.pop_front() {
        expanded_order.push(next_node);
        if next_node == goal {
            break;
        }
        for node in grid.neighbors_for_pathfinding(next_node) {
            if came_from.contains_key(&node) {
                continue;
            }
            came_from.insert(node, next_node);
            to_visit.push_back(node);
        }
    }

    if !came_from.contains_key(&goal) {
        return None;
    }

    let mut current = goal;
    let mut path: Vec<Coord> = Vec::new();
    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap();
    }

    path.push(start);
    path.reverse();
    let total_cost = path.len() - 1;
    let max_frontier = expanded_order.len();
    let expanded = came_from.len();

    Some(PathResult {
        path,
        total_cost,
        expanded,
        max_frontier,
        expanded_order,
    })
}
