use std::env;
use std::path::PathBuf;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch != "wasm32" {
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_path = manifest_dir.join("src").join("library.js");

    println!("cargo:rustc-link-arg=--js-library");
    println!("cargo:rustc-link-arg={}", lib_path.to_str().unwrap());

    let other_flags = vec![
        "-s",
        "USE_GLFW=3",
        "-s",
        "ASYNCIFY",
        "-s",
        "GL_ENABLE_GET_PROC_ADDRESS=1",
        "-s",
        "EXPORTED_RUNTIME_METHODS=ccall,cwrap,UTF8ToString",
        // "EXPORTED_RUNTIME_METHODS=[\"ccall\", \"cwrap\", \"UTF8ToString\"]",
        "-s",
        "ABORTING_MALLOC=0",
        "-s",
        "WASM_BIGINT",
        "-s",
        "EXPORTED_FUNCTIONS=['_main', '_set_target']",
    ];

    for flag in other_flags {
        println!("cargo:rustc-link-arg={flag}");
    }
}
