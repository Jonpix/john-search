use std::collections::{BinaryHeap, HashMap};
use crate::grid::grid::{Grid};
use crate::grid::types::Coord;
use crate::PathResult;
use crate::search::utils::ScoredCoord;

pub fn shortest_path_with_heuristic(grid: &Grid, start: Coord, goal: Coord, h: &dyn Fn(Coord, Coord) -> usize) -> Option<PathResult> {
    if start == goal {
        return Some(PathResult::new(start));
    }

    let mut heap: BinaryHeap<ScoredCoord> = BinaryHeap::new();
    let mut dist: HashMap<Coord, usize> = HashMap::new();
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();
    let mut expanded_order: Vec<Coord> = Vec::new();

    let mut expanded: usize = 0;
    let mut max_frontier: usize = 0;

    dist.insert(start, 0);
    came_from.insert(start, start);
    heap.push(ScoredCoord::new(start, 0, h(start, goal)));
    max_frontier = max_frontier.max(heap.len());

    while let Some(state) = heap.pop() {
        let current = state.coord;
        let popped_cost = state.actual_cost;

        let best_known = *dist.get(&current).unwrap();
        if popped_cost != best_known {
            continue;
        }

        expanded += 1;
        expanded_order.push(current);
        if current == goal {
            break;
        }

        for neighbor in grid.neighbors_for_pathfinding(current) {
            let new_cost = best_known + grid.get_cell_value(neighbor);
            let is_better = match dist.get(&neighbor) {
                None => true,
                Some(&old) => new_cost < old,
            };

            if is_better {
                dist.insert(neighbor, new_cost);
                came_from.insert(neighbor, current);
                heap.push(ScoredCoord::new(neighbor, new_cost,new_cost + h(neighbor, goal)));
                max_frontier = max_frontier.max(heap.len());
            }
        }
    }

    let total_cost  = match dist.get(&goal) {
        None => return None,
        Some(&c) => c,
    };

    let mut path: Vec<Coord> = Vec::new();
    let mut current = goal;

    while current != start {
        path.push(current);
        current = *came_from.get(&current).unwrap();
    }
    path.push(start);
    path.reverse();

    Some(PathResult {
        path,
        total_cost,
        expanded,
        max_frontier,
        expanded_order,
    })
}
