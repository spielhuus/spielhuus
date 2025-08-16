#[cfg(target_family = "wasm")]
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn main() {}
