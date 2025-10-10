use std::collections::HashMap;

use rand::prelude::*;

use crate::{Board, Direction, Generator, MazeState};

// use raylib_egui_rs::color::Color;
// use raylib_egui_rs::math::*;
// use raylib_egui_rs::raylib;

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
                board.cells[last].cursor = false;
                let neighbors = board.neighbors(self.current);
                let neighbors: Vec<(usize, &Option<usize>)> = neighbors
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_some())
                    .collect();
                let index: usize = self.rng.random_range(0..neighbors.len());
                self.current = neighbors[index].1.unwrap();
                board.cells[self.current].cursor = true;
                board.cells[self.current].backtrack = true;
                self.visited.insert(
                    last,
                    match neighbors[index].0 {
                        0 => { board.cells[last].arrow = Some(Direction::North); Direction::North }
                        1 => { board.cells[last].arrow = Some(Direction::South); Direction::South }
                        2 => { board.cells[last].arrow = Some(Direction::East); Direction::East }
                        3 => { board.cells[last].arrow = Some(Direction::West); Direction::West }
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
                board.cells[last].cursor = false;
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
                    board.cells.iter_mut().for_each(|c| { c.arrow = None; c.backtrack = false; c.cursor = false; } );
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
        println!("----------");
        println!("Wilson: visited: {}, backtrack: {}, direction: {:?}, cursor: {}", 
            board.cells[self.current].visited,
            board.cells[self.current].backtrack,
            board.cells[self.current].arrow,
            board.cells[self.current].cursor,
        );
        MazeState::Generate
    }

    fn draw(&self, board: &Board) {
        // raylib::DrawCircle(
        //     (board.x + board.cells[self.start].x * board.cell_size + board.cell_size / 2) as i32,
        //     (board.y + board.cells[self.start].y * board.cell_size + board.cell_size / 2) as i32,
        //     board.cell_size as f32 / 4.0,
        //     Color::WHITE,
        // );
        // raylib::DrawCircle(
        //     (board.x + board.cells[self.current].x * board.cell_size + board.cell_size / 2) as i32,
        //     (board.y + board.cells[self.current].y * board.cell_size + board.cell_size / 2) as i32,
        //     board.cell_size as f32 / 4.0,
        //     CURSOR_COLOR,
        // );
        // for (c, d) in &self.visited {
        //     self.draw_arrow(board, c, d);
        // }
    }
}
