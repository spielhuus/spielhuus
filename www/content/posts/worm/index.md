+++
title = 'worm'
description = ''
date = 2025-08-12T18:34:00+02:00
draft = true
tags = ['wasm']
+++

We want to have a worm that moves over the screen and has a flexible motion

## define the path


we want to let the worm follow an eight wich is rotated by 90 degrees.
we let the eight be two circles.





///
First, we need to define the JavaScript callback function that Rust will
invoke. The linker needs to be aware of this function. Add the definition to
your `main.rs` file.

```rust
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn update_counter(index: i32);
}
```

As you can see, this is a global function. We need to create and update some
global state, which can be cumbersome in Rust. However, we can handle it as
follows:

```rust
struct GameState {
    counter: i32,
}

impl GameState {
    fn new() -> Self {
        Self {
            counter: 1,
        }
    }
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = RefCell::new(None);
}
```

In the main function, we can create the game state:

```rust
let game_state = GameState::new();
GAME_STATE.with(|cell| {
    *cell.borrow_mut() = Some(game_state);
});
```

This sets up the initial game state, which we can then update and manage
throughout the application.

```rust
GAME_STATE.with(|cell| {
    if let Some(state) = cell.borrow().as_ref() {
        // change the game state
    }
})
```

## The main loop

It's not feasible to have a loop in the main function and call the update
function directly. This approach won't update the HTML page until the main loop
completes. Therefore, we need a way to update the HTML in a separate "thread."
Emscripten provides the `emscripten_set_main_loop` function for this purpose.
This method is called regularly, allowing us to update the counter. First, we
must define the methods for the linker.

```rust
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
            // update the game state
        }
    });
}
```

In the main function, we can register the callback and then exit, allowing the
main loop to take over:


```rust
#[cfg(target_arch = "wasm32")]
unsafe {
    emscripten_set_main_loop_arg(main_loop_wrapper, std::ptr::null_mut(), 0, 1);
}
```

## Make the linker happy

Of course, you can't simply define the function in your page and expect it to be
called. The linker must be aware of it. Emscripten has a unique way of handling
this: you write a JavaScript file and let Emscripten link against it. Well,
if that's the way it has to be.

The Emscripten JavaScript file looks like this:

```js
mergeInto(LibraryManager.library, {
  update_counter: function (counter) {
    console.log(`JavaScript: 'update_counter' called with: ${counter}`);
  },
});
```

## putting it all together

Configure the Emscripten toolchain in the `build.rs`

```rust
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
        "ASYNCIFY",
        "-s",
        "EXPORTED_RUNTIME_METHODS=ccall,cwrap",
        "-s",
        "ABORTING_MALLOC=0",
        "-s",
        "WASM_BIGINT",
        "-s",
        "EXPORTED_FUNCTIONS=['_main']",
    ];

    for flag in other_flags {
        println!("cargo:rustc-link-arg={flag}");
    }
}
```

## Result

<script> 
    var Module = { }; 
</script> 
<script src="/wasm_callback.js"></script> 

<figure> 
<h1>WASM Example</h1> 
<p> 
 <p>Counter <span id="counter">0</span></p> 
</p> 
</figure> 

## links

- {{< link "emscripten" >}}.
