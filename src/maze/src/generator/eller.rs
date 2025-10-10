use std::collections::HashMap;

use disjoint::DisjointSet;
use rand::prelude::*;

use crate::{Board, Generator, MazeState};

// use raylib_egui_rs::raylib;

pub const BOOL_TRUE_PROBABILITY: f64 = 0.5;

enum IState {
    Merge,
    Bottom,
    LastMerge,
    Last,
}

pub struct Eller {
    x: usize,
    y: usize,
    merged: DisjointSet,
    state: IState,
    row: HashMap<usize, Vec<usize>>,
    rng: ThreadRng,
}

impl Eller {
    pub fn new(board: &Board) -> Self {
        Self {
            x: 0,
            y: 0,
            merged: DisjointSet::with_len(board.cells.len()),
            state: IState::Merge,
            row: HashMap::new(),
            rng: rand::rng(),
        }
    }
}

impl Generator for Eller {
    fn step(&mut self, board: &mut Board) -> MazeState {
        match self.state {
            IState::Merge => {
                let cell = board.get_index(self.x, self.y);
                let neighbor = board.get_index(self.x + 1, self.y);

                if !self.merged.is_joined(cell, neighbor)
                    && (self.rng.random_bool(BOOL_TRUE_PROBABILITY))
                {
                    self.merged.join(cell, neighbor);
                    board.remove_wall(cell, neighbor);
                }

                self.row
                    .entry(self.merged.root_of(cell))
                    .or_default()
                    .push(cell);

                self.x += 1;

                // end of the row
                if self.x >= board.board_size - 1 {
                    self.row
                        .entry(self.merged.root_of(board.get_index(self.x, self.y)))
                        .or_default()
                        .push(board.get_index(self.x, self.y));
                    self.x = 0;
                    if self.y == board.board_size - 1 {
                        self.state = IState::Last;
                    } else {
                        self.state = IState::Bottom;
                    }
                }
                MazeState::Generate
            }
            IState::Bottom => {
                let cell = board.get_index(self.x, self.y);
                let neighbor = board.get_index(self.x, self.y + 1);
                if !self.merged.is_joined(cell, neighbor)
                    && self.rng.random_bool(BOOL_TRUE_PROBABILITY)
                {
                    self.merged.join(cell, neighbor);
                    board.remove_wall(cell, neighbor);
                    self.row.remove(&self.merged.root_of(cell));
                }

                self.x += 1;
                if self.x >= board.board_size {
                    for cells in self.row.values() {
                        if let Some(&index) = cells.choose(&mut self.rng) {
                            let neighbor =
                                board.get_index(board.cells[index].x, board.cells[index].y + 1);
                            board.remove_wall(index, neighbor);
                            self.merged.join(index, neighbor);
                        } else {
                            panic!("no top neighbor");
                        }
                    }
                    self.row.clear();
                    self.x = 0;
                    self.y += 1;
                    if self.y == board.board_size - 1 {
                        self.state = IState::LastMerge;
                    } else {
                        self.state = IState::Merge;
                    }
                }
                MazeState::Generate
            }
            IState::LastMerge => {
                let cell = board.get_index(self.x, self.y);
                let neighbor = board.get_index(self.x + 1, self.y);

                if !self.merged.is_joined(cell, neighbor) {
                    self.merged.join(cell, neighbor);
                    board.remove_wall(cell, neighbor);
                }

                self.row
                    .entry(self.merged.root_of(cell))
                    .or_default()
                    .push(cell);

                self.x += 1;

                // end of the row
                if self.x >= board.board_size - 1 {
                    self.row
                        .entry(self.merged.root_of(board.get_index(self.x, self.y)))
                        .or_default()
                        .push(board.get_index(self.x, self.y));
                    self.x = 0;
                    if self.y == board.board_size - 1 {
                        self.state = IState::Last;
                    } else {
                        self.state = IState::Bottom;
                    }
                }
                MazeState::Generate
            }
            IState::Last => {
                let cell = board.get_index(self.x, self.y);
                let neighbor = board.get_index(self.x, self.y - 1);
                if !self.merged.is_joined(cell, neighbor) {
                    self.merged.join(cell, neighbor);
                    board.remove_wall(cell, neighbor);
                    self.row.remove(&self.merged.root_of(cell));
                }

                self.x += 1;
                if self.x >= board.board_size - 1 {
                    for cells in self.row.values() {
                        if let Some(&index) = cells.choose(&mut self.rng) {
                            let neighbor =
                                board.get_index(board.cells[index].x, board.cells[index].y - 1);
                            board.remove_wall(index, neighbor);
                        } else {
                            panic!("no top neigbhor");
                        }
                    }
                    self.row.clear();
                }
                MazeState::GenerationDone
            }
        }
    }

    fn draw(&self, board: &Board) {
        // raylib::DrawCircle(
        //     (board.x + self.x * board.cell_size + board.cell_size / 2) as i32,
        //     (board.y + self.y * board.cell_size + board.cell_size / 2) as i32,
        //     board.cell_size as f32 / 5.0,
        //     CURSOR_COLOR,
        // );
    }
}
