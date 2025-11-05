use crate::{solver::path, Board, Cell, Direction, MazeState, Solver, USE_WALL_FOLLOWER_PATH, WALL_BOTTOM, WALL_LEFT, WALL_RIGHT, WALL_TOP, WF_TURN_TOP_LEFT, WF_TURN_TOP_RIGHT, WF_TURN_BOTTOM_LEFT, WF_TURN_BOTTOM_RIGHT };

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

    fn get_wall(&self) -> u32 {
        match self.direction {
            Direction::North => WALL_LEFT,
            Direction::South => WALL_RIGHT,
            Direction::East => WALL_TOP,
            Direction::West => WALL_BOTTOM,
        }
    }

    fn get_turn(&self, old: Direction) -> u32 {
        match (old, self.direction) {
            (Direction::North, Direction::East) => WF_TURN_TOP_RIGHT,
            (Direction::North, Direction::West) => WF_TURN_TOP_LEFT,
            (Direction::South, Direction::East) => WF_TURN_BOTTOM_LEFT,
            (Direction::South, Direction::West) => WF_TURN_BOTTOM_RIGHT,
            (Direction::East, Direction::North) => WF_TURN_BOTTOM_LEFT,
            (Direction::East, Direction::South) => WF_TURN_TOP_RIGHT,
            (Direction::West, Direction::North) => WF_TURN_BOTTOM_RIGHT,
            (Direction::West, Direction::South) => WF_TURN_TOP_LEFT,
            (Direction::North, Direction::North) => todo!(),
            (Direction::North, Direction::South) => todo!(),
            (Direction::South, Direction::North) => todo!(),
            (Direction::South, Direction::South) => todo!(),
            (Direction::East, Direction::East) => todo!(),
            (Direction::East, Direction::West) => todo!(),
            (Direction::West, Direction::East) => todo!(),
            (Direction::West, Direction::West) => todo!(),
        }
    }
}

impl Solver for WallFollower {
    fn step(&mut self, board: &mut Board) -> Result<MazeState, String> {
        let index = *self.walk_path.last().unwrap();
        // did we reached the exit
        if index == self.end {
            let mut clean_path: Vec<usize> = Vec::new();
            for path in &self.walk_path {
                if clean_path.len() >= 2 && clean_path.get(clean_path.len() - 2).unwrap() == path {
                    let removed_cell_idx = *clean_path.last().unwrap();
                    clean_path.pop();
                    path::clear_direction(board, removed_cell_idx);
                    path::update_path(board, &clean_path);
                } else if clean_path.is_empty() || clean_path.last().unwrap() != path {
                    clean_path.push(*path);
                    path::update_path(board, &clean_path);
                }
            }
            self.path = clean_path;
            for (i, item) in self.path.iter().enumerate() {
                let x = board.get_cell(*item).x;
                let y = board.get_cell(*item).y;
                println!("{:04} {}x{} {:032b} {:032b}", i, x, y, board.gpu_data[*item][0], board.gpu_data[*item][1]); 
            }
            return Ok(MazeState::Done);
        }

        // search the next cell
        let current = &board.cells[index];
        let old_direction = self.direction;

        {
        println!("process cell: {}x{}, directon: {:?}", current.x, current.y, self.direction);
        }
        if self.wall_left(current) {
            // when there is a wall on the left
            if self.front_wall(current) {
                // when we stand in front of a wall
                println!("add wall before turn: {}", self.get_wall());
                println!("In front of wall: {}", self.get_wall());
                board.gpu_data[*self.walk_path.last().unwrap()][0] |= USE_WALL_FOLLOWER_PATH;
                board.gpu_data[*self.walk_path.last().unwrap()][1] |= self.get_wall();
                self.push_wall();
                self.rotate_cw();
                // board.gpu_data[*self.walk_path.last().unwrap()][1] |= self.get_wall();
                board.gpu_data[*self.walk_path.last().unwrap()][1] |= self.get_turn(old_direction);
                println!("after rotate cw: {:b}", board.gpu_data[*self.walk_path.last().unwrap()][1]);
                self.push_wall();
            }
            // go forward
            let new_cell = self.fwd(board, current);
            board.gpu_data[*self.walk_path.last().unwrap()][0] |= USE_WALL_FOLLOWER_PATH;
            println!("Add wall: {}", self.get_wall());
            board.gpu_data[*self.walk_path.last().unwrap()][1] |= self.get_wall();
            self.push_wall();
            println!("LEFT_WALL: {:032b} {:032b}", board.gpu_data[*self.walk_path.last().unwrap()][0], board.gpu_data[*self.walk_path.last().unwrap()][1]); 
            // self.walls
            //     .push(Wall::new(self.direction, *self.walk_path.last().unwrap()));
            self.walk_path.push(new_cell);

        } else {
            self.rotate_ccw();
             board.gpu_data[*self.walk_path.last().unwrap()][0] |= USE_WALL_FOLLOWER_PATH;
             board.gpu_data[*self.walk_path.last().unwrap()][1] |= self.get_turn(old_direction);
            println!("ROTATE_CCW: {:032b} {:032b}", board.gpu_data[*self.walk_path.last().unwrap()][0], board.gpu_data[*self.walk_path.last().unwrap()][1]); 

             println!("after rotate ccw: {} {}", self.walk_path.last().unwrap(), self.get_wall());
            let new_cell = self.fwd(board, current);
            self.walk_path.push(new_cell);
            board.gpu_data[*self.walk_path.last().unwrap()][0] |= USE_WALL_FOLLOWER_PATH;
            println!("Add wall: {}", self.get_wall());
            if self.wall_left(current) {
                board.gpu_data[*self.walk_path.last().unwrap()][1] |= self.get_wall();
                println!("NO_WALL   : {:032b} {:032b}", board.gpu_data[*self.walk_path.last().unwrap()][0], board.gpu_data[*self.walk_path.last().unwrap()][1]); 
            }
        }

        Ok(MazeState::Solve)
    }

    fn get_path(&self) -> &Vec<usize> {
        &self.path

    }
}
