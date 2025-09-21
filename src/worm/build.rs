use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch != "wasm32" {
        return;
    }

    let other_flags = vec![
        "-s",
        "USE_GLFW=3",
        "-s",
        "ASYNCIFY",
        "-s",
        "GL_ENABLE_GET_PROC_ADDRESS=1",
        "-s",
        "ABORTING_MALLOC=0",
        "-s",
        "WASM_BIGINT",
    ];

    for flag in other_flags {
        println!("cargo:rustc-link-arg={flag}");
    }
}
