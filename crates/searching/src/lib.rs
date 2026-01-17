pub mod grid;
pub mod search;
pub mod path;
pub mod test_support;

// Re-exports for a clean public API
pub use grid::{CellType, Coord, Cell, Grid};
pub use path::PathResult;
pub use search::{bfs, dijkstra, a_star_manhattan, shortest_path_with_heuristic};
