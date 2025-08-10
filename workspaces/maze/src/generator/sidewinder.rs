use rand::prelude::*;

use crate::{Board, Generator, State};

pub const BOOL_TRUE_PROBABILITY: f64 = 0.5;

#[derive(Default)]
pub struct Sidewinder {
    x: usize,
    y: usize,
    set: Vec<usize>,
    rng: ThreadRng,
}

impl Sidewinder {
    pub fn new(board: &mut Board) -> Self {
        for i in 0..board.board_size - 1 {
            let cell = board.get_index(i, 0);
            let neighbor = board.get_index(i + 1, 0);
            board.remove_wall(cell, neighbor);
        }
        Self {
            x: 0,
            y: 1,
            set: vec![],
            rng: rand::rng(),
        }
    }

    fn carve(&mut self, board: &mut Board) {
        let selected = self.rng.random_range(0..self.set.len());
        let index = self.set[selected];
        let neighbor = board.get_index(board.cells[index].x, board.cells[index].y - 1);
        board.remove_wall(index, neighbor);
        self.set.clear();
    }
}

impl Generator for Sidewinder {
    fn step(&mut self, board: &mut Board) -> State {
        let cell = board.get_index(self.x, self.y);
        self.set.push(cell);
        if self.x >= board.board_size - 1 {
            self.carve(board);
        } else if self.rng.random_bool(BOOL_TRUE_PROBABILITY) {
            let neighbor = board.get_index(self.x + 1, self.y);
            board.remove_wall(cell, neighbor);
        } else {
            self.carve(board);
        }

        if self.x >= board.board_size - 1 && self.y >= board.board_size - 1 {
            State::GenerationDone
        } else {
            if self.x == board.board_size - 1 {
                self.x = 0;
                self.y += 1;
            } else {
                self.x += 1;
            }
            State::Generate
        }
    }

    fn draw(&self, _board: &Board) {}
}
