#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CellType {
    Start,
    Finish,
    Wall,
    Normal,
    Mud,
}

impl CellType {
    pub(crate) fn value(&self) -> usize {
        match self {
            CellType::Mud => 2,
            _ => 1
        }
    }
}
#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
}
