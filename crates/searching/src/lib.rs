//! Pathfinding on a simple grid: BFS and Dijkstra.
//!
//! Quick start:
//! ```
//! use searching::{Grid, bfs, shortest_path_with_heuristic};
//! let grid = Grid::from_seed(42, 10, 10, 10, 20);
//! let start = grid.get_start();
//! let goal = grid.get_finish();
//! let path = bfs(&grid, start, goal);
//! assert!(path.is_some());
//! let result = shortest_path_with_heuristic(&grid, start, goal);
//! assert!(result.is_some());
//! ```

pub mod grid;
pub mod search_algs;
pub mod path_result;
pub mod rendering;

// Re-export common items at the crate root for ergonomics.
pub use grid::{CellType, Coord, Grid};
pub use search_algs::{bfs, shortest_path_with_heuristic};
pub use path_result::PathResult;
