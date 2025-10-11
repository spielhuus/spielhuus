use rand::prelude::*;

use crate::{Board, Generator, MazeState, WALL_BOTTOM, WALL_LEFT, WALL_RIGHT, WALL_TOP};

// use raylib_egui_rs::color::Color;
// use raylib_egui_rs::raylib;

#[derive(Debug)]
struct Area {
    start: (usize, usize),
    end: (usize, usize),
}

pub struct RecursiveDivision {
    areas: Vec<Area>,
    rng: ThreadRng,
    probability: f64,
    area: Area,
}

impl RecursiveDivision {
    pub fn new(board: &mut Board) -> Self {
        //remove all walls
        for (i, cell) in &mut board.cells.iter_mut().enumerate() {
            if cell.x > 0 {
                cell.walls.left = false;
                board.gpu_data[i] &= !WALL_LEFT;
            }
            if cell.y > 0 {
                cell.walls.top = false;
                board.gpu_data[i] &= !WALL_TOP;
            }
            if cell.x < board.board_size - 1 {
                cell.walls.right = false;
                board.gpu_data[i] &= !WALL_RIGHT;
            }
            if cell.y < board.board_size - 1 {
                cell.walls.bottom = false;
                board.gpu_data[i] &= !WALL_BOTTOM;
            }
            cell.visited = true;
        }
        Self {
            areas: vec![Area {
                start: (0, 0),
                end: (board.board_size, board.board_size),
            }],
            rng: rand::rng(),
            probability: 0.5,
            area: Area {
                start: (0, 0),
                end: (board.board_size - 1, board.board_size - 1),
            },
        }
    }

    fn split_horizontal(
        &self,
        x: usize,
        y: usize,
        board: &mut Board,
        area: &Area,
        new_areas: &mut Vec<Area>,
    ) {
        for index in area.start.0..area.end.0 {
            if x != index {
                let c0 = board.get_index(index, y);
                board.gpu_data[c0] |= WALL_BOTTOM;
                board.cells[c0].walls.bottom = true;
                if y < board.board_size - 1 {
                    let c1 = board.get_index(index, y + 1);
                    board.gpu_data[c1] |= WALL_TOP;
                    board.cells[c1].walls.top = true;
                }
            }
        }

        //size is bigger then 1 cell
        if y - area.start.1 > 0 {
            new_areas.push(Area {
                start: (area.start.0, area.start.1),
                end: (area.end.0, y + 1),
            });
        }
        if area.end.1 - (y + 2) > 0 {
            new_areas.push(Area {
                start: (area.start.0, y + 1),
                end: (area.end.0, area.end.1),
            });
        }
    }

    fn split_vertical(
        &self,
        x: usize,
        y: usize,
        board: &mut Board,
        area: &Area,
        new_areas: &mut Vec<Area>,
    ) {
        // vertical
        for index in area.start.1..area.end.1 {
            if y != index {
                let c0 = board.get_index(x, index);
                board.cells[c0].walls.right = true;
                board.gpu_data[c0] |= WALL_RIGHT;
                if x < board.board_size - 1 {
                    let c1 = board.get_index(x + 1, index);
                    board.cells[c1].walls.left = true;
                    board.gpu_data[c1] |= WALL_LEFT;
                }
            }
        }
        //size is bigger then 1 cell
        if x - area.start.0 > 0 {
            new_areas.push(Area {
                start: (area.start.0, area.start.1),
                end: (x + 1, area.end.1),
            });
        }
        if area.end.0 - (x + 2) > 0 {
            new_areas.push(Area {
                start: (x + 1, area.start.1),
                end: (area.end.0, area.end.1),
            });
        }
    }
}

impl Generator for RecursiveDivision {
    fn step(&mut self, board: &mut Board) -> MazeState {
        let mut new_areas: Vec<Area> = Vec::new();
        if let Some(area) = self.areas.pop() {
            let y = self.rng.random_range(area.start.1..area.end.1 - 1);
            let x = self.rng.random_range(area.start.0..area.end.0 - 1);
            if (area.end.0 - area.start.0) < (area.end.1 - area.start.1) {
                self.split_horizontal(x, y, board, &area, &mut new_areas);
            } else if (area.end.0 - area.start.0) > (area.end.1 - area.start.1) {
                self.split_vertical(x, y, board, &area, &mut new_areas);
            } else if self.rng.random_bool(self.probability) {
                self.split_horizontal(x, y, board, &area, &mut new_areas);
            } else {
                self.split_vertical(x, y, board, &area, &mut new_areas);
            }
            self.areas.append(&mut new_areas);
            self.area = area;
            MazeState::Generate
        } else {
            MazeState::GenerationDone
        }
    }

    fn draw(&self, board: &Board) {
        // raylib::DrawRectangle(
        //     (board.x + self.area.start.0 * board.cell_size) as i32,
        //     (board.y + self.area.start.1 * board.cell_size) as i32,
        //     ((self.area.end.0 - self.area.start.0) * board.cell_size) as i32,
        //     ((self.area.end.1 - self.area.start.1) * board.cell_size) as i32,
        //     Color {
        //         r: 150,
        //         g: 0,
        //         b: 0,
        //         a: 50,
        //     },
        // );
    }
}
