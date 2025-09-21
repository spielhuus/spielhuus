use rand::prelude::*;

use crate::{Board, Generator, State};

pub const BOOL_TRUE_PROBABILITY: f64 = 0.5;

#[derive(Default)]
pub struct BinaryTree {
    x: usize,
    y: usize,
    rng: ThreadRng,
}

impl BinaryTree {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            rng: rand::rng(),
        }
    }
}

impl Generator for BinaryTree {
    fn step(&mut self, board: &mut Board) -> State {
        if self.x >= board.board_size - 1 && self.y >= board.board_size - 1 {
            return State::GenerationDone;
        }

        let east = if self.x == board.board_size - 1 {
            false
        } else if self.y == board.board_size - 1 {
            true
        } else {
            self.rng.random_bool(BOOL_TRUE_PROBABILITY)
        };

        let cell = board.get_index(self.x, self.y);
        let neighbor = if east {
            board.get_index(self.x + 1, self.y)
        } else {
            board.get_index(self.x, self.y + 1)
        };

        board.remove_wall(cell, neighbor);

        if self.x == board.board_size - 1 {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }
        State::Generate
    }

    fn draw(&self, _board: &Board) {}
}
