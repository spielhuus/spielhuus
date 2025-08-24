+++
title = 'WASM application with raylib'
description = 'Create a WebAssembly (WASM) application using Rust, Emscripten and Raylib.'
date = 2025-08-12T18:35:00+02:00
draft = false
tags = ['wasm']
+++

We aim to develop a graphical application using Raylib. Raylib, an easy-to-use
game engine, is optimal for visualizing our recreational programming exercises.
For Rust integration, custom bindings closely aligning with the Raylib `API`
will be utilized, incorporating support for `egui`.

**Previous Setup Examples**
- [Simple WASM application]({{< relref "wasm.md" >}})
- [Invoking JavaScript function from WASM]({{< relref "wasm_callback.md" >}})

## Configuring `Cargo.toml`

To integrate `raylib` with Rust, add the `raylib-egui-rs` crate to your
`Cargo.toml` file:


```toml
[dependencies]
raylib-egui-rs = { git = "https://github.com/spielhuus/raylib-egui-rs.git" }

```

## Initialize raylib

We utilize a `game_loop` and game state as per the previous examples. The main
function initializes Raylib and manages the drawing loop.

```rust
pub fn main() {
    // initialize raylib
    raylib::SetConfigFlags(raylib::ConfigFlags::FLAG_MSAA_4X_HINT as u32);
    raylib::InitWindow(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, TITLE);
    raylib::SetTargetFPS(60);

    // initialize the game state
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
```

## Create the game loop

The game loop, shared by native and WASM applications, enables drawing with
raylib functions.

```rust
fn update(state: &mut GameState) {
    raylib::BeginDrawing();
    raylib::ClearBackground(Color::BLACK);

    // code for painting

    raylib::EndDrawing();
}
```

# Configure build.rs

Ensure Emscripten links the code using `GLFW`.

```rust
use std::env;
use std::path::PathBuf;

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
        "EXPORTED_RUNTIME_METHODS=[\"ccall\", \"cwrap\", \"UTF8ToString\"]",
        "-s",
        "ABORTING_MALLOC=0",
        "-s",
        "WASM_BIGINT",
    ];

    for flag in other_flags {
        println!("cargo:rustc-link-arg={flag}");
    }
}
```

Build the project targeting WebAssembly (`wasm`).

```bash
source emsdk/emsdk_env.sh; cargo build --target wasm32-unknown-emscripten --release 
```

## Create Your Web Page

Add this JavaScript code to your web page to include the WebAssembly module and
start the game loop:

```html
<figure>
  <canvas id=canvas oncontextmenu=event.preventdefault()></canvas>
</figure>

<script>
    function on_load() {
    }
    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
    };

</script>
{{< wasm path="js/wasm_raylib/wasm_raylib.js" >}}

```

## Result

<figure>
  <canvas id=canvas oncontextmenu=event.preventdefault()></canvas>
</figure>

<script>
    function on_load() {
    }
    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
    };

</script>

## links

- {{< link "emscripten" >}}
- {{< link "raylib" >}}
- {{< link "raylib-egui-rs" >}}
- {{< github "wasm_raylib" >}}
