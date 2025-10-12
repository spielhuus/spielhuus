use std::collections::HashMap;

use rand::prelude::*;

use crate::{
    ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, ARROW_UP, Board, CELL_BACKTRACK, CELL_CURSOR, Direction,
    Generator, MazeState,
};

enum IState {
    Search,
    FollowPath,
}

pub struct Wilson {
    visited: HashMap<usize, Direction>,
    current: usize,
    start: usize,
    ust: Vec<usize>,
    state: IState,
    available: Vec<usize>,
    rng: ThreadRng,
}

impl Wilson {
    pub fn new(board: &mut Board) -> Self {
        let mut available: Vec<usize> = (0..=(board.board_size).pow(2) - 1).collect();
        let mut rng = rand::rng();
        let target = rng.random_range(0..board.board_size ^ 2) as usize;
        available.retain(|&x| x != target);
        let start = rng.random_range(0..available.len()) as usize;
        board.cells[target].visited = true;
        Self {
            visited: HashMap::new(),
            current: start,
            start,
            ust: vec![target],
            state: IState::Search,
            available,
            rng,
        }
    }
}

impl Generator for Wilson {
    fn step(&mut self, board: &mut Board) -> MazeState {
        match self.state {
            IState::Search => {
                let last = self.current;
                board.gpu_data[last][0] &= !CELL_CURSOR;
                let neighbors = board.neighbors(self.current);
                let neighbors: Vec<(usize, &Option<usize>)> = neighbors
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_some())
                    .collect();
                let index: usize = self.rng.random_range(0..neighbors.len());
                self.current = neighbors[index].1.unwrap();
                board.gpu_data[self.current][0] |= CELL_CURSOR;
                board.gpu_data[self.current][0] |= CELL_BACKTRACK;
                self.visited.insert(
                    last,
                    match neighbors[index].0 {
                        0 => {
                            board.gpu_data[last][0] |= ARROW_UP;
                            Direction::North
                        }
                        1 => {
                            board.gpu_data[last][0] |= ARROW_DOWN;
                            Direction::South
                        }
                        2 => {
                            board.gpu_data[last][0] |= ARROW_RIGHT;
                            Direction::East
                        }
                        3 => {
                            board.gpu_data[last][0] |= ARROW_LEFT;
                            Direction::West
                        }
                        _ => panic!("unknwon direction"),
                    },
                );

                if self.ust.contains(&self.current) {
                    self.current = self.start;
                    self.state = IState::FollowPath;
                }
            }
            IState::FollowPath => {
                let last = self.current;
                board.gpu_data[last][0] &= !CELL_BACKTRACK;
                self.ust.push(self.current);
                self.available.retain(|&x| x != self.current);
                let neighbors = board.neighbors(self.current);
                let direction = self.visited.get(&self.current);
                if let Some(direction) = direction {
                    match direction {
                        Direction::North => {
                            board.remove_wall(last, neighbors[0].unwrap());
                            self.current = neighbors[0].unwrap();
                        }
                        Direction::South => {
                            board.remove_wall(last, neighbors[1].unwrap());
                            self.current = neighbors[1].unwrap();
                        }
                        Direction::East => {
                            board.remove_wall(last, neighbors[2].unwrap());
                            self.current = neighbors[2].unwrap();
                        }
                        Direction::West => {
                            board.remove_wall(last, neighbors[3].unwrap());
                            self.current = neighbors[3].unwrap();
                        }
                    }
                }

                if self.ust.contains(&self.current) {
                    board.gpu_data.iter_mut().for_each(|c| {
                        c[0] &= !CELL_CURSOR;
                        c[0] &= !CELL_BACKTRACK;
                        c[0] &= !ARROW_DOWN;
                        c[0] &= !ARROW_UP;
                        c[0] &= !ARROW_LEFT;
                        c[0] &= !ARROW_RIGHT;
                    });
                    if self.available.is_empty() {
                        return MazeState::GenerationDone;
                    }
                    self.visited.clear();
                    self.start = self.available[self.rng.random_range(0..self.available.len())];
                    self.current = self.start;
                    self.state = IState::Search;
                }
            }
        }
        MazeState::Generate
    }
}
