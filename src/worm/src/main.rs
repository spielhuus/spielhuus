// #[cfg(target_family = "wasm")]
// #[unsafe(no_mangle)]
// pub extern "C" fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

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

const SEGMENTS: usize = 14;
const SEGMENT_DISTANCE: f32 = 70.0;
const SEGMENT_SIZES: &[f32; SEGMENTS] = &[
    70.0, 40.0, 70.0, 50.0, 50.0, 30.0, 20.0, 20.0, 10.0, 8.0, 6.0, 4.0, 4.0, 2.0,
];

const TITLE: &str = "Das kleine Haus des Nikolaus";

use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::ffi::{CString, c_char, c_int, c_void};

use raylib_egui_rs::{
    color::{CSSPalette, Color},
    math::Vector2,
    raylib,
};

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

struct Chain {
    pos: Vector2,
}

struct GameState {
    path_angle: f32,
    act_circle: usize,
    step: f32,
    chain: Vec<Chain>,
    show_lines: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            path_angle: 0.0,
            act_circle: 0,
            step: -0.02,
            chain: vec![],
            show_lines: false,
        }
    }
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = RefCell::new(None);
    static JSON_BUFFER: RefCell<Option<String>> = RefCell::new(None);
}

pub fn main() {
    // initialize raylib
    raylib::SetConfigFlags(raylib::ConfigFlags::FLAG_MSAA_4X_HINT as u32);
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(60);

    // setup the chain
    let mut chain: Vec<Chain> = vec![];
    for i in 0..SEGMENTS {
        chain.push(Chain {
            pos: Vector2 {
                x: (raylib::GetScreenWidth() / 2 + i as i32 * SEGMENT_SIZES[i] as i32) as f32,
                y: (raylib::GetScreenHeight() / 2) as f32,
            },
        })
    }

    // initialize the state
    let mut game_state = GameState::new();
    game_state.chain = chain;
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

fn constrain_distance(point: Vector2, anchor: Vector2, distance: f32) -> Vector2 {
    raylib::Vector2Add(
        raylib::Vector2Scale(
            raylib::Vector2Normalize(raylib::Vector2Subtract(point, anchor)),
            distance,
        ),
        anchor,
    )
}

fn update(state: &mut GameState) {
    raylib::BeginDrawing();
    raylib::ClearBackground(Color::BLACK);

    state.show_lines = raylib::IsMouseButtonDown(0);

    state.chain[0].pos = raylib::GetMousePosition();
    for i in 0..state.chain.len() - 1 {
        state.chain[i + 1].pos =
            constrain_distance(state.chain[i + 1].pos, state.chain[i].pos, SEGMENT_DISTANCE);
    }

    let mut points = vec![];
    let mut points_b = vec![];
    let dx = raylib::Vector2Subtract(state.chain[1].pos, state.chain[0].pos);
    for target_angle in [0, 45, 135, 180] {
        let angle =
            dx.y.atan2(dx.x) + std::f32::consts::PI / 2.0 + (target_angle as f32).to_radians();
        let pos = Vector2 {
            x: state.chain[0].pos.x + angle.cos() * SEGMENT_SIZES[0],
            y: state.chain[0].pos.y + angle.sin() * SEGMENT_SIZES[0],
        };
        points.push((pos, state.chain[0].pos));
    }

    for (i, link) in state.chain.windows(2).enumerate() {
        println!("{:#?}", SEGMENT_SIZES[i]);
        let dx = raylib::Vector2Subtract(link[0].pos, link[1].pos);
        let angle = dx.y.atan2(dx.x) + std::f32::consts::PI / 2.0;
        let pos = Vector2 {
            x: link[1].pos.x + angle.cos() * SEGMENT_SIZES[i],
            y: link[1].pos.y + angle.sin() * SEGMENT_SIZES[i],
        };
        points.push((pos, state.chain[i].pos));

        let angle = dx.y.atan2(dx.x) - std::f32::consts::PI / 2.0;
        points_b.push((
            Vector2 {
                x: link[1].pos.x + angle.cos() * SEGMENT_SIZES[i],
                y: link[1].pos.y + angle.sin() * SEGMENT_SIZES[i],
            },
            state.chain[i].pos,
        ));
    }
    points.extend(points_b.into_iter().rev());

    for i in 0..points.len() {
        let p1 = points[i].0;
        let p2 = points[(i + 1) % points.len()].0;
        let p3 = points[(i + 2) % points.len()].0;
        let p4 = points[(i + 3) % points.len()].0;
        for s in 0..10 {
            let t = s as f32 / 10.0;
            let pt = raylib::GetSplinePointBasis(p1, p2, p3, p4, t);
            raylib::DrawCircleV(pt, 8.0, Color::MAGENTA);
        }
    }

    if state.show_lines {
        for link in &state.chain {
            raylib::DrawCircleV(link.pos, 6.0, Color::RED);
        }
        for (pt, _) in &points {
            raylib::DrawCircleV(*pt, 2.0, Color::RED);
        }
    }

    raylib::EndDrawing();
}
