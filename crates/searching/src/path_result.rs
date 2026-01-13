use crate::grid::Coord;

#[derive(Debug, Clone)]
pub struct PathResult {
    pub(crate) path: Vec<Coord>,
    pub(crate) total_cost: usize,
    pub(crate) expanded: usize,
    pub(crate) max_frontier: usize,
    pub(crate) expanded_order: Vec<Coord>
}

impl PathResult {
    pub fn empty() -> Self {
        PathResult {
            path: vec![],
            total_cost: 0,
            expanded: 0,
            max_frontier: 0,
            expanded_order: vec![]
        }
    }
    pub fn new(start: Coord) -> Self{
        PathResult {
            path: vec![start],
            total_cost: 0,
            expanded: 0,
            max_frontier: 1,
            expanded_order: vec![start]
        }
    }
    pub fn path(&self) -> &[Coord] { &self.path }
    pub fn into_path(self) -> Vec<Coord> { self.path }
    pub fn total_cost(&self) -> usize { self.total_cost }
    pub fn expanded(&self) -> usize { self.expanded }
    pub fn max_frontier(&self) -> usize { self.max_frontier }
    pub fn expanded_order(&self) -> &[Coord] { &self.expanded_order}
}
