use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use std::ffi::{c_int, c_void};

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
            game_state.counter += 1;
            update_counter(game_state.counter);
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn update_counter(index: i32);
}

struct GameState {}

impl GameState {
    fn new() -> Self {
        Self {}
    }
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = RefCell::new(None);
}

pub fn main() {
    let game_state = GameState::new();
    GAME_STATE.with(|cell| {
        *cell.borrow_mut() = Some(game_state);
    });

    #[cfg(target_arch = "wasm32")]
    unsafe {
        emscripten_set_main_loop_arg(main_loop_wrapper, std::ptr::null_mut(), 0, 1);
    }
}
