use rand::prelude::*;

use crate::{
    Board, CELL_CURSOR, CELL_VISITED, Generator, MazeState, WALL_BOTTOM, WALL_LEFT, WALL_RIGHT,
    WALL_TOP,
};

enum IState {
    Hunt,
    Kill,
}

pub struct HuntAndKill {
    visited: Vec<usize>,
    current_cell: usize,
    state: IState,
    rng: ThreadRng,
}

impl HuntAndKill {
    pub fn new(board: &mut Board) -> Self {
        let mut rng = rand::rng();
        let current_cell = rng.random_range(0..board.board_size.pow(2)) as usize;
        board.cells[current_cell].visited = true;
        Self {
            visited: vec![current_cell],
            current_cell,
            state: IState::Kill,
            rng,
        }
    }

    fn contains(&self, index: &usize) -> bool {
        self.visited.contains(index)
    }
}

impl Generator for HuntAndKill {
    fn step(&mut self, board: &mut Board) -> MazeState {
        match self.state {
            IState::Hunt => {
                for y in 0..board.board_size {
                    for x in 0..board.board_size {
                        let current = board.get_index(x, y);
                        // skip if visited
                        if self.contains(&(current)) {
                            continue;
                        }
                        // get visited
                        let visited_neighbors: Vec<usize> = board
                            .neighbors(current)
                            .into_iter()
                            .flatten()
                            .filter(|item| self.contains(item))
                            .collect();

                        if !visited_neighbors.is_empty() {
                            self.current_cell = current;
                            board.cells[current].visited = true;
                            self.visited.push(current);
                            let index = self.rng.random_range(0..visited_neighbors.len());
                            let next = visited_neighbors[index];
                            match board.cells[self.current_cell].direction(&board.cells[next]) {
                                crate::Direction::North => {
                                    board.cells[self.current_cell].walls.top = false;
                                    board.cells[next].walls.bottom = false;
                                    board.gpu_data[self.current_cell][0] &= !WALL_TOP;
                                    board.gpu_data[next][0] &= !WALL_BOTTOM;
                                }
                                crate::Direction::South => {
                                    board.cells[self.current_cell].walls.bottom = false;
                                    board.cells[next].walls.top = false;
                                    board.gpu_data[self.current_cell][0] &= !WALL_BOTTOM;
                                    board.gpu_data[next][0] &= !WALL_TOP;
                                }
                                crate::Direction::East => {
                                    board.cells[self.current_cell].walls.right = false;
                                    board.cells[next].walls.left = false;
                                    board.gpu_data[self.current_cell][0] &= !WALL_RIGHT;
                                    board.gpu_data[next][0] &= !WALL_LEFT;
                                }
                                crate::Direction::West => {
                                    board.cells[self.current_cell].walls.left = false;
                                    board.cells[next].walls.right = false;
                                    board.gpu_data[self.current_cell][0] &= !WALL_LEFT;
                                    board.gpu_data[next][0] &= !WALL_RIGHT;
                                }
                            }
                            self.state = IState::Kill;
                            return MazeState::Generate;
                        }
                    }
                }
                return MazeState::GenerationDone;
            }
            IState::Kill => {
                // get the neighbors of the current cell and pick a random neighbor
                board.gpu_data[self.current_cell][0] &= !CELL_CURSOR;
                let neighbors: Vec<usize> = board
                    .neighbors(self.current_cell)
                    .into_iter()
                    .flatten()
                    .filter(|item| !self.contains(item))
                    .collect();

                // start hunt when no neighbors where found
                if neighbors.is_empty() {
                    self.state = IState::Hunt;
                    return MazeState::Generate;
                }

                let index = self.rng.random_range(0..neighbors.len());
                board.gpu_data[neighbors[index]][0] |= CELL_CURSOR;
                let next = neighbors[index];
                // remove wall
                if !self.contains(&next) {
                    match board.cells[self.current_cell].direction(&board.cells[next]) {
                        crate::Direction::North => {
                            board.cells[self.current_cell].walls.top = false;
                            board.cells[next].walls.bottom = false;
                            board.gpu_data[self.current_cell][0] &= !WALL_TOP;
                            board.gpu_data[next][0] &= !WALL_BOTTOM;
                        }
                        crate::Direction::South => {
                            board.cells[self.current_cell].walls.bottom = false;
                            board.cells[next].walls.top = false;
                            board.gpu_data[self.current_cell][0] &= !WALL_BOTTOM;
                            board.gpu_data[next][0] &= !WALL_TOP;
                        }
                        crate::Direction::East => {
                            board.cells[self.current_cell].walls.right = false;
                            board.cells[next].walls.left = false;
                            board.gpu_data[self.current_cell][0] &= !WALL_RIGHT;
                            board.gpu_data[next][0] &= !WALL_LEFT;
                        }
                        crate::Direction::West => {
                            board.cells[self.current_cell].walls.left = false;
                            board.cells[next].walls.right = false;
                            board.gpu_data[self.current_cell][0] &= !WALL_LEFT;
                            board.gpu_data[next][0] &= !WALL_RIGHT;
                        }
                    }
                    board.cells[next].visited = true;
                    board.gpu_data[next][0] |= CELL_VISITED;
                    self.visited.push(next);
                }
                self.current_cell = next;
            }
        }

        if self.visited.len() >= board.cells.len() {
            board.gpu_data[self.current_cell][0] &= !CELL_CURSOR;
            MazeState::GenerationDone
        } else {
            MazeState::Generate
        }
    }
}
