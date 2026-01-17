use crate::grid::grid::Coord;

pub fn in_bounds(c: Coord, width: isize, height: isize) -> bool {
    if c.x >= width || c.x < 0 {
        return false;
    }

    if c.y >= height || c.y < 0 {
        return false;
    }
    true
}

pub fn index_from_coord(c: Coord, width: isize, height: isize) -> usize {
    assert!(in_bounds(c, width, height));
    (c.y * width + c.x) as usize
}

pub fn coord_from_index(idx: usize, width: usize) -> Coord {
    let x = idx % width;
    let y = idx / width;
    Coord {
        x: x as isize,
        y: y as isize,
    }
}