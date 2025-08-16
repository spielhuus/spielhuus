fn main() {
    if std::env::var("TARGET").unwrap() == "wasm32-unknown-emscripten" {
        println!("cargo:rustc-link-arg=-sASYNCIFY");
        println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=ccall,cwrap");
    }
}
