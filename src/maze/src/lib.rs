pub mod generator;
pub mod solver;

#[cfg(feature = "egui")]
mod egui_utils;

use std::{iter, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use wgpu::util::DeviceExt;

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

use crate::{
    generator::{
        MazeAlgorithm, aldous_broder::AldousBroder, backtracking::Backtracking,
        binary_tree::BinaryTree, eller::Eller, growing_tree::GrowingTree,
        hunt_and_kill::HuntAndKill, kruskal::Kruskal, prim::Prim,
        recursive_division::RecursiveDivision, sidewinder::Sidewinder, wilson::Wilson,
    },
    solver::{PathfindingAlgorithm, genetic::PathEvolver},
};

const BORDER: usize = 5;

const INITIAL_CELL_COUNT: usize = 9;

pub const WALL_TOP: u32 = 1 << 0;
pub const WALL_RIGHT: u32 = 1 << 1;
pub const WALL_BOTTOM: u32 = 1 << 2;
pub const WALL_LEFT: u32 = 1 << 3;
pub const CELL_VISITED: u32 = 1 << 4;
pub const CELL_BACKTRACK: u32 = 1 << 5;
pub const CELL_CURSOR: u32 = 1 << 6;
pub const PATH_HORIZONTAL: u32 = 1 << 7;
pub const PATH_VERTICAL: u32 = 1 << 8;
pub const PATH_UP_LEFT: u32 = 1 << 9;
pub const PATH_UP_RIGHT: u32 = 1 << 10;
pub const PATH_DOWN_LEFT: u32 = 1 << 11;
pub const PATH_DOWN_RIGHT: u32 = 1 << 12;
pub const START_LEFT: u32 = 1 << 13;
pub const START_RIGHT: u32 = 1 << 14;
pub const START_UP: u32 = 1 << 15;
pub const START_DOWN: u32 = 1 << 16;
pub const END_LEFT: u32 = 1 << 17;
pub const END_RIGHT: u32 = 1 << 18;
pub const END_UP: u32 = 1 << 19;
pub const END_DOWN: u32 = 1 << 20;
pub const ARROW_LEFT: u32 = 1 << 21;
pub const ARROW_RIGHT: u32 = 1 << 22;
pub const ARROW_UP: u32 = 1 << 23;
pub const ARROW_DOWN: u32 = 1 << 24;
pub const CROSSED: u32 = 1 << 25;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MazeState {
    None,
    Wait,
    Generate,
    GenerationDone,
    Solve,
    Done,
}

pub trait Generator {
    fn step(&mut self, board: &mut Board) -> MazeState;
    fn draw(&self, board: &Board);
}

pub trait Solver {
    fn step(&mut self, board: &mut Board) -> Result<MazeState, String>;
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

#[derive(Debug, Copy, Clone)]
pub enum CellState {
    None,
    Visited,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Crossed,
    WallLeft,
    WallUp,
    WallRight,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub visited: bool,
    // pub backtrack: bool,
    pub walls: Walls,
    pub state: CellState,
    // pub cursor: bool,
    pub crossed: bool,
    pub arrow: Option<Direction>,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            visited: false,
            // backtrack: false,
            walls: Walls::default(),
            state: CellState::None,
            // cursor: false,
            crossed: false,
            arrow: None,
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
    pub gpu_data: Vec<u32>,
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
            gpu_data: vec![WALL_TOP|WALL_BOTTOM|WALL_RIGHT|WALL_LEFT; board_size.pow(2)],
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
                self.gpu_data[cell] &= !WALL_TOP;
                self.gpu_data[neighbor] &= !WALL_BOTTOM;
            }
            crate::Direction::South => {
                self.cells[cell].walls.bottom = false;
                self.cells[neighbor].walls.top = false;
                self.gpu_data[cell] &= !WALL_BOTTOM;
                self.gpu_data[neighbor] &= !WALL_TOP;
            }
            crate::Direction::East => {
                self.cells[cell].walls.right = false;
                self.cells[neighbor].walls.left = false;
                self.gpu_data[cell] &= !WALL_RIGHT;
                self.gpu_data[neighbor] &= !WALL_LEFT;
            }
            crate::Direction::West => {
                self.cells[cell].walls.left = false;
                self.cells[neighbor].walls.right = false;
                self.gpu_data[cell] &= !WALL_LEFT;
                self.gpu_data[neighbor] &= !WALL_RIGHT;
            }
        }
        self.cells[cell].visited = true;
        self.cells[neighbor].visited = true;
    }

    fn reset(&mut self) {
        self.cells.iter_mut().for_each(|cell| {
            cell.visited = false;
            // cell.backtrack = false;
            cell.walls.left = true;
            cell.walls.right = true;
            cell.walls.top = true;
            cell.walls.bottom = true;
        });
    }

    // fn create_gpu_data(&self) -> Vec<u32> {
    //     self.cells
    //         .iter()
    //         .map(|cell| {
    //             let mut cell_data: u32 = 0;
    //             if cell.walls.top {
    //                 cell_data |= WALL_TOP;
    //             }
    //             if cell.walls.right {
    //                 cell_data |= WALL_RIGHT;
    //             }
    //             if cell.walls.bottom {
    //                 cell_data |= WALL_BOTTOM;
    //             }
    //             if cell.walls.left {
    //                 cell_data |= WALL_LEFT;
    //             }
    //             if cell.visited {
    //                 cell_data |= CELL_VISITED;
    //             }
    //             if cell.backtrack {
    //                 cell_data |= CELL_BACKTRACK;
    //             }
    //             if cell.crossed {
    //                 cell_data |= CROSSED;
    //             }
    //             if cell.cursor {
    //                 cell_data |= CELL_CURSOR;
    //             }
    //             if let Some(arrow) = cell.arrow {
    //                 match arrow {
    //                     Direction::North => cell_data |= ARROW_UP,
    //                     Direction::South => cell_data |= ARROW_DOWN,
    //                     Direction::East => cell_data |= ARROW_RIGHT,
    //                     Direction::West => cell_data |= ARROW_LEFT,
    //                 }
    //             }
    //             cell_data
    //         })
    //         .collect()
    // }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    resolution: [f32; 2],
    time: f32,
    grid_size: u32,
}

enum GuiState {
    None,
    Refresh,
    Generate,
    Solve,
    Reset,
    // Step,
}

#[derive(Debug)]
pub enum PathDirection {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Horizontal,
    Vertical,
    StartLeft,
    StartDown,
    StartUp,
    StartRight,
    EndLeft,
    EndDown,
    EndUp,
    EndRight,
}

impl From<PathDirection> for u32 {
    fn from(direction: PathDirection) -> u32 {
        match direction {
            PathDirection::Horizontal => PATH_HORIZONTAL,
            PathDirection::Vertical => PATH_VERTICAL,
            PathDirection::UpLeft => PATH_UP_LEFT,
            PathDirection::UpRight => PATH_UP_RIGHT,
            PathDirection::DownLeft => PATH_DOWN_LEFT,
            PathDirection::DownRight => PATH_DOWN_RIGHT,
            PathDirection::StartLeft => START_LEFT,
            PathDirection::StartRight => START_RIGHT,
            PathDirection::StartUp => START_UP,
            PathDirection::StartDown => START_DOWN,
            PathDirection::EndLeft => END_LEFT,
            PathDirection::EndRight => END_RIGHT,
            PathDirection::EndUp => END_UP,
            PathDirection::EndDown => END_DOWN,
        }
    }
}

fn update_path(board: &mut Board, path: &[usize]) {
    if path.len() >= 3 {
        // second last step in path
        let direction = crate::direction(&board.cells[path[path.len()-2]], Some(&board.cells[path[path.len()-3]]), Some(&board.cells[path[path.len()-1]]));
        crate::clear_direction(board, path[path.len()-2]);
        board.gpu_data[path[path.len()-2]] |= <PathDirection as std::convert::Into<u32>>::into(direction);
        // end of path
        let direction = crate::direction(&board.cells[path[path.len()-1]], Some(&board.cells[path[path.len()-2]]), None);
        crate::clear_direction(board, path[path.len()-1]);
        board.gpu_data[path[path.len()-1]] |= <PathDirection as std::convert::Into<u32>>::into(direction);
    }
    if path.len() == 2 {
        let direction = crate::direction(&board.cells[path[path.len()-2]], None, Some(&board.cells[path[path.len()-1]]));
        crate::clear_direction(board, path[path.len()-2]);
        board.gpu_data[path[path.len()-2]] |= <PathDirection as std::convert::Into<u32>>::into(direction);
    }
}

fn clear_direction(board: &mut Board, cell: usize) {
    board.gpu_data[cell] &= !PATH_HORIZONTAL;
    board.gpu_data[cell] &= !PATH_VERTICAL;
    board.gpu_data[cell] &= !PATH_UP_LEFT;
    board.gpu_data[cell] &= !PATH_UP_RIGHT;
    board.gpu_data[cell] &= !PATH_DOWN_LEFT;
    board.gpu_data[cell] &= !PATH_DOWN_RIGHT;
    board.gpu_data[cell] &= !START_LEFT;
    board.gpu_data[cell] &= !START_RIGHT;
    board.gpu_data[cell] &= !START_UP;
    board.gpu_data[cell] &= !START_DOWN;
    board.gpu_data[cell] &= !END_LEFT;
    board.gpu_data[cell] &= !END_RIGHT;
    board.gpu_data[cell] &= !END_UP;
    board.gpu_data[cell] &= !END_DOWN;
    
}

fn direction(current: &Cell, prev: Option<&Cell>, next: Option<&Cell>) -> PathDirection {
    // +---+---+---+    current.y < previous.y &&
    // +   +   +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // +   + c + n +    current.x < next.x
    // +---+---+---+
    // +   + p +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   +   +   +    current.y < next.y &&
    // +---+---+---+    current.x < previous.x &&
    // +   + c + p +    currnet.x == next.x
    // +---+---+---+
    // +   + n +   +
    // +---+---+---+
    //
    // +---+---+---+    current.y < previous.y &&
    // +   +   +   +    current.y == next.y &&
    // +---+---+---+    currnet.x == previous.x &&
    // + n + c +   +    current.x > next.x
    // +---+---+---+
    // +   + p +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   +   +   +    current.y < next.y &&
    // +---+---+---+    current.x > previous.x &&
    // + p + c +   +    current.x == next.x
    // +---+---+---+
    // +   + n +   +
    // +---+---+---+

    //
    // +---+---+---+    current.y > previous.y &&
    // +   + p +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // +   + c + n +    currnet.x < next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   + n +   +    current.y > next.y &&
    // +---+---+---+    current.x < prvious.x &&
    // +   + c + p +    current.x == next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    //
    // +---+---+---+    current.y > previous.y &&
    // +   + p +   +    current.y == next.y &&
    // +---+---+---+    current.x == previous.x &&
    // + n + c +   +    current.x > next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    // +---+---+---+    current.y == previous.y &&
    // +   + n +   +    current.y > next.y &&
    // +---+---+---+    current.x > previous.x &&
    // + p + c +   +    current.x == next.x
    // +---+---+---+
    // +   +   +   +
    // +---+---+---+
    //
    if let (Some(prev), Some(next)) = (prev, next) {
        if current.x == next.x && current.x == prev.x {
            return PathDirection::Vertical;
        } else if current.y == next.y && current.y == prev.y {
            return PathDirection::Horizontal;
        } else if (current.y < prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x < next.x)
            || (current.y == prev.y
                && current.y < next.y
                && current.x < prev.x
                && current.x == next.x)
        {
            return PathDirection::DownRight;
        } else if (current.y < prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x > next.x)
            || (current.y == prev.y
                && current.y < next.y
                && current.x > prev.x
                && current.x == next.x)
        {
            return PathDirection::DownLeft;
        } else if (current.y > prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x < next.x)
            || (current.y == prev.y
                && current.y > next.y
                && current.x < prev.x
                && current.x == next.x)
        {
            return PathDirection::UpRight;
        } else if (current.y > prev.y
            && current.y == next.y
            && current.x == prev.x
            && current.x > next.x)
            || (current.y == prev.y
                && current.y > next.y
                && current.x > prev.x
                && current.x == next.x)
        {
            return PathDirection::UpLeft;
        }
    } else if let Some(next) = next {
        if next.x > current.x {
            return PathDirection::StartRight;
        } else if next.x < current.x {
            return PathDirection::StartLeft;
        } else if next.y > current.y {
            return PathDirection::StartDown;
        } else if next.y < current.y {
            return PathDirection::StartUp;
        } else {
            panic!("direction not found")
        }
    } else if let Some(prev) = prev {
        if prev.x > current.x {
            return PathDirection::EndLeft;
        } else if prev.x < current.x {
            return PathDirection::EndRight;
        } else if prev.y > current.y {
            return PathDirection::EndUp;
        } else if prev.y < current.y {
            return PathDirection::EndDown;
        } else {
            panic!("direction not found")
        }
    }
    panic!("direction not found for cell: {:?}", current)
}

pub struct State {
    board: Board,
    selected_generator: MazeAlgorithm,
    selected_solver: PathfindingAlgorithm,
    cell_count: usize,
    cell_size: usize,
    generator: Box<dyn Generator>,
    solver: Box<dyn Solver>,
    state: MazeState,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    window: Arc<Window>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    maze_buffer: wgpu::Buffer,
    #[cfg(not(target_arch = "wasm32"))]
    start_time: std::time::Instant,
    #[cfg(target_arch = "wasm32")]
    start_time: f64,
    #[cfg(target_arch = "wasm32")]
    canvas: HtmlCanvasElement,
    #[cfg(feature = "egui")]
    egui_renderer: egui_utils::EguiRenderer,
    #[cfg(feature = "egui")]
    scale_factor: f32,
}

impl State {
    async fn new(
        window: Arc<Window>,
        #[cfg(target_arch = "wasm32")] canvas: HtmlCanvasElement,
    ) -> anyhow::Result<State> {
        #[cfg(not(target_arch = "wasm32"))]
        let start_time = std::time::Instant::now();
        #[cfg(target_arch = "wasm32")]
        let start_time = web_sys::window().unwrap().performance().unwrap().now();

        let size = window.inner_size();
        let is_surface_configured = size.width > 0 && size.height > 0; 
        let board = Board::new(BORDER, INITIAL_CELL_COUNT, 5); //TODO: is the cell size used?
        let solver = Box::new(solver::djikstra::Djikstra::new(&board));
        let generator = Box::new(Backtracking::new());

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::default(),
                required_limits: adapter.limits(),
                ..Default::default()
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb()) 
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync, //Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        if is_surface_configured {
            surface.configure(&device, &config);
        }

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("grid_shader.wgsl").into()),
        });

        // Setup buffers and bind groups (no changes needed here)
        let uniforms = Uniforms {
            resolution: [0.0, 0.0],
            time: 0.0,
            grid_size: INITIAL_CELL_COUNT as u32,
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        // let initial_maze_data = board.create_gpu_data();
        let maze_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Maze Storage Buffer"),
            contents: bytemuck::cast_slice(&board.gpu_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: maze_buffer.as_entire_binding(),
                },
            ],
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        #[cfg(feature = "egui")]
        let egui_renderer = egui_utils::EguiRenderer::new(&device, config.format, None, 1, &window);

        Ok(Self {
            board,
            selected_generator: MazeAlgorithm::RecursiveBacktracker,
            selected_solver: PathfindingAlgorithm::RecursiveBacktracker,
            cell_count: INITIAL_CELL_COUNT,
            cell_size: (window.inner_size().width as usize - 2 * BORDER) / 5,
            generator,
            solver,
            state: MazeState::Wait,
            start_time,
            surface,
            device,
            queue,
            config,
            is_surface_configured,
            render_pipeline,
            window,
            maze_buffer,
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
            #[cfg(target_arch = "wasm32")]
            canvas,
            #[cfg(feature = "egui")]
            egui_renderer,
            #[cfg(feature = "egui")]
            scale_factor: 1.0,
        })
    }

    fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if let (KeyCode::Escape, true) = (code, is_pressed) {
            event_loop.exit()
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        if !self.is_surface_configured {
            return Ok(());
        }

        // update maze state
        let mut needs_next_frame = false;
        let mut maze_updated = false;
        match self.state {
            MazeState::None => {}
            MazeState::Wait => {}
            MazeState::GenerationDone => {}
            MazeState::Generate => {
                self.state = self.generator.step(&mut self.board);
                maze_updated = true;
                if self.state == MazeState::Generate {
                    needs_next_frame = true;
                }
            }
            MazeState::Solve => {
                self.state = self.solver.step(&mut self.board).unwrap();
                maze_updated = true;
                if self.state == MazeState::Solve {
                    needs_next_frame = true;
                }
            }
            MazeState::Done => {
                self.state = MazeState::Wait;
            }
        }

        if !self.is_surface_configured {
            return Ok(());
        }

        if maze_updated {
            // let mut maze_gpu_data = self.board.create_gpu_data();
            // get the path
            // if self.state == MazeState::Solve || self.state == MazeState::Done {
            //     let path = self.solver.get_path();
            //     if path.len() > 1 {
            //         for (i, item) in path.iter().enumerate() {
            //             let prev = if i > 0 { path.get(i - 1) } else { None };
            //             let next = path.get(i + 1); // get handles out-of-bounds by returning None
            //             let direction = direction(
            //                 &self.board.cells[*item],
            //                 if let Some(prev) = prev {
            //                     Some(&self.board.cells[*prev])
            //                 } else {
            //                     None
            //                 },
            //                 if let Some(next) = next {
            //                     Some(&self.board.cells[*next])
            //                 } else {
            //                     None
            //                 },
            //             );
            //             match direction {
            //                 PathDirection::UpLeft => maze_gpu_data[*item] |= PATH_UP_LEFT,
            //                 PathDirection::UpRight => maze_gpu_data[*item] |= PATH_UP_RIGHT,
            //                 PathDirection::DownLeft => maze_gpu_data[*item] |= PATH_DOWN_LEFT,
            //                 PathDirection::DownRight => maze_gpu_data[*item] |= PATH_DOWN_RIGHT,
            //                 PathDirection::Horizontal => maze_gpu_data[*item] |= PATH_HORIZONTAL,
            //                 PathDirection::Vertical => maze_gpu_data[*item] |= PATH_VERTICAL,
            //                 PathDirection::StartLeft => maze_gpu_data[*item] |= START_LEFT,
            //                 PathDirection::StartDown => maze_gpu_data[*item] |= START_DOWN,
            //                 PathDirection::StartUp => maze_gpu_data[*item] |= START_UP,
            //                 PathDirection::StartRight => maze_gpu_data[*item] |= START_RIGHT,
            //                 PathDirection::EndLeft => maze_gpu_data[*item] |= END_LEFT,
            //                 PathDirection::EndDown => maze_gpu_data[*item] |= END_DOWN,
            //                 PathDirection::EndUp => maze_gpu_data[*item] |= END_UP,
            //                 PathDirection::EndRight => maze_gpu_data[*item] |= END_RIGHT,
            //             }
            //         }
            //     }
            // }
            self.queue
                .write_buffer(&self.maze_buffer, 0, bytemuck::cast_slice(&self.board.gpu_data));
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Main Render Encoder"),
            });

        // draw the maze
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Maze Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            // Clear the background
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Update uniforms for the maze shader
            #[cfg(not(target_arch = "wasm32"))]
            let elapsed = self.start_time.elapsed().as_secs_f32();
            #[cfg(target_arch = "wasm32")]
            let elapsed = {
                use web_sys::window;
                let now = window().unwrap().performance().unwrap().now();
                (now - self.start_time) as f32 / 1000.0
            };

            let updated_uniforms = Uniforms {
                resolution: [
                    self.window.inner_size().width as f32,
                    self.window.inner_size().height as f32,
                ],
                time: elapsed,
                grid_size: self.cell_count as u32,
            };
            self.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(&[updated_uniforms]),
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }

        #[cfg(feature = "egui")]
        {
            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [self.config.width, self.config.height],
                pixels_per_point: self.window.scale_factor() as f32,
            };

            {
                let mut new_generate = false;
                let mut new_solve = false;
                let mut generator = self.selected_generator;
                let mut solver = self.selected_solver;
                let mut new_cell_count = self.cell_count;
                let mut gui_state = GuiState::None;
                self.egui_renderer.begin_frame(&self.window);

                egui_winit::egui::Window::new("Maze Controls")
                    .resizable(true)
                    .min_width(300.0)
                    .show(self.egui_renderer.context(), |ui| {
                        ui.label("Config:");
                        egui::Grid::new("edit_grid")
                            .num_columns(2)
                            .spacing([10.0, 4.0])
                            .show(ui, |ui| {
                                ui.label("Size:");
                                if ui
                                    .add(
                                        egui::Slider::new(&mut new_cell_count, 10..=100)
                                            .step_by(10.0),
                                    )
                                    .changed()
                                {
                                    let new_cell_size = (self.window.inner_size().height as usize
                                        - 2 * BORDER)
                                        / new_cell_count;
                                    self.cell_count = new_cell_count;
                                    self.cell_size = new_cell_size;
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
                                // ui.label("Steps:");
                                // ui.add(egui::Slider::new(&mut state.steps, 1..=100));
                                // ui.end_row();
                            });
                        ui.separator();
                        if ui.button("generate").clicked() {
                            new_generate = true;
                            gui_state = GuiState::Generate;
                        }
                        if ui.button("solve").clicked() {
                            new_solve = true;
                            gui_state = GuiState::Solve;
                        }
                        // if ui.button("step").clicked() {
                        //     gui_state = MazeState::Step;
                        // }
                        // if ui.button("reset").clicked() {
                        //     gui_state = MazeState::Reset;
                        // }
                        ui.separator();
                        ui.label("Info:");
                        // ui.label(format!("State: {}", self.state));
                        // ui.label(format!("Size: {}x{}", state.cell_count, state.cell_count));
                        // ui.label(format!("Step: {}", state.step_count));
                        // ui.label(format!(
                        //     "Solution length: {}",
                        //     state.solver.get_path().len()
                        // ));
                    });

                if solver != self.selected_solver {
                    self.selected_solver = solver;
                    self.init_solver();
                    self.state = MazeState::Wait;
                }
                if generator != self.selected_generator {
                    self.selected_generator = generator;
                    self.init_maze();
                }

                match gui_state {
                    GuiState::Generate => {
                        self.state = MazeState::Generate;
                        self.init_maze();
                        self.window.request_redraw();
                    }
                    GuiState::Solve => {
                        self.state = MazeState::Solve;
                        self.init_solver();
                        self.window.request_redraw();
                    }
                    GuiState::Refresh => {
                        self.init_maze();
                        self.state = MazeState::Generate;
                        self.window.request_redraw();
                    }
                    GuiState::Reset => {
                        self.init_maze();
                        self.window.request_redraw();
                    }
                    // GuiState::Step => {
                    //     self.state = State::Solve;
                    //     self.step_by_step = true;
                    //     self.step = true;
                    //     self.solver.draw(&self.board);
                    // }
                    GuiState::None => {}
                }
            }

            self.egui_renderer.end_frame_and_draw(
                &self.device,
                &self.queue,
                &mut encoder,
                &self.window,
                &view,
                screen_descriptor,
            );
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        #[cfg(feature = "egui")]
        self.window.request_redraw();

        #[cfg(target_arch = "wasm32")]
        if needs_next_frame {
            self.window.request_redraw();
        }

        Ok(())
    }

    fn init_maze(&mut self) {
        self.board = Board::new(BORDER, self.cell_count, self.cell_size);
        self.board.reset();
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
        // self.step = false;
        // self.step_count = 0;
        // TODO 
        // let mut new_maze_data = self.board.create_gpu_data();
        self.maze_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Maze Storage Buffer (resized)"),
                contents: bytemuck::cast_slice(&self.board.gpu_data),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });
        self.uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group (recreated)"),
            layout: &self.uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.maze_buffer.as_entire_binding(),
                },
            ],
        });
        self.init_solver();
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
                Box::new(solver::dead_end_filing::DeadEndFilling::new(&mut self.board))
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

pub enum UserEvent {
    StateInitialized(State),
    GenerateMaze,
    SolveMaze,
    Generator(MazeAlgorithm),
    Solver(PathfindingAlgorithm),
    Size(usize),
}

impl std::fmt::Debug for UserEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserEvent::StateInitialized(_) => write!(f, "StateInitialized(<State object>)"),
            UserEvent::GenerateMaze => write!(f, "GenerateMaze"),
            UserEvent::SolveMaze => write!(f, "SolveMaze"),
            UserEvent::Generator(maze_algorithm) => write!(f, "Solver({})", maze_algorithm),
            UserEvent::Solver(pathfinding_algorithm) => write!(f, "PathFindingAlogrithm({})", pathfinding_algorithm),
            UserEvent::Size(size) => write!(f, "Size({})", size),
        }
    }
}

pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<UserEvent>>, 
    state: Option<State>,
    #[cfg(target_arch = "wasm32")]
    _event_closures: Vec<Closure<dyn FnMut(web_sys::Event)>>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<UserEvent>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
            #[cfg(target_arch = "wasm32")]
            _event_closures: Vec::new(),
        }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        //create the html ui
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use web_sys::{
                console, Document, Element, HtmlButtonElement, HtmlFormElement, HtmlOptionElement,
                HtmlSelectElement, HtmlInputElement, Window,
            };
            
            let proxy = self.proxy.as_ref().unwrap().clone();
            let on_generate_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlButtonElement>() {
                    event.prevent_default();
                    log::info!("generate clicked");
                    if let Err(e) = proxy.send_event(UserEvent::GenerateMaze) {
                        log::error!("Failed to send GenerateMaze event: {:?}", e);
                    }
                }
            });
            
            let proxy = self.proxy.as_ref().unwrap().clone();
            let on_solve_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlButtonElement>() {
                    if let Some(input_element) = target.dyn_ref::<HtmlButtonElement>() {
                        event.prevent_default();
                        log::info!("solve clicked");
                        if let Err(e) = proxy.send_event(UserEvent::SolveMaze) {
                            log::error!("Failed to send SolveMaze event: {:?}", e);
                        }
                    }
                }
            });

            let proxy = self.proxy.as_ref().unwrap().clone();
            let on_select_generator_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlSelectElement>() {
                    let value_str = input_element.value();
                    log::info!("generator selected: {}", value_str);
                    if let Err(e) = proxy.send_event(UserEvent::Generator(
                            match value_str.as_str() {
                                 "1" => MazeAlgorithm::RecursiveBacktracker,
                                 "2" => MazeAlgorithm::Kruskal,
                                 "3" => MazeAlgorithm::Eller,
                                 "4" => MazeAlgorithm::Prim,
                                 "5" => MazeAlgorithm::RecursiveDivision,
                                 "6" => MazeAlgorithm::AldousBroder,
                                 "7" => MazeAlgorithm::Wilson,
                                 "8" => MazeAlgorithm::HuntAndKill,
                                 "9" => MazeAlgorithm::GrowingTree,
                                 "10" => MazeAlgorithm::BinaryTree,
                                 "11" => MazeAlgorithm::Sidewinder,
                                 _ => MazeAlgorithm::RecursiveBacktracker,
                            }
                    
                    )) {
                        log::error!("Failed to send SolveMaze event: {:?}", e);
                    }
                }
            });

            let proxy = self.proxy.as_ref().unwrap().clone();
            let on_select_solver_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlSelectElement>() {
                    let value_str = input_element.value();
                    log::info!("solver selected: {}", value_str);
                    if let Err(e) = proxy.send_event(UserEvent::Solver(
                            match value_str.as_str() {
                                "1" => PathfindingAlgorithm::Dijkstra,
                                "2" => PathfindingAlgorithm::RecursiveBacktracker,
                                "3" => PathfindingAlgorithm::AStar,
                                "4" => PathfindingAlgorithm::DeadEndFilling,
                                "5" => PathfindingAlgorithm::WallFollower,
                                "6" => PathfindingAlgorithm::Genetic,
                                _ => PathfindingAlgorithm::Dijkstra,
                            }
                    
                    )) {
                        log::error!("Failed to send SolveMaze event: {:?}", e);
                    }
                }
            });

            let proxy = self.proxy.as_ref().unwrap().clone();
            let on_select_size_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
                let target = event.target().expect("Event should have a target");
                if let Some(input_element) = target.dyn_ref::<HtmlInputElement>() {
                    let value_str = input_element.value();
                    log::info!("size selected: {}", value_str);
                    if let Ok(size) = value_str.parse::<usize>() {
                        if size >= 1 && size <= 10 {
                            if let Err(e) = proxy.send_event(UserEvent::Size(size * 10 - 1)) {
                                log::error!("Failed to send SolveMaze event: {:?}", e);
                            }
                        }
                    }
                }
            });


            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");

            //get the text input
            let generate_button = document
                .get_element_by_id("generate")
                .expect("should have an input with id 'generate'");
            let generate_button_element: HtmlButtonElement = generate_button.dyn_into().map_err(|_| ()).unwrap();
            generate_button_element
                .add_event_listener_with_callback(
                    "click",
                    on_generate_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_generate_callback);


            let solve_button = document
                .get_element_by_id("solve")
                .expect("should have an input with id 'solve'");
            let solve_button_element: HtmlButtonElement = solve_button.dyn_into().map_err(|_| ()).unwrap();
            solve_button_element
                .add_event_listener_with_callback(
                    "click",
                    on_solve_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_solve_callback);

            let generator_choice = document
                .get_element_by_id("generator")
                .expect("should have an input with id 'generator'");
            let generator_choice_element: HtmlSelectElement = generator_choice.dyn_into().map_err(|_| ()).unwrap();
            generator_choice_element
                .add_event_listener_with_callback(
                    "change",
                    on_select_generator_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_select_generator_callback);

            let solver_choice = document
                .get_element_by_id("solver")
                .expect("should have an input with id 'solver'");
            let solver_choice_element: HtmlSelectElement = solver_choice.dyn_into().map_err(|_| ()).unwrap();
            solver_choice_element
                .add_event_listener_with_callback(
                    "change",
                    on_select_solver_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_select_solver_callback);

            let size_choice = document
                .get_element_by_id("size")
                .expect("should have an input with id 'size'");
            let size_choice_element: HtmlInputElement = size_choice.dyn_into().map_err(|_| ()).unwrap();
            size_choice_element
                .add_event_listener_with_callback(
                    "input",
                    on_select_size_callback.as_ref().unchecked_ref(),
                )
                .unwrap();
            self._event_closures.push(on_select_size_callback);
        }

        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        let canvas = {
            // Use a block to scope variables
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "shader";

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
            let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

            window_attributes = window_attributes.with_canvas(Some(canvas.clone()));
            window_attributes.active = false;
            canvas
        };

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                 let new_state = State::new(window, canvas)
                                .await
                                .expect("Unable to create canvas!!!");
                // Send the correct enum variant
                assert!(proxy.send_event(UserEvent::StateInitialized(new_state)).is_ok());

                    // assert!(
                    //     proxy
                    //         .send_event(
                    //             State::new(window, canvas)
                    //                 .await
                    //                 .expect("Unable to create canvas!!!")
                    //         )
                    //         .is_ok()
                    // )
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: UserEvent) {
        match event {
            UserEvent::StateInitialized(mut initial_state) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        initial_state.resize(
                            initial_state.window.inner_size().width,
                            initial_state.window.inner_size().height,
                        );
                        initial_state.window.request_redraw();
                    }
                    self.state = Some(initial_state);
                }
            UserEvent::GenerateMaze => {
                    if let Some(state) = &mut self.state {
                        // This is your original code, now in a safe context!
                        state.init_maze();
                        state.state = MazeState::Generate;
                        state.window.request_redraw();
                    } else {
                        log::warn!("GenerateMaze event received before state was initialized.");
                    }
                }
            UserEvent::SolveMaze => {
                    if let Some(state) = &mut self.state {
                        state.state = MazeState::Solve;
                        state.init_solver();
                        state.window.request_redraw();
                    } else {
                        log::warn!("SolveMaze event received before state was initialized.");
                    }
                }
            UserEvent::Generator(maze_algorithm) => {
                    if let Some(state) = &mut self.state {
                        state.selected_generator = maze_algorithm;
                        state.init_maze();
                    } else {
                        log::warn!("SolveMaze event received before state was initialized.");
                    }
            },
            UserEvent::Solver(pathfinding_algorithm) => {
                    if let Some(state) = &mut self.state {
                        state.selected_solver = pathfinding_algorithm;
                        state.init_solver();
                    } else {
                        log::warn!("SolveMaze event received before state was initialized.");
                    }
            },
            UserEvent::Size(size) => {
                    if let Some(state) = &mut self.state {
                        state.cell_count = size;
                        state.init_maze();
                        state.window.request_redraw();
                    } else {
                        log::warn!("SolveMaze event received before state was initialized.");
                    }
            },
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        #[cfg(feature = "egui")]
        state.egui_renderer.handle_input(&state.window, &event);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.render().unwrap();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                // #[cfg(target_arch = "wasm32")]
                // if code == KeyCode::F11 && key_state.is_pressed() {
                //     state.toggle_fullscreen();
                // }
                state.handle_key(event_loop, code, key_state.is_pressed());
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // ensure the `wgpu::Surface` is dropped first.
        self.state.take();
    }
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap_throw();

    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap_throw();
    Ok(())
}
