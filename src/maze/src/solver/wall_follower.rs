use crate::{Board, Cell, Direction, Solver, State};

const LINE_WIDTH: f32 = 1.0;

struct Wall {
    direction: Direction,
    cell: usize,
}

impl Wall {
    fn new(direction: Direction, cell: usize) -> Self {
        Self { direction, cell }
    }
}

pub struct WallFollower {
    end: usize,
    pub path: Vec<usize>,
    walls: Vec<Wall>,
    direction: Direction,
    distance: usize,
}

use raylib_egui_rs::color::Color;
use raylib_egui_rs::math::*;
use raylib_egui_rs::raylib;

impl WallFollower {
    pub fn new(board: &Board) -> Self {
        Self {
            end: board.get_index(board.board_size - 1, board.board_size - 1),
            path: vec![0],
            walls: vec![Wall::new(Direction::East, 0)],
            direction: Direction::East,
            distance: 4,
        }
    }
    fn wall_left(&self, cell: &Cell) -> bool {
        match self.direction {
            Direction::North => cell.walls.left,
            Direction::South => cell.walls.right,
            Direction::East => cell.walls.top,
            Direction::West => cell.walls.bottom,
        }
    }
    fn front_wall(&self, cell: &Cell) -> bool {
        match self.direction {
            Direction::North => cell.walls.top,
            Direction::South => cell.walls.bottom,
            Direction::East => cell.walls.right,
            Direction::West => cell.walls.left,
        }
    }
    fn rotate_cw(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::South => self.direction = Direction::West,
            Direction::East => self.direction = Direction::South,
            Direction::West => self.direction = Direction::North,
        }
    }
    fn rotate_ccw(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
            Direction::West => self.direction = Direction::South,
        }
    }
    fn fwd(&mut self, board: &Board, cell: &Cell) -> usize {
        match self.direction {
            Direction::North => {
                if !cell.walls.top {
                    board.get_index(cell.x, cell.y - 1)
                } else {
                    board.get_index(cell.x, cell.y)
                }
            }
            Direction::South => {
                if !cell.walls.bottom {
                    board.get_index(cell.x, cell.y + 1)
                } else {
                    board.get_index(cell.x, cell.y)
                }
            }
            Direction::East => {
                if !cell.walls.right {
                    board.get_index(cell.x + 1, cell.y)
                } else {
                    board.get_index(cell.x, cell.y)
                }
            }
            Direction::West => {
                if !cell.walls.left {
                    board.get_index(cell.x - 1, cell.y)
                } else {
                    board.get_index(cell.x, cell.y)
                }
            }
        }
    }
}

impl Solver for WallFollower {
    fn step(&mut self, board: &Board) -> Result<State, String> {
        let index = *self.path.last().unwrap();
        if index == self.end {
            let mut clean_path: Vec<usize> = Vec::new();
            for path in &self.path {
                if clean_path.len() > 2 && clean_path.get(clean_path.len() - 2).unwrap() == path {
                    clean_path.pop();
                } else if clean_path.is_empty() || clean_path.last().unwrap() != path {
                    clean_path.push(*path);
                }
            }
            self.path = clean_path;
            return Ok(State::Done);
        }

        let current = &board.cells[index];

        if self.wall_left(current) {
            if self.front_wall(current) {
                self.walls
                    .push(Wall::new(self.direction, *self.path.last().unwrap()));
                self.rotate_cw();
                self.walls
                    .push(Wall::new(self.direction, *self.path.last().unwrap()));
            }
            let new_cell = self.fwd(board, current);
            self.walls
                .push(Wall::new(self.direction, *self.path.last().unwrap()));
            self.path.push(new_cell);
        } else {
            self.rotate_ccw();
            let new_cell = self.fwd(board, current);
            self.path.push(new_cell);
        }

        Ok(State::Solve)
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path
    }

    fn draw(&self, board: &Board) {
        for wall in &self.walls {
            match wall.direction {
                Direction::East => {
                    raylib::DrawLineEx(
                        Vector2 {
                            x: (board.x + board.cells[wall.cell].x * board.cell_size) as f32,
                            y: (board.y
                                + board.cells[wall.cell].y * board.cell_size
                                + self.distance) as f32,
                        },
                        Vector2 {
                            x: (board.x
                                + board.cells[wall.cell].x * board.cell_size
                                + board.cell_size) as f32,
                            y: (board.y
                                + board.cells[wall.cell].y * board.cell_size
                                + self.distance) as f32,
                        },
                        LINE_WIDTH,
                        Color::RED,
                    );
                }
                Direction::West => {
                    raylib::DrawLineEx(
                        Vector2 {
                            x: (board.x + board.cells[wall.cell].x * board.cell_size) as f32,
                            y: (board.y
                                + board.cells[wall.cell].y * board.cell_size
                                + board.cell_size
                                - self.distance) as f32,
                        },
                        Vector2 {
                            x: (board.x
                                + board.cells[wall.cell].x * board.cell_size
                                + board.cell_size) as f32,
                            y: (board.y
                                + board.cells[wall.cell].y * board.cell_size
                                + board.cell_size
                                - self.distance) as f32,
                        },
                        LINE_WIDTH,
                        Color::RED,
                    );
                }
                Direction::South => {
                    raylib::DrawLineEx(
                        Vector2 {
                            x: (board.x
                                + board.cells[wall.cell].x * board.cell_size
                                + board.cell_size
                                - self.distance) as f32,
                            y: (board.y + board.cells[wall.cell].y * board.cell_size) as f32,
                        },
                        Vector2 {
                            x: (board.x
                                + board.cells[wall.cell].x * board.cell_size
                                + board.cell_size
                                - self.distance) as f32,
                            y: (board.y
                                + board.cells[wall.cell].y * board.cell_size
                                + board.cell_size) as f32,
                        },
                        LINE_WIDTH,
                        Color::RED,
                    );
                }
                Direction::North => {
                    raylib::DrawLineEx(
                        Vector2 {
                            x: (board.x
                                + board.cells[wall.cell].x * board.cell_size
                                + self.distance) as f32,
                            y: (board.y + board.cells[wall.cell].y * board.cell_size) as f32,
                        },
                        Vector2 {
                            x: (board.x
                                + board.cells[wall.cell].x * board.cell_size
                                + self.distance) as f32,
                            y: (board.y
                                + board.cells[wall.cell].y * board.cell_size
                                + board.cell_size) as f32,
                        },
                        LINE_WIDTH,
                        Color::RED,
                    );
                }
            }
        }
    }
}
