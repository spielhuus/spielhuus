use crate::{solver::path, Board, MazeState, Solver, CROSSED};

pub struct DeadEndFilling {
    end: usize,
    dead_ends: Vec<usize>,
    dead_path: Vec<usize>,
    pub path: Vec<usize>,
    current: i32,
}

impl DeadEndFilling {
    pub fn new(board: &mut Board) -> Self {
        println!("DeadEndFilling::new, size: {}", board.board_size);
        let mut dead_ends = vec![];
        let start_index = 0;
        let end_index = board.get_index(board.board_size - 1, board.board_size - 1);
        for (i, cell) in board.cells.iter().enumerate() {
            if i == start_index || i == end_index {
                continue;
            }
            if cell.is_dead_end() {
                let index = board.get_index(cell.x, cell.y);
                board.gpu_data[index][0] |= CROSSED;
                dead_ends.push(index);
            }
        }
        Self {
            end: board.get_index(board.board_size - 1, board.board_size - 1),
            dead_ends,
            dead_path: vec![],
            path: vec![],
            current: 0,
        }
    }

    fn cross_dead_ends(&self, board: &mut Board) {
        board.cells.iter_mut().for_each(|c| c.crossed = false);
        self.dead_path
            .iter()
            .for_each(|c| board.cells[*c].crossed = true);
    }
}

impl Solver for DeadEndFilling {
    fn step(&mut self, board: &mut Board) -> Result<MazeState, String> {
        if let Some(cell) = self.dead_ends.pop() {
            self.current = cell as i32;
            let current = &board.cells[cell];
            let neighbors: Vec<usize> = board
                .neighbors(board.get_index(current.x, current.y))
                .into_iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if let Some(c) = c {
                        if !self.dead_path.contains(&c) && !self.path.contains(&c) {
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

            if neighbors.len() == 1 {
                let next = &board.cells[*neighbors.first().unwrap()];
                if !(next.x == board.board_size - 1 && next.y == board.board_size - 1
                    || next.x == 0 && next.y == 0)
                {
                    board.gpu_data[*neighbors.first().unwrap()][0] |= CROSSED;
                    self.dead_ends.push(*neighbors.first().unwrap());
                }
                self.dead_path.push(cell);
            }
        } else {
            if self.path.is_empty() {
                self.path.push(0);
            }
            let index = self.path.last().unwrap();
            if *index == self.end {
                board.cells.iter_mut().for_each(|c| c.crossed = false);
                return Ok(MazeState::Done);
            }
            let current = &board.cells[*self.path.last().unwrap()];
            let neighbors: Vec<usize> = board
                .neighbors(board.get_index(current.x, current.y))
                .into_iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    if let Some(c) = c {
                        if !self.dead_path.contains(&c) && !self.path.contains(&c) {
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

            if neighbors.len() != 1 {
                return Err(format!("neighbors is: {:?}", neighbors));
            }
            if neighbors.len() > 0 {
                self.path.push(*neighbors.first().unwrap());
                path::update_path(board, &self.path);
            } else {
                board.gpu_data.iter_mut().for_each(|c| c[0] &= !CROSSED);
                return Ok(MazeState::Done);
            }
        }
        self.cross_dead_ends(board);
        Ok(MazeState::Solve)
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path
    }
}
