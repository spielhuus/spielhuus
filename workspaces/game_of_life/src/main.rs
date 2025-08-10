use raylib_egui_rs::color::Color;
use raylib_egui_rs::egui::EguiRaylib;
use raylib_egui_rs::math::*;
use raylib_egui_rs::raylib;

use rand::prelude::*;

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_int, c_void};

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const TITLE: &str = "Cellular Automata";
const CELL_SIZE: i32 = 4;

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
    rule: u8,
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
            rule: 90,
            data,
            screen_width,
            screen_height,
            egui_raylib: EguiRaylib::new(),
        }
    }
    fn set_rule(&mut self, rule: u8) {
        // self.data = vec![vec![0; self.width as usize]; self.height as usize];
        // self.data[0][(self.width / 2) as usize] = 1;
        // self.rule = rule;
        // self.act_row = 0;
    }
    fn set_cell_size(&mut self, size: i32) {
        self.cell_size = size;
        self.width = self.screen_width / size;
        self.height = self.screen_height / size;
        // self.data = vec![vec![0; self.width as usize]; self.height as usize];
        // self.data[0][(self.width / 2) as usize] = 1;
        // self.act_row = 0;
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
    raylib::InitWindow(
        SCREEN_WIDTH as i32,
        SCREEN_HEIGHT as i32,
        TITLE,
        // CString::new(TITLE).expect("cstr").as_ptr(),
    );
    raylib::SetTargetFPS(120);

    // initialize the maze
    let mut game_state = GameState::new();

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

    for y in 0..state.height {
        for x in 0..state.width {
            let cell = Rectangle {
                x: (x * state.cell_size) as f32,
                y: (y * state.cell_size) as f32,
                width: (state.cell_size) as f32,
                height: (state.cell_size) as f32,
            };
            raylib::DrawRectangleRec(
                cell,
                if state.data[y as usize][x as usize] == 1 {
                    Color::GREEN
                } else {
                    Color::BLACK
                },
            );
        }
    }

    let mut new_data = state.data.clone();
    for y in 0..state.height {
        for x in 0..state.width {
            let n = Neighbours::moore(&state.data, x, y);
            let alive = state.data[y as usize][x as usize] > 0;
            if alive && (n > 3 || n < 2) {
                new_data[y as usize][x as usize] = 0;
            } else if !alive && n == 3 {
                new_data[y as usize][x as usize] = 1;
            }
        }
    }
    state.data = new_data;

    let mut rule = state.rule;
    let mut cell_size = state.cell_size;
    let mut reload = false;
    state.egui_raylib.draw(|egui_ctx| {
        egui::Window::new("Configuration").show(egui_ctx, |ui| {
            ui.label("Config:");
            egui::Grid::new("edit_grid")
                .num_columns(2)
                .spacing([10.0, 4.0])
                .show(ui, |ui| {
                    ui.label("Rule:");
                    if ui.add(egui::Slider::new(&mut rule, 0..=254)).changed() {
                        reload = true;
                    }
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

    if rule != state.rule {
        state.set_rule(rule);
    }
    if cell_size != state.cell_size {
        state.set_cell_size(cell_size);
    }

    raylib::EndDrawing();
}
