mod nikolaus;

#[cfg(not(target_arch = "wasm32"))]
use egui::ScrollArea;

use nikolaus::Result;
use raylib_egui_rs::{color::Color, egui::EguiRaylib, math::Vector2, raylib};

use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_char, c_int, c_void};

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
unsafe extern "C" fn main_loop_wrapper(_arg: *mut c_void) {
    GAME_STATE.with(|cell| {
        if let Some(game_state) = &mut *cell.borrow_mut() {
            update(game_state);
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn select_solution(index: i32);
}

#[cfg(target_arch = "wasm32")]
const SCREEN_WIDTH: usize = 100;
#[cfg(not(target_arch = "wasm32"))]
const SCREEN_WIDTH: usize = 1280;
#[cfg(target_arch = "wasm32")]
const SCREEN_HEIGHT: usize = 150;
#[cfg(not(target_arch = "wasm32"))]
const SCREEN_HEIGHT: usize = 720;
const STROKE: Color = Color {
    r: 0xe7,
    g: 0x4c,
    b: 0x3c,
    a: 0xff,
};
const TITLE: &str = "Das kleine Haus des Nikolaus";
const SPEED: f32 = 0.03;
#[cfg(not(target_arch = "wasm32"))]
const SCALE: f32 = 2.0;
#[cfg(not(target_arch = "wasm32"))]
const X: f32 = 400.0;
#[cfg(not(target_arch = "wasm32"))]
const Y: f32 = 20.0;

struct GameState {
    positions: [Vector2; 5],
    data: Result,
    start: usize,
    selected: usize,
    #[cfg(not(target_arch = "wasm32"))]
    egui_raylib: EguiRaylib,
    step: usize,
    scale: f32,
}

impl GameState {
    fn new() -> Self {
        let mut positions: [Vector2; 5] = [
            Vector2 { x: 1.0, y: 150.0 },
            Vector2 { x: 1.0, y: 50.0 },
            Vector2 { x: 50.0, y: 1.0 },
            Vector2 { x: 100.0, y: 50.0 },
            Vector2 { x: 100.0, y: 150.0 },
        ];
        #[cfg(not(target_arch = "wasm32"))]
        {
            positions = positions.map(|p| Vector2 {
                x: p.x * SCALE + X,
                y: p.y * SCALE + Y,
            });
        }

        Self {
            positions,
            data: Vec::new(),
            start: 0,
            selected: 0,
            #[cfg(not(target_arch = "wasm32"))]
            egui_raylib: EguiRaylib::new(),
            step: 1,
            scale: 0.0,
        }
    }
    fn set_data(&mut self, data: Result) {
        self.data = data;
    }
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = const { RefCell::new(None) };
    static JSON_BUFFER: RefCell<Option<String>> = const { RefCell::new(None) };
}

#[unsafe(no_mangle)]
pub extern "C" fn solutions() -> i32 {
    GAME_STATE.with(|cell| {
        if let Some(state) = cell.borrow().as_ref() {
            state.data.len() as i32
        } else {
            panic!("can not get result");
        }
    })
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn get_steps() -> *const c_char {
    GAME_STATE.with(|cell| {
        if let Some(state) = cell.borrow().as_ref() {
            println!("get steps: {}", state.data.len());
            let data = &state.data;
            let mut json = String::from("[");
            for (i, path) in data.iter().enumerate() {
                json.push('[');
                for (j, val) in path.0.iter().enumerate() {
                    json.push_str(&val.to_string());
                    if j < path.0.len() - 1 {
                        json.push(',');
                    }
                }
                json.push(']');
                if i < data.len() - 1 {
                    json.push(',');
                }
            }
            json.push(']');

            JSON_BUFFER.with(|buf| {
                *buf.borrow_mut() = Some(json);
                buf.borrow().as_ref().unwrap().as_ptr()
            }) as *const c_char
        } else {
            panic!("can not get result");
        }
    })
}

pub fn main() {
    // initialize raylib
    raylib::SetConfigFlags(
        raylib::ConfigFlags::FLAG_MSAA_4X_HINT as u32
            | raylib::ConfigFlags::FLAG_WINDOW_TRANSPARENT as u32,
    );
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(60);

    // initialize the maze
    let mut game_state = GameState::new();
    game_state.set_data(nikolaus::nikolaus(game_state.start));
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

    if !state.data.is_empty() {
        for i in 0..state.step {
            let item = state.data[state.selected].1[i];
            if i < state.step - 1 {
                raylib::DrawLineEx(
                    state.positions[item.0],
                    state.positions[item.1],
                    4.0,
                    STROKE,
                );
            } else {
                let x1 = state.positions[item.0].x;
                let y1 = state.positions[item.0].y;
                let x2 = state.positions[item.1].x;
                let y2 = state.positions[item.1].y;
                let x_new = x1 + (state.scale * (x2 - x1));
                let y_new = y1 + (state.scale * (y2 - y1));
                raylib::DrawLineEx(
                    state.positions[item.0],
                    Vector2 { x: x_new, y: y_new },
                    6.0,
                    STROKE,
                );
            }
        }
        if state.scale < 1.0 {
            state.scale += SPEED;
        } else {
            state.scale = 0.0;
            state.step = (state.step % 8) + 1;
            if state.step == 1 {
                state.selected = (state.selected + 1) % state.data.len();

                #[cfg(target_arch = "wasm32")]
                unsafe {
                    select_solution(state.selected as i32);
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    state.egui_raylib.draw(|egui_ctx| {
        egui::Window::new("Configuration").show(egui_ctx, |ui| {
            ui.label("Config:");
            ui.label(format!("Start: {}", state.start));
            ScrollArea::vertical()
                .id_salt("start")
                .max_height(100.0)
                .show(ui, |ui| {
                    for start in 0..9 {
                        let is_selected = start == state.start;
                        if ui
                            .selectable_label(is_selected, format!("{start}"))
                            .clicked()
                        {
                            state.start = start;
                            state.data = nikolaus::nikolaus(state.start);
                        }
                    }
                });
            ui.separator();
            ui.label(format!("Results: {}", state.data.len()));
            ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                for (i, result) in state.data.iter().enumerate() {
                    let is_selected = i == state.selected;
                    if ui
                        .selectable_label(is_selected, format!("{i} = {:?}", result.0))
                        .clicked()
                    {
                        state.selected = i;
                    }
                }
            });
        });
    });
    raylib::EndDrawing();
}
