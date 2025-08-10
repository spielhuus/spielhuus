use rand::prelude::*;

use crate::{Board, CURSOR_COLOR, Generator, State};

use raylib_egui_rs::raylib;

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
        let current_cell = rng.random_range(0..board.board_size ^ 2) as usize;
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
    fn step(&mut self, board: &mut Board) -> State {
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
                                }
                                crate::Direction::South => {
                                    board.cells[self.current_cell].walls.bottom = false;
                                    board.cells[next].walls.top = false;
                                }
                                crate::Direction::East => {
                                    board.cells[self.current_cell].walls.right = false;
                                    board.cells[next].walls.left = false;
                                }
                                crate::Direction::West => {
                                    board.cells[self.current_cell].walls.left = false;
                                    board.cells[next].walls.right = false;
                                }
                            }
                            self.state = IState::Kill;
                            return State::Generate;
                        }
                    }
                }
                return State::GenerationDone;
            }
            IState::Kill => {
                // get the neighbors of the current cell and pick a random neighbor
                let neighbors: Vec<usize> = board
                    .neighbors(self.current_cell)
                    .into_iter()
                    .flatten()
                    .filter(|item| !self.contains(item))
                    .collect();

                // start hunt when no neighbors where found
                if neighbors.is_empty() {
                    self.state = IState::Hunt;
                    return State::Generate;
                }

                let index = self.rng.random_range(0..neighbors.len());
                let next = neighbors[index];
                // remove wall
                if !self.contains(&next) {
                    match board.cells[self.current_cell].direction(&board.cells[next]) {
                        crate::Direction::North => {
                            board.cells[self.current_cell].walls.top = false;
                            board.cells[next].walls.bottom = false;
                        }
                        crate::Direction::South => {
                            board.cells[self.current_cell].walls.bottom = false;
                            board.cells[next].walls.top = false;
                        }
                        crate::Direction::East => {
                            board.cells[self.current_cell].walls.right = false;
                            board.cells[next].walls.left = false;
                        }
                        crate::Direction::West => {
                            board.cells[self.current_cell].walls.left = false;
                            board.cells[next].walls.right = false;
                        }
                    }
                    board.cells[next].visited = true;
                    self.visited.push(next);
                }
                self.current_cell = next;
            }
        }

        if self.visited.len() >= board.cells.len() {
            State::GenerationDone
        } else {
            State::Generate
        }
    }

    fn draw(&self, board: &Board) {
        raylib::DrawCircle(
            (board.x + board.cells[self.current_cell].x * board.cell_size + board.cell_size / 2)
                as i32,
            (board.y + board.cells[self.current_cell].y * board.cell_size + board.cell_size / 2)
                as i32,
            board.cell_size as f32 / 4.0,
            CURSOR_COLOR,
        );
    }
}
