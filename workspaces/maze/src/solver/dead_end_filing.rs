use crate::{Board, PATH_COLOR, Solver, State, path};

use raylib_egui_rs::color::Color;
use raylib_egui_rs::raylib;

pub struct DeadEndFilling {
    end: usize,
    dead_ends: Vec<usize>,
    dead_path: Vec<usize>,
    pub path: Vec<usize>,
    current: i32,
}

impl DeadEndFilling {
    pub fn new(board: &Board) -> Self {
        let mut dead_ends = vec![];
        for cell in &board.cells {
            if cell.is_dead_end() {
                dead_ends.push(board.get_index(cell.x, cell.y));
            }
        }
        Self {
            end: board.get_index(board.board_size - 1, board.board_size - 1),
            dead_ends,
            dead_path: vec![],
            path: vec![],
            current: 0,
        }
    }
}

impl Solver for DeadEndFilling {
    fn step(&mut self, board: &Board) -> Result<State, String> {
        if let Some(cell) = self.dead_ends.pop() {
            self.current = cell as i32;
            let current = &board.cells[cell];
            let neighbors: Vec<usize> = board
                .neighbors(board.get_index(current.x, current.y))
                .into_iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if let Some(c) = c {
                        if !self.dead_path.contains(&c) && !self.path.contains(&c) {
                            match i {
                                0 => {
                                    if !current.walls.top {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                1 => {
                                    if !current.walls.bottom {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                2 => {
                                    if !current.walls.left {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                3 => {
                                    if !current.walls.right {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if neighbors.len() == 1 {
                let next = &board.cells[*neighbors.first().unwrap()];
                if !(next.x == board.board_size - 1 && next.y == board.board_size - 1
                    || next.x == 0 && next.y == 0)
                {
                    self.dead_ends.push(*neighbors.first().unwrap());
                }
                self.dead_path.push(cell);
            }
        } else {
            if self.path.is_empty() {
                self.path.push(0);
            }
            let index = self.path.last().unwrap();
            if *index == self.end {
                return Ok(State::Done);
            }
            let current = &board.cells[*self.path.last().unwrap()];
            let neighbors: Vec<usize> = board
                .neighbors(board.get_index(current.x, current.y))
                .into_iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if let Some(c) = c {
                        if !self.dead_path.contains(&c) && !self.path.contains(&c) {
                            match i {
                                0 => {
                                    if !current.walls.top {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                1 => {
                                    if !current.walls.bottom {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                2 => {
                                    if !current.walls.left {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                3 => {
                                    if !current.walls.right {
                                        Some(c)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if neighbors.len() != 1 {
                return Err(format!("neighbors is: {:?}", neighbors));
            }
            self.path.push(*neighbors.first().unwrap());
        }

        Ok(State::Solve)
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path
    }

    fn draw(&self, board: &Board) {
        unsafe {
            for index in &self.dead_path {
                let cell = &board.cells[*index];
                raylib::DrawLine(
                    (board.x + cell.x * board.cell_size + 1) as i32,
                    (board.y + cell.y * board.cell_size + 1) as i32,
                    (board.x + cell.x * board.cell_size + board.cell_size - 1) as i32,
                    (board.y + cell.y * board.cell_size + board.cell_size - 1) as i32,
                    Color::RED,
                );
                raylib::DrawLine(
                    (board.x + cell.x * board.cell_size + board.cell_size - 1) as i32,
                    (board.y + cell.y * board.cell_size + 1) as i32,
                    (board.x + cell.x * board.cell_size + 1) as i32,
                    (board.y + cell.y * board.cell_size + board.cell_size - 1) as i32,
                    Color::RED,
                );
            }
            let current = &board.cells[self.current as usize];
            raylib::DrawCircle(
                (board.x + current.x * board.cell_size + board.cell_size / 2) as i32,
                (board.y + current.y * board.cell_size + board.cell_size / 2) as i32,
                board.cell_size as f32 / 5.0,
                Color::GREEN,
            );
            path::draw_path(board, self.get_path(), PATH_COLOR);
        }
    }
}
