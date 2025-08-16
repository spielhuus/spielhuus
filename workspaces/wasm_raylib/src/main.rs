use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
const SCREEN_WIDTH: usize = 100;
#[cfg(not(target_arch = "wasm32"))]
const SCREEN_WIDTH: usize = 1280;
#[cfg(target_arch = "wasm32")]
const SCREEN_HEIGHT: usize = 150;
#[cfg(not(target_arch = "wasm32"))]
const SCREEN_HEIGHT: usize = 720;
const TITLE: &str = "raylib example";
const SPEED: f32 = 100.0;

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_int, c_void};

use raylib_egui_rs::{color::Color, math::Vector2, raylib};

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
    GAME_STATE.with(|cell| {
        if let Some(game_state) = &mut *cell.borrow_mut() {
            update(game_state);
        }
    });
}

struct GameState {
    position: Vector2,
    force: Vector2,
    size: Vector2,
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = RefCell::new(None);
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
    let state = GameState {
        position: Vector2 { x: 0.0, y: 0.0 },
        force: Vector2 { x: SPEED, y: SPEED },
        size: Vector2 { x: 20.0, y: 20.0 },
    };

    GAME_STATE.with(|cell| {
        *cell.borrow_mut() = Some(state);
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
    let dt = raylib::GetFrameTime();
    raylib::ClearBackground(Color::BLANK);

    state.position = raylib::Vector2Add(
        state.position,
        raylib::Vector2Multiply(state.force, Vector2 { x: dt, y: dt }),
    );
    if state.position.x < 0.0 || state.position.x + state.size.x >= raylib::GetScreenWidth() as f32
    {
        state.force.x *= -1.0;
    }
    if state.position.y < 0.0 || state.position.y + state.size.y >= raylib::GetScreenHeight() as f32
    {
        state.force.y *= -1.0;
    }

    raylib::DrawRectangleV(state.position, state.size, Color::LIME);

    raylib::EndDrawing();
}
