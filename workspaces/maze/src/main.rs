// use generative::{
//     constants::FONT,
//     maze::{
//         Board, Generator, PATH_COLOR, Solver, State,
//         generator::{
//             MazeAlgorithm, aldous_broder::AldousBroder, backtracking::Backtracking,
//             binary_tree::BinaryTree, eller::Eller, growing_tree::GrowingTree,
//             hunt_and_kill::HuntAndKill, kruskal::Kruskal, prim::Prim,
//             recursive_division::RecursiveDivision, sidewinder::Sidewinder, wilson::Wilson,
//         },
//         path,
//         solver::{self, PathfindingAlgorithm, genetic::PathEvolver},
//     },
// };

pub mod generator;
pub mod path;
pub mod solver;

use std::fmt;

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use raylib_egui_rs::raylib;
use raylib_egui_rs::{color::Color, egui::EguiRaylib};

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
            crate::Direction::North => {
                self.cells[cell].walls.top = false;
                self.cells[neighbor].walls.bottom = false;
            }
            crate::Direction::South => {
                self.cells[cell].walls.bottom = false;
                self.cells[neighbor].walls.top = false;
            }
            crate::Direction::East => {
                self.cells[cell].walls.right = false;
                self.cells[neighbor].walls.left = false;
            }
            crate::Direction::West => {
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

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_int, c_void};

use crate::{
    generator::{
        MazeAlgorithm, aldous_broder::AldousBroder, backtracking::Backtracking,
        binary_tree::BinaryTree, eller::Eller, growing_tree::GrowingTree,
        hunt_and_kill::HuntAndKill, kruskal::Kruskal, prim::Prim,
        recursive_division::RecursiveDivision, sidewinder::Sidewinder, wilson::Wilson,
    },
    solver::{PathfindingAlgorithm, genetic::PathEvolver},
};

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const BORDER: usize = 5;
const TITLE: &str = "Maze";

enum GuiState {
    None,
    Refresh,
    Generate,
    Solve,
    Reset,
    Step,
}

struct GameState {
    screen_width: i32,
    screen_height: i32,
    cell_count: usize,
    cell_size: usize,
    step_by_step: bool,
    step: bool,
    step_count: usize,
    steps: usize,
    error: Option<String>,
    board: Board,
    selected_generator: MazeAlgorithm,
    generator: Box<dyn Generator>,
    solver: Box<dyn Solver>,
    state: State,
    selected_solver: PathfindingAlgorithm,
    egui_raylib: EguiRaylib,
}

impl GameState {
    fn init_maze(&mut self) {
        self.board = Board::new(BORDER, self.cell_count, self.cell_size);
        self.init_solver();
        self.generator = match self.selected_generator {
            MazeAlgorithm::RecursiveBacktracker => Box::new(Backtracking::new()),
            MazeAlgorithm::Kruskal => Box::new(Kruskal::new(&self.board)),
            MazeAlgorithm::Eller => Box::new(Eller::new(&self.board)),
            MazeAlgorithm::Prim => Box::new(Prim::new(&self.board)),
            MazeAlgorithm::RecursiveDivision => Box::new(RecursiveDivision::new(&mut self.board)),
            MazeAlgorithm::AldousBroder => Box::new(AldousBroder::new(&self.board)),
            MazeAlgorithm::Wilson => Box::new(Wilson::new(&mut self.board)),
            MazeAlgorithm::HuntAndKill => Box::new(HuntAndKill::new(&mut self.board)),
            MazeAlgorithm::GrowingTree => Box::new(GrowingTree::new(&self.board)),
            MazeAlgorithm::BinaryTree => Box::new(BinaryTree::new()),
            MazeAlgorithm::Sidewinder => Box::new(Sidewinder::new(&mut self.board)),
        };
        self.step = false;
        self.step_count = 0;
    }
    fn init_solver(&mut self) {
        self.solver = match self.selected_solver {
            PathfindingAlgorithm::Dijkstra => {
                Box::new(solver::djikstra::Djikstra::new(&self.board))
            }
            PathfindingAlgorithm::RecursiveBacktracker => {
                Box::new(solver::backtracker::Backtracker::new(&self.board))
            }
            PathfindingAlgorithm::AStar => Box::new(solver::a_star::AStar::new(&self.board)),
            PathfindingAlgorithm::DeadEndFilling => {
                Box::new(solver::dead_end_filing::DeadEndFilling::new(&self.board))
            }
            PathfindingAlgorithm::WallFollower => {
                Box::new(solver::wall_follower::WallFollower::new(&self.board))
            }
            PathfindingAlgorithm::Genetic => {
                Box::new(solver::genetic::Genetic::<PathEvolver>::new(&self.board))
            }
        }
    }
}

#[allow(static_mut_refs)]
#[cfg(target_arch = "wasm32")]
type EmArgCallbackFunc = unsafe extern "C" fn(arg: *mut c_void);

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn emscripten_set_main_loop_arg(
        func: EmArgCallbackFunc,
        arg: *mut c_void,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" fn main_loop_wrapper(arg: *mut c_void) {
    let game_state = &mut *(arg as *mut GameState);
    update(game_state);
}

fn main() {
    // initialize raylib
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(60);

    // initialize the maze
    let board = Board::new(BORDER, 5, 5);
    let solver = Box::new(solver::djikstra::Djikstra::new(&board));
    let generator = Box::new(Backtracking::new());
    let mut game_state = GameState {
        screen_width: raylib::GetScreenWidth(),
        screen_height: raylib::GetScreenHeight(),
        cell_count: 5,
        cell_size: (raylib::GetScreenHeight() as usize - 2 * BORDER) / 5,
        step_by_step: false,
        step: false,
        step_count: 0,
        steps: 1,
        error: None,
        board,
        solver,
        generator,
        state: State::Wait,
        selected_generator: MazeAlgorithm::RecursiveBacktracker,
        selected_solver: PathfindingAlgorithm::Dijkstra,
        egui_raylib: EguiRaylib::new(),
    };

    let mut text_buffer: Vec<u8> = vec![20; 0];
    text_buffer.extend_from_slice(format!("   {}", game_state.cell_count).as_bytes());

    game_state.init_maze();

    // Main game loop
    #[cfg(not(target_arch = "wasm32"))]
    {
        while !raylib::WindowShouldClose() {
            update(&mut game_state);
        }
        raylib::CloseWindow();
    }
    #[cfg(target_arch = "wasm32")]
    {
        let boxed_state = Box::new(game_state);
        let state_ptr = Box::into_raw(boxed_state) as *mut c_void;
        unsafe {
            emscripten_set_main_loop_arg(main_loop_wrapper, state_ptr, 0, 1);
        }
    }
}

fn update(state: &mut GameState) {
    raylib::BeginDrawing();
    raylib::ClearBackground(Color::BLACK);

    // draw the board
    state.board.draw();

    match state.state {
        State::Wait => {}
        State::GenerationDone => {
            state.init_solver();
        }
        State::Generate => {
            state.generator.draw(&state.board);
            if !state.step_by_step || state.step {
                for _ in 0..state.steps {
                    state.state = state.generator.step(&mut state.board);
                    state.step_count += 1;
                    state.step = false;
                }
            }
        }
        State::Solve => {
            if let Some(error) = &state.error {
                //TODO
                raylib::DrawText(
                    error.as_str(),
                    state.screen_width - 350,
                    600,
                    24,
                    Color::RED,
                );
            } else if !state.step_by_step || state.step {
                for _ in 0..state.steps {
                    match state.solver.step(&state.board) {
                        Ok(sstate) => {
                            state.state = sstate;
                            state.step_count += 1;
                            state.step = false;
                        }
                        Err(str) => state.error = Some(str),
                    }
                    if state.state != State::Solve {
                        break;
                    }
                }
            }
            state.solver.draw(&state.board);
        }
        State::Done => path::draw_path(&state.board, state.solver.get_path(), PATH_COLOR),
    }

    let mut generator = state.selected_generator;
    let mut solver = state.selected_solver;
    let mut new_cell_count = state.cell_count;
    let mut gui_state = GuiState::None;
    state.egui_raylib.draw(|egui_ctx| {
        egui::SidePanel::right("Sidepanel")
            .exact_width(300.0)
            .show(egui_ctx, |ui| {
                // egui::Window::new("UI")
                //     .min_width(300.0)
                //     .show(egui_ctx, |ui| {
                ui.label("Config:");
                egui::Grid::new("edit_grid")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("Size:");
                        if ui
                            .add(egui::Slider::new(&mut new_cell_count, 10..=100).step_by(10.0))
                            .changed()
                        {
                            let new_cell_size =
                                (state.screen_height as usize - 2 * BORDER) / new_cell_count;
                            state.cell_count = new_cell_count;
                            state.cell_size = new_cell_size;
                            gui_state = GuiState::Refresh;
                        }
                        ui.end_row();
                        ui.label("Generator:");
                        egui::ComboBox::from_label("Generator")
                            .selected_text(format!("{generator}"))
                            .show_ui(ui, |ui| {
                                for algorithm in MazeAlgorithm::all_variants() {
                                    ui.selectable_value(
                                        &mut generator,
                                        *algorithm,
                                        algorithm.to_string(),
                                    );
                                }
                            });
                        ui.end_row();
                        ui.label("Solver:");
                        egui::ComboBox::from_label("Solver")
                            .selected_text(format!("{solver}"))
                            .show_ui(ui, |ui| {
                                for algorithm in PathfindingAlgorithm::all_variants() {
                                    ui.selectable_value(
                                        &mut solver,
                                        *algorithm,
                                        algorithm.to_string(),
                                    );
                                }
                            });
                        ui.end_row();
                        ui.label("Steps:");
                        ui.add(egui::Slider::new(&mut state.steps, 1..=100));
                        ui.end_row();
                    });
                ui.separator();
                if ui.button("generate").clicked() {
                    gui_state = GuiState::Generate;
                }
                if ui.button("solve").clicked() {
                    gui_state = GuiState::Solve;
                }
                if ui.button("step").clicked() {
                    gui_state = GuiState::Step;
                }
                if ui.button("reset").clicked() {
                    gui_state = GuiState::Reset;
                }
                ui.separator();
                ui.label("Info:");
                ui.label(format!("State: {}", state.state));
                ui.label(format!("Size: {}x{}", state.cell_count, state.cell_count));
                ui.label(format!("Step: {}", state.step_count));
                ui.label(format!(
                    "Solution length: {}",
                    state.solver.get_path().len()
                ));
                // });
            });
    });

    match gui_state {
        GuiState::Generate => {
            state.step_by_step = false;
            state.state = State::Generate;
            state.step = false;
            state.step_count = 0;
            state.init_maze();
        }
        GuiState::Solve => {
            state.error = None;
            state.state = State::Solve;
            state.step = false;
            state.step_count = 0;
            state.init_solver();
        }
        GuiState::Refresh => {
            state.init_maze();
        }
        GuiState::Reset => {
            state.init_maze();
        }
        GuiState::Step => {
            state.state = State::Solve;
            state.step_by_step = true;
            state.step = true;
            state.solver.draw(&state.board);
        }
        GuiState::None => {}
    }

    if solver != state.selected_solver {
        state.selected_solver = solver;
        state.init_solver();
        state.state = State::Wait;
    }
    if generator != state.selected_generator {
        state.selected_generator = generator;
        state.init_maze();
    }
    raylib::EndDrawing();
}
