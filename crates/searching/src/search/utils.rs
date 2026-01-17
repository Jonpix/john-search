use crate::grid::grid::Coord;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScoredCoord {
    pub coord: Coord,
    pub actual_cost: usize,
    pub estimated_total_cost: usize,
}

impl ScoredCoord {
    pub(crate) fn new(coord: Coord, actual_cost: usize, estimated_total_cost: usize) -> Self {
        Self { coord, actual_cost, estimated_total_cost }
    }
}
impl Ord for ScoredCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .estimated_total_cost
            .cmp(&self.estimated_total_cost)
            .then_with(|| self.coord.y.cmp(&other.coord.y))
            .then_with(|| self.coord.x.cmp(&other.coord.x))
    }
}

impl PartialOrd for ScoredCoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
