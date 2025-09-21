use crate::{Board, PATH_COLOR, Solver, State, path};

pub struct AStar {
    end: usize,
    positions: Vec<usize>,
    pub path: Vec<usize>,
    // rng: ThreadRng,
}

impl AStar {
    pub fn new(board: &Board) -> Self {
        Self {
            end: board.get_index(board.board_size - 1, board.board_size - 1),
            positions: vec![0],
            path: vec![0],
            // rng: rand::rng(),
        }
    }
}

impl Solver for AStar {
    fn step(&mut self, board: &Board) -> Result<State, String> {
        let current = &board.cells[*self.path.last().unwrap()];
        let neighbors: Option<(usize, usize)> = board
            .neighbors(board.get_index(current.x, current.y))
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if let Some(c) = c {
                    let neighbor = &board.cells[c];
                    let distance =
                        (board.board_size - neighbor.x) + (board.board_size - neighbor.y);
                    if !self.positions.contains(&c) {
                        match i {
                            0 => {
                                if !current.walls.top {
                                    Some((c, distance))
                                } else {
                                    None
                                }
                            }
                            1 => {
                                if !current.walls.bottom {
                                    Some((c, distance))
                                } else {
                                    None
                                }
                            }
                            2 => {
                                if !current.walls.left {
                                    Some((c, distance))
                                } else {
                                    None
                                }
                            }
                            3 => {
                                if !current.walls.right {
                                    Some((c, distance))
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
            .min_by(|a, b| a.1.cmp(&b.1));

        if let Some((cell, _)) = neighbors {
            self.path.push(cell);
            self.positions.push(cell);
            if cell == self.end {
                return Ok(State::Done);
            }
        } else {
            self.path.pop();
        }

        Ok(State::Solve)
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path
    }

    fn draw(&self, board: &Board) {
        path::draw_path(board, self.get_path(), PATH_COLOR);
    }
}
