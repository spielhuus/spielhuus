fn main() {
    if std::env::var("TARGET").unwrap() == "wasm32-unknown-emscripten" {
        println!("cargo:rustc-link-arg=-sUSE_GLFW=3");
        println!("cargo:rustc-link-arg=-sASYNCIFY");
        println!("cargo:rustc-link-arg=-sGL_ENABLE_GET_PROC_ADDRESS=1");
        println!("cargo:rustc-link-arg=--bind");
        println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=ccall,cwrap,UTF8ToString");
    }
}
