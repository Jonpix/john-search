use std::collections::HashSet;
use crate::{CellType, Coord, Grid};
const RESET: &str = "\x1b[0m";

const GREEN: &str = "\x1b[92m";
const RED: &str = "\x1b[91m";
const WHITE: &str = "\x1b[97m";
const YELLOW: &str = "\x1b[93m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[94m";


impl Grid {

    /// Render the grid as ASCII, optionally overlaying:
    /// - `path`: shown as `*` (does not overwrite S/F/#)
    /// - `expanded`: shown as `+` (does not overwrite S/F/# or path)
    ///
    /// Legend:
    /// S = start, F = finish, # = wall, ~ = mud, . = normal, * = path, + = expanded
    pub fn render(&self, path: &[Coord], expanded: &[Coord]) -> String {
        let path_set: HashSet<Coord> = path.iter().copied().collect();
        let expanded_set: HashSet<Coord> = expanded.iter().copied().collect();

        let mut out = String::with_capacity((self.width + 1) * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let c = Coord { x: x as isize, y: y as isize };
                let idx = self.index_from_coord(c);
                let cell = &self.cells[idx];
                let cell_str = match cell.cell_type {
                    CellType::Start  => format!("{GREEN}S{RESET}"),
                    CellType::Finish => format!("{RED}F{RESET}"),
                    CellType::Wall   => format!("{WHITE}█{RESET}"),

                    CellType::Mud => {
                        if path_set.contains(&c) {
                            format!("{BLUE}●{RESET}")
                        } else if expanded_set.contains(&c) {
                            format!("{CYAN}○{RESET}")
                        } else {
                            format!("{YELLOW}~{RESET}")
                        }
                    }

                    CellType::Normal => {
                        if path_set.contains(&c) {
                            format!("{BLUE}●{RESET}")
                        } else if expanded_set.contains(&c) {
                            format!("{CYAN}○{RESET}")
                        } else {
                            "·".to_string()
                        }
                    }
                };


                out.push_str(&cell_str);
                out.push(' ');
            }
            out.push('\n');
        }
        let legend = format!(
            "{GREEN}S{RESET}=Start  \
     {RED}F{RESET}=Finish  \
     {WHITE}█{RESET}=Wall  \
     {YELLOW}~{RESET}=Mud  \
     {CYAN}○{RESET}=Expanded  \
     {BLUE}●{RESET}=Path  \
     ·=Normal"
        );
        out.push_str(&legend);
        out.push('\n');
        out

    }
}
