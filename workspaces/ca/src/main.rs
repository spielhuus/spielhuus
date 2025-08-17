use raylib_egui_rs::color::Color;
use raylib_egui_rs::egui::EguiRaylib;
use raylib_egui_rs::math::*;
use raylib_egui_rs::raylib;

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_int, c_void};

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;
const TITLE: &str = "Cellular Automata";
const CELL_SIZE: i32 = 4;

struct GameState {
    screen_width: i32,
    screen_height: i32,
    cell_size: i32,
    width: i32,
    height: i32,
    rule: u8,
    lines: usize,
    data: Vec<Vec<u8>>,
    act_row: usize,
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
        data[0][(width / 2) as usize] = 1;
        let mut text_rule: Vec<u8> = String::from("90").into_bytes();
        text_rule.resize(10, 0);
        let mut text_size: Vec<u8> = format!("{CELL_SIZE}").into_bytes();
        text_size.resize(10, 0);
        Self {
            cell_size,
            width,
            height,
            rule: 90,
            lines: 1,
            data,
            screen_width,
            screen_height,
            act_row: 0,
            egui_raylib: EguiRaylib::new(),
        }
    }
    fn set_rule(&mut self, rule: u8) {
        self.data = vec![vec![0; self.width as usize]; self.height as usize];
        self.data[0][(self.width / 2) as usize] = 1;
        self.rule = rule;
        self.act_row = 0;
    }
    fn set_cell_size(&mut self, size: i32) {
        self.cell_size = size;
        self.width = self.screen_width / size;
        self.height = self.screen_height / size;
        self.data = vec![vec![0; self.width as usize]; self.height as usize];
        self.data[0][(self.width / 2) as usize] = 1;
        self.act_row = 0;
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
    raylib::SetConfigFlags(
        raylib::ConfigFlags::FLAG_MSAA_4X_HINT as u32
            | raylib::ConfigFlags::FLAG_WINDOW_TRANSPARENT as u32,
    );
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(240);

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
    raylib::ClearBackground(Color::BLANK);

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
                    Color::BLANK
                },
            );
        }
    }

    for _ in 0..state.lines {
        if state.act_row < state.height as usize - 1 {
            let current_row_index = state.act_row;
            let next_row_index = state.act_row + 1;
            let width = state.width as usize;

            for x in 0..width {
                // Get neighbor indices with wrapping
                let left_index = (x + width - 1) % width;
                let middle_index = x;
                let right_index = (x + 1) % width;

                // Get the values of the three cells in the row above
                let left_val = state.data[current_row_index][left_index];
                let middle_val = state.data[current_row_index][middle_index];
                let right_val = state.data[current_row_index][right_index];

                // Combine them into a 3-bit pattern (e.g., 110 becomes 6)
                let pattern = (left_val << 2) | (middle_val << 1) | right_val;

                // Use the pattern to check the rule bit
                // (state.rule >> pattern) & 1 will be 1 if the bit is set, 0 otherwise
                let new_val = (state.rule >> pattern) & 1;

                state.data[next_row_index][x] = new_val;
            }

            state.act_row += 1;
        } else {
            break;
        }
    }

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
                    ui.label("Lines:");
                    ui.add(egui::Slider::new(&mut state.lines, 1..=100));
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
