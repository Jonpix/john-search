use crate::grid::{Algorithms, Coord, Grid, ScoredCoord};
use crate::path_result::PathResult;
use std::collections::{BinaryHeap, HashMap, VecDeque};

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

pub fn manhattan_heuristic(start: Coord, finish: Coord) -> usize {
    ((start.x - finish.x).abs() + (start.y - finish.y).abs()) as usize
}
pub fn zero_heuristic(_: Coord, _: Coord) -> usize {
    0
}
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
        for node in grid.neighbors(next_node) {
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
    let max_frontier = 0; //TODO: this is wrong, but it's not used anywhere else
    let expanded = came_from.len();

    Some(PathResult {
        path,
        total_cost,
        expanded,
        max_frontier,
        expanded_order,
    })
}
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

        for neighbor in grid.neighbors(current) {
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
pub fn a_star_manhattan(grid: &Grid, start: Coord, goal: Coord) -> Option<PathResult> {
    shortest_path_with_heuristic(grid, start, goal, &manhattan_heuristic)
}
pub fn dijkstra(grid: &Grid, start: Coord, goal: Coord) -> Option<PathResult> {
    shortest_path_with_heuristic(grid, start, goal, &zero_heuristic)
}

pub fn search(grid: &Grid, start: Coord, goal: Coord, alg: &Algorithms) -> Option<PathResult> {
    match alg {
        Algorithms::Bfs => bfs(grid, start, goal),
        Algorithms::Dijkstra => dijkstra(grid, start, goal),
        Algorithms::AStarManhattan => a_star_manhattan(grid, start, goal),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // pulls in bfs from this file
        use crate::grid::{Cell, CellType, Coord, Grid};

    fn grid_with_mud(
        width: usize,
        height: usize,
        start: Coord,
        finish: Coord,
        walls: &[Coord],
        mud: &[Coord],
    ) -> Grid {
        let mut cells = vec![
            Cell { cell_type: CellType::Normal };
            width * height
        ];

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

        let idx = |c: Coord| -> usize {
            assert!(
                c.x >= 0 && c.y >= 0,
                "negative coord in test helper: {:?}",
                c
            );
            let (x, y) = (c.x as usize, c.y as usize);
            assert!(
                x < width && y < height,
                "out of bounds coord in test helper: {:?}",
                c
            );
            y * width + x
        };

        cells[idx(start)].cell_type = CellType::Start;
        cells[idx(finish)].cell_type = CellType::Finish;

        for &w in walls {
            cells[idx(w)].cell_type = CellType::Wall;
        }

        Grid {
            width,
            height,
            cells,
            allow_diagonal: false,
        }
    }

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

        assert_eq!(bfs_result.path.len(), result.path.len());
        assert_path_is_valid(&grid, &result.path, start, goal);
        assert_eq!(result.total_cost, bfs_result.path.len() - 1);
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


        assert!(bfs_result.path.len() < dijkstra_result.path.len());
        assert!(dijkstra_result.total_cost < (bfs_result.path.len() as u32 * 2) as usize);

        assert_path_is_valid(&grid, &dijkstra_result.path, start, goal);
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

        assert_eq!(result.path.len(), 3);
        assert_eq!(result.total_cost, 1 + 2); // normal + mud
    }


    fn assert_path_is_valid(grid: &Grid, path: &[Coord], start: Coord, goal: Coord) {
        assert!(!path.is_empty(), "path should not be empty");
        assert_eq!(
            path.first().copied(),
            Some(start),
            "path must start at start"
        );
        assert_eq!(path.last().copied(), Some(goal), "path must end at goal");

        for pair in path.windows(2) {
            let a = pair[0];
            let b = pair[1];
            assert!(
                grid.neighbors(a).any(|n| n == b),
                "invalid step {:?} -> {:?}",
                a,
                b
            );
        }
    }

    #[test]
    fn bfs_start_equals_goal() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

        let start = Coord { x: 1, y: 1 };
        let result = bfs(&grid, start, start).expect("expected a path");
        assert_eq!(result.path, vec![start]);
    }

    #[test]
    fn bfs_open_grid_shortest_path_len_and_validity() {
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

        let start = Coord { x: 0, y: 0 };
        let goal = Coord { x: 2, y: 2 };

        let result = bfs(&grid, start, goal).expect("expected a path");
        assert_path_is_valid(&grid, &result.path, start, goal);

        // Manhattan distance is 4 moves => 5 coords
        assert_eq!(result.path.len(), 5);
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
        assert_path_is_valid(&grid, &result.path, start, goal);

        // Must detour: 4 moves => 5 coords
        assert_eq!(result.path.len(), 5);
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
        // This exact-path test assumes your Grid neighbor order is:
        // [(-1,0), (1,0), (0,-1), (0,1)]  => Left, Right, Up, Down
        //
        // From (0,0), Left and Up are invalid, so it will try Right then Down.
        // From (1,0), it will consider Left (already seen), then Right, then Up, then Down, etc.
        let grid = grid_with(3, 3, Coord { x: 0, y: 0 }, Coord { x: 2, y: 2 }, &[]);

        let start = Coord { x: 0, y: 0 };
        let goal = Coord { x: 2, y: 2 };

        let result = bfs(&grid, start, goal).expect("expected a path");

        // With this ordering, the typical BFS shortest path found is:
        // (0,0) -> (1,0) -> (2,0) -> (2,1) -> (2,2)
        // If you change offsets ordering, this test SHOULD fail.
        let expected = vec![
            Coord { x: 0, y: 0 },
            Coord { x: 1, y: 0 },
            Coord { x: 2, y: 0 },
            Coord { x: 2, y: 1 },
            Coord { x: 2, y: 2 },
        ];

        assert_eq!(result.path, expected);
    }
}
