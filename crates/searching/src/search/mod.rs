pub mod bfs;
pub mod dijkstra;
pub mod core_searching;
pub mod heuristics;
pub mod a_star;
// keep internal utilities private within the search module
mod utils;

// Re-export primary search APIs for ergonomic access from crate root
pub use bfs::bfs;
pub use dijkstra::dijkstra;
pub use a_star::a_star_manhattan;
pub use core_searching::shortest_path_with_heuristic;