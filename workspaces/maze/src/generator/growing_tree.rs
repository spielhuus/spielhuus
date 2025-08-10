use rand::prelude::*;

use crate::{Board, Generator, State};

pub struct GrowingTree {
    visited: Vec<usize>,
    cells: Vec<usize>,
    rng: ThreadRng,
}

impl GrowingTree {
    pub fn new(board: &Board) -> Self {
        let cell = rand::rng().random_range(0..board.board_size ^ 2) as usize;
        Self {
            visited: vec![],
            cells: vec![cell],
            rng: rand::rng(),
        }
    }

    fn contains(&self, index: &usize) -> bool {
        self.visited.contains(index) || self.cells.contains(index)
    }
}

impl Generator for GrowingTree {
    fn step(&mut self, board: &mut Board) -> State {
        let index = self.rng.random_range(0..self.cells.len());
        let cell = self.cells[index];
        let neighbors: Vec<usize> = board
            .neighbors(cell)
            .into_iter()
            .flatten()
            .filter(|item| !self.contains(item))
            .collect();

        if neighbors.is_empty() {
            self.cells.retain(|&x| x != cell);
            self.visited.push(cell);
        } else {
            let index = self.rng.random_range(0..neighbors.len());
            let neighbor = neighbors[index];
            board.remove_wall(cell, neighbor);
            self.cells.push(neighbor);
        }

        if self.visited.len() >= board.cells.len() {
            State::GenerationDone
        } else {
            State::Generate
        }
    }

    fn draw(&self, _board: &Board) {}
}
