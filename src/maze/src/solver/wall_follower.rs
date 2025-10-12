use crate::{solver::path, Board, Cell, Direction, MazeState, Solver, USE_WALL_FOLLOWER_PATH, WF_STRAIGHT_E, WF_STRAIGHT_N, WF_STRAIGHT_S, WF_STRAIGHT_W, WF_TURN_LEFT_E_TO_N, WF_TURN_LEFT_N_TO_W, WF_TURN_LEFT_S_TO_E, WF_TURN_LEFT_W_TO_S, WF_TURN_RIGHT_E_TO_S, WF_TURN_RIGHT_N_TO_E, WF_TURN_RIGHT_S_TO_W, WF_TURN_RIGHT_W_TO_N, WF_UTURN_ON_E_SIDE, WF_UTURN_ON_N_SIDE, WF_UTURN_ON_S_SIDE, WF_UTURN_ON_W_SIDE };

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
    pub walk_path: Vec<usize>,
    walls: Vec<Wall>,
    direction: Direction,
    distance: usize,
}

// use raylib_egui_rs::color::Color;
// use raylib_egui_rs::math::*;
// use raylib_egui_rs::raylib;



impl WallFollower {
    pub fn new(board: &Board) -> Self {
        println!("WallFollower::new, board size: {}", board.board_size);
        Self {
            end: board.get_index(board.board_size - 1, board.board_size - 1),
            path: vec![0],
            walk_path: vec![0],
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
    fn push_wall(&mut self) {
        self.walls
            .push(Wall::new(self.direction, *self.walk_path.last().unwrap()));
    }

    pub fn get_direction_from_to(&self, board: &Board, from_idx:  usize, to_idx: usize) -> Option<Direction> {
        if from_idx == to_idx {
            return None;
        }

        let from_cell = &board.cells[from_idx];
        let to_cell = &board.cells[to_idx];

        let dx = to_cell.x as isize - from_cell.x as isize;
        let dy = to_cell.y as isize - from_cell.y as isize;

        if dx == 1 && dy == 0 { Some(Direction::East) }
        else if dx == -1 && dy == 0 { Some(Direction::West) }
        else if dx == 0 && dy == 1 { Some(Direction::South) }
        else if dx == 0 && dy == -1 { Some(Direction::North) }
        else { None }
    }
}

impl Solver for WallFollower {
    fn step(&mut self, board: &mut Board) -> Result<MazeState, String> {
        let index = *self.walk_path.last().unwrap();
        if index == self.end {
            let mut clean_path: Vec<usize> = Vec::new();
            for path in &self.walk_path {
                if clean_path.len() > 2 && clean_path.get(clean_path.len() - 2).unwrap() == path {
                    clean_path.pop();
                    path::update_path(board, &clean_path);
                } else if clean_path.is_empty() || clean_path.last().unwrap() != path {
                    clean_path.push(*path);
                    path::update_path(board, &clean_path);
                }
            }
            self.path = clean_path;
            // self.update_gpu_for_wall_follower(board, &self.path);
            return Ok(MazeState::Done);
        }

        let current = &board.cells[index];

        if self.wall_left(current) {
            if self.front_wall(current) {
                self.push_wall();
                // self.walls
                //     .push(Wall::new(self.direction, *self.walk_path.last().unwrap()));
                self.rotate_cw();
                self.push_wall();
                // self.walls
                //     .push(Wall::new(self.direction, *self.walk_path.last().unwrap()));
            }
            let new_cell = self.fwd(board, current);
            self.push_wall();
            // self.walls
            //     .push(Wall::new(self.direction, *self.walk_path.last().unwrap()));
            self.walk_path.push(new_cell);
        } else {
            self.rotate_ccw();
            let new_cell = self.fwd(board, current);
            self.walk_path.push(new_cell);
        }

        Ok(MazeState::Solve)
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path

    }
}
