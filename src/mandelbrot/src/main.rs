use std::ops::{Add, Mul};

use raylib_egui_rs::color;
use raylib_egui_rs::color::Color;
use raylib_egui_rs::egui::EguiRaylib;
use raylib_egui_rs::math::*;
use raylib_egui_rs::raylib;

use rand::prelude::*;

use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::ffi::{CStr, c_char, c_int, c_void};

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const TITLE: &str = "Cellular Automata";

const RE_START: f64 = -2.0;
const RE_END: f64 = 1.7;
const IM_START: f64 = -1.5;
const IM_END: f64 = 1.5;

const MAX_ITER: u32 = 80;

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = const { RefCell::new(None) };
}

struct GameState {
    screen_width: i32,
    screen_height: i32,
    data: Vec<Vec<Color>>,
    egui_raylib: EguiRaylib,
}

impl GameState {
    fn new() -> Self {
        let screen_width = raylib::GetScreenWidth();
        let screen_height = raylib::GetScreenHeight();

        Self {
            data: GameState::mandelbrot(screen_width, screen_height),
            screen_width,
            screen_height,
            egui_raylib: EguiRaylib::new(),
        }
    }

    fn mandelbrot(screen_width: i32, screen_height: i32) -> Vec<Vec<color::Color>> {
        let mut data =
            vec![vec![color::Color::BLANK; screen_width as usize]; screen_height as usize];
        // --- Main loop to generate pixels ---
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                // Map the pixel to a complex number C
                let cx = RE_START + (x as f64 / SCREEN_WIDTH as f64) * (RE_END - RE_START);
                let cy = IM_START + (y as f64 / SCREEN_HEIGHT as f64) * (IM_END - IM_START);
                let c = Complex::new(cx, cy);

                // Start the iteration
                let mut z = Complex::new(0.0, 0.0);
                let mut n = 0;

                // The escape-time algorithm loop
                while z.norm_sq() <= 4.0 && n < MAX_ITER {
                    // The Mandelbrot formula: Z_n+1 = Z_n^2 + C
                    z = z * z + c;
                    n += 1;
                }

                // Color the pixel based on the result
                let (r, g, b) = if n == MAX_ITER {
                    // Inside the set: black
                    (0, 0, 0)
                } else {
                    // Outside the set: colorful
                    let i = n as f64;
                    let r = (0.5 * (1.0 + (i * 0.1).sin()) * 255.0) as u8;
                    let g = (0.5 * (1.0 + (i * 0.15).sin()) * 255.0) as u8;
                    let b = (0.5 * (1.0 + (i * 0.2).sin()) * 255.0) as u8;
                    (r, g, b)
                };

                // Write the RGB pixel data to the file
                // write!(writer, "{} {} {} ", r, g, b)?;
                data[y as usize][x as usize] = color::Color { r, g, b, a: 255 };
            }
        }
        data
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

#[derive(Copy, Clone)]
struct Complex {
    re: f64, // Real part
    im: f64, // Imaginary part
}

impl Complex {
    // Constructor
    fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    // Calculate the squared magnitude (re^2 + im^2).
    // This is much faster than `sqrt(re^2 + im^2)` for the escape check.
    fn norm_sq(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

fn update(state: &mut GameState) {
    raylib::BeginDrawing();
    raylib::ClearBackground(Color::BLANK);

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            raylib::DrawPixel(x, y, state.data[y as usize][x as usize]);
        }
    }

    raylib::EndDrawing();
}
