use std::fmt::format;
use eframe::egui;
use eframe::epaint::StrokeKind;
use searching::{bfs, CellType, Coord, Grid, PathResult};
use std::time::{SystemTime, UNIX_EPOCH};
use egui::Rect;
use searching::search_algs::{a_star_manhattan, dijkstra};

const COLS: usize = 20;
const ROWS: usize = 20;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Simple Grid"),
        ..Default::default()
    };

    eframe::run_native(
        "Searching",
        options,
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}

struct App {
    grid: Grid,
    selected_cell_type: CellType,
    group_one_value: Algorithms,
    path_result: PathResult,
    current_step_number: usize,

}

impl App {
    fn new() -> Self {
        Self {
            current_step_number: 0,
            selected_cell_type: CellType::Normal,
            group_one_value: Algorithms::Bfs,
            grid: Grid::from_seed(178712312973918, COLS, ROWS, 30, 20),
            path_result: PathResult::empty(),
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * COLS + x
    }

    fn cell_color(&self, ct: CellType) -> egui::Color32 {
        match ct {
            CellType::Start => egui::Color32::GREEN,
            CellType::Finish => egui::Color32::RED,
            CellType::Wall => egui::Color32::BLACK,
            CellType::Normal => egui::Color32::DARK_GRAY,
            CellType::Mud => egui::Color32::from_hex("#633727").unwrap(),
        }
    }
    fn get_rect(rect: Rect, col: usize, row: usize, cell: f32) -> Rect{
        Rect::from_min_size(
            egui::pos2(
                rect.left() + col as f32 * cell,
                rect.top() + row as f32 * cell,
            ),
            egui::vec2(cell, cell),
        )
    }

    fn get_bread_crumb_rect(rect: Rect, col: usize, row: usize, cell: f32) -> Rect{
        Rect::from_min_size(
            egui::pos2(
                rect.left() + cell / 4.0,
                rect.top() + cell / 4.0,
            ),
            egui::vec2(cell / 2.0, cell / 2.0),
        )
    }
    
    fn draw_grid(&mut self, ui: &mut egui::Ui) {
        egui::Frame::new()
            .inner_margin(egui::Margin::same(2))
            .show(ui, |ui| {
                // Reserve a rect sized to an integer number of cells
                let available = ui.available_size();
                let cell = (available.x / COLS as f32)
                    .min(available.y / ROWS as f32)
                    .floor()
                    .max(10.0);

                let grid_size = egui::vec2(cell * COLS as f32, cell * ROWS as f32);
                let (rect, _) = ui.allocate_exact_size(grid_size, egui::Sense::hover());

                // Reuse painter and stroke
                let painter = ui.painter().clone();
                let stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
                let path: &[Coord] = self.path_result.path();
                let expanded: &[Coord] = self.path_result.expanded_order();


                for row in 0..ROWS {
                    for col in 0..COLS {
                        let i = self.idx(col, row);
                        let cell_rect = Self::get_rect(rect, col, row, cell);
                        let response = ui.allocate_rect(cell_rect, egui::Sense::click());
                        let pointer_down = ui.input(|i| i.pointer.primary_down());
                        let mut color = self.cell_color(self.grid.cells[i].cell_type);

                        painter.rect_filled(cell_rect, 0.0, color);
                        painter.rect_stroke(cell_rect, 0.0, stroke, StrokeKind::Inside);
                        let coord = Coord { x: col as isize, y: row as isize };
                        let coord_index = expanded.iter().position(|c| c == &coord).unwrap_or(expanded.len());
                        if self.current_step_number >= coord_index && path.contains(&coord) {
                            let bread_crumb_rect = Self::get_bread_crumb_rect(cell_rect, col, row, cell);
                            color = egui::Color32::CYAN;
                            painter.circle_filled(bread_crumb_rect.center(), bread_crumb_rect.width() / 2.0, color);
                        } else if self.current_step_number >= coord_index && expanded.contains(&Coord { x: col as isize, y: row as isize }) {
                            let bread_crumb_rect = Self::get_bread_crumb_rect(cell_rect, col, row, cell);
                            color = egui::Color32::from_rgba_unmultiplied(206, 227, 20, 128);
                            painter.circle_filled(bread_crumb_rect.center(), bread_crumb_rect.width() / 2.0, color);
                        }

                        if (ui.rect_contains_pointer(cell_rect) && pointer_down) || response.clicked() {
                            self.grid.cells[i].cell_type = self.selected_cell_type;
                        }
                    }
                }
            });
    }
}
#[derive(PartialEq)]
enum Algorithms { Bfs , Dijkstra, AStarManhattan  }

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Algorithm");
                ui.radio_value(&mut self.group_one_value, Algorithms::Bfs, "BFS");
                ui.radio_value(&mut self.group_one_value, Algorithms::AStarManhattan, "A* (Manhattan)");
                ui.radio_value(&mut self.group_one_value, Algorithms::Dijkstra, "Dijkstra");
                ui.separator();
                ui.label("Cell Tools");
                ui.radio_value(&mut self.selected_cell_type, CellType::Start, "Start");
                ui.radio_value(&mut self.selected_cell_type, CellType::Finish, "Finish");
                ui.radio_value(&mut self.selected_cell_type, CellType::Normal, "Normal");
                ui.radio_value(&mut self.selected_cell_type, CellType::Wall, "Wall");
                ui.radio_value(&mut self.selected_cell_type, CellType::Mud, "Mud");
                ui.separator();
                ui.label("Actions");

                ui.horizontal(|ui| {
                    if ui.button("Search").clicked(){
                        self.current_step_number = 0;
                        self.path_result = match self.group_one_value {
                            Algorithms::Dijkstra => { dijkstra(&self.grid, self.grid.get_start(), self.grid.get_finish()).unwrap_or( PathResult::empty()) },
                            Algorithms::Bfs => { bfs(&self.grid, self.grid.get_start(), self.grid.get_finish()).unwrap_or( PathResult::empty())}
                            Algorithms::AStarManhattan => {a_star_manhattan(&self.grid, self.grid.get_start(), self.grid.get_finish()).unwrap_or( PathResult::empty())}
                        };
                    }
                    if ui.button("Reset Grid").clicked(){
                        self.path_result = PathResult::empty();
                        self.current_step_number = 0;
                        let now = SystemTime::now();
                        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
                        let seed: u64 = since_epoch.as_nanos() as u64; // Or use as_secs() for seconds
                        self.grid = Grid::from_seed(seed, COLS, ROWS, 10, 20);
                    }
                    if ui.button("Clear Grid").clicked(){
                        self.path_result = PathResult::empty();
                        self.current_step_number = 0;
                        for cell in &mut self.grid.cells {
                            cell.cell_type = CellType::Normal;
                        }
                    };

                });
                if !self.path_result.path().is_empty() {
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Prev").clicked(){
                            if self.current_step_number > 0 {
                                self.current_step_number -= 1;
                            }
                        }
                        ui.label(format!("Step {}", self.current_step_number + 1));
                        if ui.button("Next").clicked(){
                            if self.current_step_number < self.path_result.expanded_order().len() {
                                self.current_step_number += 1;
                            }
                        }
                    });
                }
            });
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_grid(ui);
        });
    }
}
