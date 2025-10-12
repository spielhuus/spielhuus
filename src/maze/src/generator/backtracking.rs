use rand::prelude::*;

use crate::{Board, CELL_BACKTRACK, CELL_CURSOR, CELL_VISITED, Generator, MazeState};

#[derive(Default)]
pub struct Backtracking {
    current: usize,
    rng: ThreadRng,
}

impl Backtracking {
    pub fn new() -> Self {
        Self {
            current: 0,
            rng: rand::rng(),
        }
    }
}

impl Generator for Backtracking {
    fn step(&mut self, board: &mut Board) -> MazeState {
        let n = board.neighbors(self.current);
        board.gpu_data[self.current][0] &= !CELL_CURSOR;
        let free: Option<&Option<usize>> = n
            .iter()
            .filter(|i| i.is_some() && !board.cells[i.unwrap()].visited)
            .choose(&mut self.rng);

        if let Some(&Some(free)) = free {
            // remove the walls
            board.remove_wall(self.current, free);
            // set next cell as current
            board.gpu_data[free][0] |= CELL_BACKTRACK;
            board.gpu_data[free][0] |= CELL_CURSOR;
            self.current = free;
            board.path.push(free)
        } else if let Some(last) = board.path.pop() {
            board.gpu_data[self.current][0] &= !CELL_BACKTRACK;
            board.gpu_data[self.current][0] |= CELL_VISITED;
            board.gpu_data[last][0] |= CELL_CURSOR;
            board.gpu_data[last][0] &= !CELL_BACKTRACK;
            board.gpu_data[last][0] |= CELL_VISITED;
            self.current = last;
        } else {
            board.gpu_data[self.current][0] &= !CELL_CURSOR;
            board.gpu_data[self.current][0] |= CELL_VISITED;
            return MazeState::GenerationDone;
        }

        MazeState::Generate
    }
}
