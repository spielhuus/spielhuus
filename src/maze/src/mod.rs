pub mod generator;
pub mod path;
pub mod solver;

use std::fmt;

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use raylib_egui_rs::color::Color;
use raylib_egui_rs::raylib;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let index: u8 = rng.random_range(0..4);
        match index {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }
}

pub const WALL_COLOR: Color = Color {
    r: 100,
    g: 100,
    b: 100,
    a: 255,
};
pub const PATH_COLOR: Color = Color {
    r: 100,
    g: 255,
    b: 100,
    a: 255,
};
pub const CURSOR_COLOR: Color = Color {
    r: 125,
    g: 0,
    b: 17,
    a: 255,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    Wait,
    Generate,
    GenerationDone,
    Solve,
    Done,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Wait => write!(f, "Waiting"),
            State::Generate => write!(f, "Generating"),
            State::GenerationDone => write!(f, "Generation Done"),
            State::Solve => write!(f, "Solving"),
            State::Done => write!(f, "Done"),
        }
    }
}

pub trait Generator {
    fn step(&mut self, board: &mut Board) -> State;
    fn draw(&self, board: &Board);
}

pub trait Solver {
    fn step(&mut self, board: &Board) -> Result<State, String>;
    fn get_path(&self) -> &Vec<usize>;
    fn draw(&self, board: &Board);
}

#[derive(Clone, Debug)]
pub struct Walls {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl Default for Walls {
    fn default() -> Self {
        Self {
            left: true,
            right: true,
            top: true,
            bottom: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub visited: bool,
    pub walls: Walls,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            visited: false,
            walls: Walls::default(),
        }
    }

    /***
     * Gives the direction of the other cell to this one
     */
    pub fn direction(&self, other: &Cell) -> Direction {
        if self.x == other.x && self.y < other.y {
            Direction::South
        } else if self.x == other.x && self.y > other.y {
            Direction::North
        } else if self.x > other.x && self.y == other.y {
            Direction::West
        } else if self.x < other.x && self.y == other.y {
            Direction::East
        } else {
            panic!("whohwo")
        }
    }

    pub fn count_walls(&self) -> usize {
        let mut walls = 0;
        if self.walls.top {
            walls += 1;
        }
        if self.walls.bottom {
            walls += 1;
        }
        if self.walls.left {
            walls += 1;
        }
        if self.walls.right {
            walls += 1;
        }
        walls
    }

    pub fn is_dead_end(&self) -> bool {
        self.count_walls() == 3
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub path: Vec<usize>,
    pub board_size: usize,
    pub finish: bool,
    pub cell_size: usize,
    pub x: usize,
    pub y: usize,
}

impl Board {
    pub fn new(border: usize, board_size: usize, cell_size: usize) -> Self {
        let mut board = Self {
            cells: Vec::new(),
            path: vec![0],
            board_size,
            finish: false,
            cell_size,
            x: border,
            y: border,
        };
        board.init();
        board
    }

    fn init(&mut self) {
        for i in 0..self.board_size {
            for j in 0..self.board_size {
                self.cells.push(Cell::new(i, j));
            }
        }
        // self.cells[0].visited = true;
        self.cells[0].walls.left = false;
        self.cells.last_mut().unwrap().walls.right = false;
    }

    pub fn get_cell(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        let index = x * self.board_size + y;
        assert!(self.cells[index].x == x && self.cells[index].y == y,);
        index
    }

    /**
     * return the neighbors [top, bottom, right, left]
     */
    pub fn neighbors(&self, cell_index: usize) -> Vec<Option<usize>> {
        let mut res = Vec::<Option<usize>>::new();
        if self.cells[cell_index].y > 0 {
            res.push(Some(cell_index - 1));
        } else {
            res.push(None);
        }
        if self.cells[cell_index].y < self.board_size - 1 {
            res.push(Some(cell_index + 1));
        } else {
            res.push(None);
        }
        if self.cells[cell_index].x > 0 {
            res.push(Some(cell_index - self.board_size));
        } else {
            res.push(None);
        }
        if self.cells[cell_index].x < self.board_size - 1 {
            res.push(Some(cell_index + self.board_size));
        } else {
            res.push(None);
        }
        res
    }

    pub fn remove_wall(&mut self, cell: usize, neighbor: usize) {
        match self.cells[cell].direction(&self.cells[neighbor]) {
            crate::maze::Direction::North => {
                self.cells[cell].walls.top = false;
                self.cells[neighbor].walls.bottom = false;
            }
            crate::maze::Direction::South => {
                self.cells[cell].walls.bottom = false;
                self.cells[neighbor].walls.top = false;
            }
            crate::maze::Direction::East => {
                self.cells[cell].walls.right = false;
                self.cells[neighbor].walls.left = false;
            }
            crate::maze::Direction::West => {
                self.cells[cell].walls.left = false;
                self.cells[neighbor].walls.right = false;
            }
        }
        self.cells[cell].visited = true;
        self.cells[neighbor].visited = true;
    }

    pub fn draw(&self) {
        for cell in &self.cells {
            let x = self.x + cell.x * self.cell_size;
            let y = self.y + cell.y * self.cell_size;
            if cell.walls.top {
                raylib::DrawLine(
                    x as i32,
                    y as i32,
                    (x + self.cell_size) as i32,
                    y as i32,
                    WALL_COLOR,
                );
            }
            if cell.walls.right {
                raylib::DrawLine(
                    (x + self.cell_size) as i32,
                    y as i32,
                    (x + self.cell_size) as i32,
                    (y + self.cell_size) as i32,
                    WALL_COLOR,
                );
            }
            if cell.walls.bottom {
                raylib::DrawLine(
                    (x + self.cell_size) as i32,
                    (y + self.cell_size) as i32,
                    x as i32,
                    (y + self.cell_size) as i32,
                    WALL_COLOR,
                );
            }
            if cell.walls.left {
                raylib::DrawLine(
                    x as i32,
                    (y + self.cell_size) as i32,
                    x as i32,
                    y as i32,
                    WALL_COLOR,
                );
            }
            if !cell.visited {
                raylib::DrawRectangle(
                    x as i32,
                    y as i32,
                    (self.cell_size) as i32,
                    (self.cell_size) as i32,
                    Color {
                        r: 60,
                        g: 60,
                        b: 60,
                        a: 100,
                    },
                );
            }
        }
    }
}
