pub mod types;
pub mod grid;
// keep utilities private
mod utils;

// Re-exports for a clean public surface
pub use types::{Coord, CellType, Cell};
pub use grid::Grid;