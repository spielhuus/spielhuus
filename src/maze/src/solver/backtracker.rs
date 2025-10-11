use rand::{rngs::ThreadRng, seq::IndexedRandom};

use crate::{Board, MazeState, PathDirection, Solver};

pub struct Backtracker {
    end: usize,
    positions: Vec<usize>,
    pub path: Vec<usize>,
    rng: ThreadRng,
}

impl Backtracker {
    pub fn new(board: &Board) -> Self {
        Self {
            end: board.get_index(board.board_size - 1, board.board_size - 1),
            positions: vec![0],
            path: vec![0],
            rng: rand::rng(),
        }
    }
}

impl Solver for Backtracker {
    fn step(&mut self, board: &mut Board) -> Result<MazeState, String> {
        let current = &board.cells[*self.path.last().unwrap()];
        let neighbors: Vec<usize> = board
            .neighbors(board.get_index(current.x, current.y))
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if let Some(c) = c {
                    if !self.positions.contains(&c) {
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

        let cell = neighbors.choose(&mut self.rng);
        if let Some(&cell) = cell {
            self.path.push(cell);
            crate::update_path(board, &self.path);
            self.positions.push(cell);
            if cell == self.end {
                return Ok(MazeState::Done);
            }
        } else {
            let cell = self.path.pop();
            crate::clear_direction(board, cell.unwrap());
            crate::update_path(board, &self.path);
        }

        Ok(MazeState::Solve)
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path
    }

    fn draw(&self, board: &Board) {
        // path::draw_path(board, self.get_path(), PATH_COLOR);
    }
}
