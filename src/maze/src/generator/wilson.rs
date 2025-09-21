use std::collections::HashMap;

use rand::prelude::*;

use crate::{Board, CURSOR_COLOR, Direction, Generator, State};

use raylib_egui_rs::color::Color;
use raylib_egui_rs::math::*;
use raylib_egui_rs::raylib;

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

    fn draw_arrow(&self, board: &Board, cell: &usize, direction: &Direction) {
        let x = board.cells[*cell].x;
        let y = board.cells[*cell].y;
        let (start_pos, end_pos) = match direction {
            Direction::North => (
                Vector2 {
                    x: (x * board.cell_size + board.cell_size / 2) as f32,
                    y: (y * board.cell_size + board.cell_size - board.cell_size / 3) as f32,
                },
                Vector2 {
                    x: (x * board.cell_size + board.cell_size / 2) as f32,
                    y: (y * board.cell_size + board.cell_size / 3) as f32,
                },
            ),
            Direction::South => (
                Vector2 {
                    x: (x * board.cell_size + board.cell_size / 2) as f32,
                    y: (y * board.cell_size + board.cell_size / 3) as f32,
                },
                Vector2 {
                    x: (x * board.cell_size + board.cell_size / 2) as f32,
                    y: (y * board.cell_size + board.cell_size - board.cell_size / 3) as f32,
                },
            ),
            Direction::East => (
                Vector2 {
                    x: (x * board.cell_size + board.cell_size - board.cell_size / 3) as f32,
                    y: (y * board.cell_size + board.cell_size / 2) as f32,
                },
                Vector2 {
                    x: (x * board.cell_size + board.cell_size / 3) as f32,
                    y: (y * board.cell_size + board.cell_size / 2) as f32,
                },
            ),
            Direction::West => (
                Vector2 {
                    x: (x * board.cell_size + board.cell_size / 3) as f32,
                    y: (y * board.cell_size + board.cell_size / 2) as f32,
                },
                Vector2 {
                    x: (x * board.cell_size + board.cell_size - board.cell_size / 3) as f32,
                    y: (y * board.cell_size + board.cell_size / 2) as f32,
                },
            ),
        };
        // Draw the shaft of the arrow
        raylib::DrawLineEx(start_pos, end_pos, 2.0, Color::RED);

        // Calculate the direction vector of the arrow
        let direction: Vector2 = raylib::Vector2Subtract(end_pos, start_pos);
        let length = raylib::Vector2Length(direction);

        // If the arrow has negligible length, skip drawing the head to avoid issues
        if length < 0.001 {
            return;
        };

        // Normalize the direction vector
        let norm_dir = raylib::Vector2Normalize(direction);

        // Calculate the center point of the arrowhead's base
        // This point is 'headSize' units back from 'endPos' along the arrow's direction
        let base_center = raylib::Vector2Add(
            end_pos,
            raylib::Vector2Scale(norm_dir, -(board.cell_size as f32) / 10.0),
        );

        // Calculate a vector perpendicular to the arrow's direction
        // This is used to find the two base vertices of the arrowhead triangle
        let perp_dir = Vector2 {
            x: -norm_dir.y,
            y: norm_dir.x,
        }; // Rotates (dx, dy) to (-dy, dx) for perpendicular 

        // Calculate the two base vertices of the arrowhead triangle
        let v1 = raylib::Vector2Add(
            base_center,
            raylib::Vector2Scale(perp_dir, board.cell_size as f32 / 20.0),
        );
        let v2 = raylib::Vector2Subtract(
            base_center,
            raylib::Vector2Scale(perp_dir, board.cell_size as f32 / 20.0),
        );

        // Draw the arrowhead triangle
        raylib::DrawTriangle(v1, end_pos, v2, Color::RED);
    }
}

impl Generator for Wilson {
    fn step(&mut self, board: &mut Board) -> State {
        match self.state {
            IState::Search => {
                let last = self.current;
                let neighbors = board.neighbors(self.current);
                let neighbors: Vec<(usize, &Option<usize>)> = neighbors
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_some())
                    .collect();
                let index: usize = self.rng.random_range(0..neighbors.len());
                self.current = neighbors[index].1.unwrap();

                self.visited.insert(
                    last,
                    match neighbors[index].0 {
                        0 => Direction::North,
                        1 => Direction::South,
                        2 => Direction::East,
                        3 => Direction::West,
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
                    if self.available.is_empty() {
                        return State::GenerationDone;
                    }
                    self.visited.clear();
                    self.start = self.available[self.rng.random_range(0..self.available.len())];
                    self.current = self.start;
                    self.state = IState::Search;
                }
            }
        }
        State::Generate
    }

    fn draw(&self, board: &Board) {
        raylib::DrawCircle(
            (board.x + board.cells[self.start].x * board.cell_size + board.cell_size / 2) as i32,
            (board.y + board.cells[self.start].y * board.cell_size + board.cell_size / 2) as i32,
            board.cell_size as f32 / 4.0,
            Color::WHITE,
        );
        raylib::DrawCircle(
            (board.x + board.cells[self.current].x * board.cell_size + board.cell_size / 2) as i32,
            (board.y + board.cells[self.current].y * board.cell_size + board.cell_size / 2) as i32,
            board.cell_size as f32 / 4.0,
            CURSOR_COLOR,
        );
        for (c, d) in &self.visited {
            self.draw_arrow(board, c, d);
        }
    }
}
