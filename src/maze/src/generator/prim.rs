use rand::prelude::*;

use crate::{Board, CELL_CURSOR, Generator, MazeState};

#[derive(Debug)]
struct FreeCell {
    index: usize,
    neighbor: usize,
}

pub struct Prim {
    visited: Vec<usize>,
    cells: Vec<FreeCell>,
    rng: ThreadRng,
}

impl Prim {
    pub fn new(board: &Board) -> Self {
        let mut rng = rand::rng();
        let current = rng.random_range(0..board.board_size ^ 2) as usize;
        let cells = board
            .neighbors(current)
            .into_iter()
            .flatten()
            .map(|index| FreeCell {
                index,
                neighbor: current,
            })
            .collect();

        Self {
            visited: vec![current],
            cells,
            rng,
        }
    }

    fn contains(&self, index: &usize) -> bool {
        self.cells
            .iter()
            .filter(|item| item.index == *index)
            .count()
            > 0
            || self.visited.contains(index)
    }
}

impl Generator for Prim {
    fn step(&mut self, board: &mut Board) -> MazeState {
        let index = self.rng.random_range(0..self.cells.len());
        let item = self.cells.remove(index);

        // remove wall
        board.remove_wall(item.index, item.neighbor);
        board.gpu_data[item.index][0] &= !CELL_CURSOR;

        // calc next cells
        let neighbors: Vec<usize> = board.neighbors(item.index).into_iter().flatten().collect();
        for n in &neighbors {
            if !self.contains(n) {
                board.gpu_data[*n][0] |= CELL_CURSOR;
                self.cells.push(FreeCell {
                    index: *n,
                    neighbor: item.index,
                });
            }
        }

        self.visited.push(item.index);

        if self.cells.is_empty() {
            MazeState::GenerationDone
        } else {
            MazeState::Generate
        }
    }
}
