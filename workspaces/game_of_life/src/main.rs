use raylib_egui_rs::color::Color;
use raylib_egui_rs::egui::EguiRaylib;
use raylib_egui_rs::math::*;
use raylib_egui_rs::raylib;

use rand::prelude::*;

use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::ffi::{CStr, c_char, c_int, c_void};

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const TITLE: &str = "Cellular Automata";
const CELL_SIZE: i32 = 4;

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = const { RefCell::new(None) };
}

struct Neighbours {}

impl Neighbours {
    pub fn moore(cells: &Vec<Vec<u8>>, x: i32, y: i32) -> usize {
        let mut neighbours = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let dx = (x as isize + j + cells[0].len() as isize) as usize % cells[0].len();
                let dy = (y as isize + i + cells.len() as isize) as usize % cells.len();
                neighbours += cells[dy][dx] as usize;
            }
        }
        neighbours
    }
}

struct GameState {
    screen_width: i32,
    screen_height: i32,
    cell_size: i32,
    width: i32,
    height: i32,
    birth: Vec<u8>,
    survive: Vec<u8>,
    data: Vec<Vec<u8>>,
    egui_raylib: EguiRaylib,
}

impl GameState {
    fn new() -> Self {
        let cell_size = CELL_SIZE;
        let screen_width = raylib::GetScreenWidth();
        let screen_height = raylib::GetScreenHeight();
        let width = screen_width / cell_size;
        let height = screen_height / cell_size;
        let mut data = vec![vec![0; width as usize]; height as usize];

        let mut rng = rand::rng();
        for y in 0..height {
            for x in 0..width {
                if rng.random_bool(0.5) {
                    data[y as usize][x as usize] = 1;
                }
            }
        }
        Self {
            cell_size,
            width,
            height,
            birth: vec![3],
            survive: vec![2, 3],
            data,
            screen_width,
            screen_height,
            egui_raylib: EguiRaylib::new(),
        }
    }

    fn set_birth(&mut self, birth: Vec<u8>) {
        self.data = vec![vec![0; self.width as usize]; self.height as usize];
        let mut rng = rand::rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if rng.random_bool(0.5) {
                    self.data[y as usize][x as usize] = 1;
                }
            }
        }
        self.birth = birth;
    }
    fn set_survive(&mut self, survive: Vec<u8>) {
        self.data = vec![vec![0; self.width as usize]; self.height as usize];
        let mut rng = rand::rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if rng.random_bool(0.5) {
                    self.data[y as usize][x as usize] = 1;
                }
            }
        }
        self.survive = survive;
    }
    fn set_cell_size(&mut self, size: i32) {
        self.cell_size = size;
        self.width = self.screen_width / size;
        self.height = self.screen_height / size;
        self.data = vec![vec![0; self.width as usize]; self.height as usize];
        let mut rng = rand::rng();
        for y in 0..self.height {
            for x in 0..self.width {
                if rng.random_bool(0.5) {
                    self.data[y as usize][x as usize] = 1;
                }
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
    GAME_STATE.with(|cell| {
        if let Some(state) = &mut *cell.borrow_mut() {
            update(state);
        }
    });
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn set_size(size: i32) {
    GAME_STATE.with(|cell| {
        if let Some(state) = cell.borrow_mut().as_mut() {
            state.set_cell_size(size);
        } else {
            panic!("can not get result");
        }
    })
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn set_birth(birth: *const c_char) {
    unsafe {
        let c_str = CStr::from_ptr(birth);
        let birth_str = c_str.to_string_lossy().into_owned();
        GAME_STATE.with(|cell| {
            if let Some(state) = cell.borrow_mut().as_mut() {
                let new_birth: Vec<u8> = birth_str
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                    .collect();
                if new_birth != state.birth {
                    state.set_birth(new_birth);
                }
            } else {
                panic!("can not get result");
            }
        })
    }
}

#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn set_survive(survive: *const c_char) {
    unsafe {
        let c_str = CStr::from_ptr(survive);
        let survive_str = c_str.to_string_lossy().into_owned();
        GAME_STATE.with(|cell| {
            if let Some(state) = cell.borrow_mut().as_mut() {
                let new_survive: Vec<u8> = survive_str
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                    .collect();
                if new_survive != state.survive {
                    state.set_survive(new_survive);
                }
            } else {
                panic!("can not get result");
            }
        })
    }
}

fn main() {
    // initialize raylib
    raylib::SetConfigFlags(
        raylib::ConfigFlags::FLAG_MSAA_4X_HINT as u32
            | raylib::ConfigFlags::FLAG_WINDOW_TRANSPARENT as u32,
    );
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(60);

    // initialize the maze
    let mut game_state = GameState::new();
    GAME_STATE.with(|cell| {
        *cell.borrow_mut() = Some(game_state);
    });

    // Main game loop
    #[cfg(not(target_arch = "wasm32"))]
    {
        while !raylib::WindowShouldClose() {
            GAME_STATE.with(|cell| {
                if let Some(state) = &mut *cell.borrow_mut() {
                    update(state);
                }
            });
        }
        raylib::CloseWindow();
    }
    #[cfg(target_arch = "wasm32")]
    {
        unsafe {
            emscripten_set_main_loop_arg(main_loop_wrapper, std::ptr::null_mut(), 0, 1);
        }
    }
}

fn update(state: &mut GameState) {
    raylib::BeginDrawing();
    raylib::ClearBackground(Color::BLANK);

    //draw the world
    for y in 0..state.height {
        for x in 0..state.width {
            let cell = Rectangle {
                x: (x * state.cell_size) as f32,
                y: (y * state.cell_size) as f32,
                width: (state.cell_size) as f32,
                height: (state.cell_size) as f32,
            };
            if state.data[y as usize][x as usize] == 1 {
                raylib::DrawRectangleRec(cell, Color::GREEN);
            }
        }
    }

    // calculate new generation
    let mut new_data = vec![vec![0; state.width as usize]; state.height as usize];
    for y in 0..state.height {
        for x in 0..state.width {
            let n = Neighbours::moore(&state.data, x, y);
            let alive = state.data[y as usize][x as usize] > 0;

            if (alive && state.survive.contains(&(n as u8)))
                || (!alive && state.birth.contains(&(n as u8)))
            {
                new_data[y as usize][x as usize] = 1;
            }
        }
    }
    state.data = new_data;

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut cell_size = state.cell_size;
        let mut reload = false;

        let mut birth_str: String = state.birth.iter().map(|n| n.to_string()).collect();
        let mut survive_str: String = state.survive.iter().map(|n| n.to_string()).collect();

        state.egui_raylib.draw(|egui_ctx| {
            egui::Window::new("Configuration").show(egui_ctx, |ui| {
                ui.label("Config:");
                egui::Grid::new("edit_grid")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        let width = 50.0;
                        ui.label("B:");
                        ui.add(egui::TextEdit::singleline(&mut birth_str).desired_width(width));
                        ui.label("S:");
                        ui.add(egui::TextEdit::singleline(&mut survive_str).desired_width(width));
                        ui.end_row();
                        ui.label("Size:");
                        if ui.add(egui::Slider::new(&mut cell_size, 1..=10)).changed() {
                            reload = true;
                        }
                        ui.end_row();
                    });
                ui.separator();
            });
        });

        if cell_size != state.cell_size {
            state.set_cell_size(cell_size);
        }

        let new_birth: Vec<u8> = birth_str
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();
        if new_birth != state.birth {
            state.set_birth(new_birth);
        }
        let new_survive: Vec<u8> = survive_str
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();
        if new_survive != state.survive {
            state.set_survive(new_survive);
        }
    }
    raylib::EndDrawing();
}
