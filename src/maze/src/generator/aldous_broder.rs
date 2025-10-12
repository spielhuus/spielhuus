use rand::prelude::*;

use crate::{Board, CELL_CURSOR, Generator, MazeState};

pub struct AldousBroder {
    visited: Vec<usize>,
    current_cell: usize,
    rng: ThreadRng,
}

impl AldousBroder {
    pub fn new(board: &Board) -> Self {
        let mut rng = rand::rng();
        let current_cell = rng.random_range(0..board.board_size ^ 2) as usize;
        Self {
            visited: vec![current_cell],
            current_cell,
            rng,
        }
    }

    fn contains(&self, index: &usize) -> bool {
        self.visited.contains(index)
    }
}

impl Generator for AldousBroder {
    fn step(&mut self, board: &mut Board) -> MazeState {
        // get the neighbors of the current cell and pick a random neighbor
        let neighbors: Vec<usize> = board
            .neighbors(self.current_cell)
            .into_iter()
            .flatten()
            .collect();
        let index = self.rng.random_range(0..neighbors.len());
        let next = neighbors[index];
        // remove wall
        if !self.contains(&next) {
            board.remove_wall(self.current_cell, next);
            self.visited.push(next);
        }
        board.gpu_data[self.current_cell][0] &= !CELL_CURSOR;
        self.current_cell = next;

        if self.visited.len() >= board.cells.len() {
            MazeState::GenerationDone
        } else {
            board.gpu_data[next][0] |= CELL_CURSOR;
            MazeState::Generate
        }
    }
}
