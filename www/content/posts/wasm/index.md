+++
title = 'Simple WASM application'
description = 'Create a simple WebAssembly (WASM) application using Rust and Emscripten.'
date = 2025-08-10T18:34:00+02:00
draft = false
tags = ['wasm']
+++

In this article, we will guide you through creating WebAssembly (WASM)
applications using Rust and Emscripten.

## Installing Emscripten

First, you need to install Emscripten.
 
```bash 
git clone --depth 1 https://github.com/emscripten-core/emsdk.git 
cd emsdk
./emsdk install latest 
./emsdk activate latest 
```

## Rust to WebAssembly

First, create a new Rust project with a `main.rs` file. I assume you are
familiar with this process. For now, leave the `main` function empty. In the
context of Emscripten, the `main` function does not block page loading when
used for isolated function calls; it simply serves as the program's entry point
if executed as a standalone application.

```rust
pub fn main() {} 
```

Add a function to your `main.rs` that can be called from JavaScript. For
example:

```rust
#[cfg(target_family = "wasm")] 
#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 { 
    a + b 
} 
```

In this example, the `add` function takes two integers as arguments and returns
their sum. Note the use of `#[no_mangle]` and `extern "C"` to ensure the
function name is not mangled and is accessible from JavaScript.

We need to configure Emscripten to handle C types and other necessary settings.
To achieve this, we will set the `EMCC_CFLAGS` environment variable. To ensure
configurations can be made for each workspace, we will perform this setup in
the `build.rs` file.

```rust
fn main() { 
    if std::env::var("TARGET").unwrap() == "wasm32-unknown-emscripten" { 
        println!("cargo:rustc-link-arg=-sASYNCIFY"); 
        println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=ccall,cwrap"); 
    } 
} 
```

Next, we can build the WebAssembly (`wasm`) and JavaScript (`js`) files. Note
that without specifying the release option (e.g., `--release`), the JavaScript
file will not be generated.

```bash
source emsdk/emsdk_env.sh; cargo build --target wasm32-unknown-emscripten --release 
```

## Create Your Web Page

To include the WebAssembly module in your web page, you will need to add some
JavaScript code. For example:

```html
<script> 
    let fn_add;  
    function on_load() { 
        fn_add = Module.cwrap( 
            "add", 
            "number",
            ["number","number"] 
        ); 
    } 
 
    var Module = { 
        postRun: [ on_load ], 
    }; 
 
    function add_one() { 
            let text = document.getElementById("counter").innerText; 
            let val = fn_add(parseInt(text), 1);
            document.getElementById("counter").innerText = val; 
    } 
</script> 
<script src="/wasm.js"></script> 

<figure> 
<h1>WASM Example</h1> 
<p> 
 <p>Counter <span id="counter">0</span></p> 
</p> 
<p> 
  <button onClick="add_one()">ADD</button> 
</p> 
</figure> 
```

This example demonstrates how to fetch the WebAssembly module, instantiate it,
and call the `add` function exported from the Rust code.

## Result

<script> 
    let fn_add;  
    function on_load() { 
        fn_add = Module.cwrap( 
            "add", 
            "number",
            ["number","number"] 
        ); 
    } 
 
    var Module = { 
        postRun: [ on_load ], 
    }; 
 
    function add_one() { 
            let text = document.getElementById("counter").innerText; 
            let val = fn_add(parseInt(text), 1);
            document.getElementById("counter").innerText = val; 
    } 
</script> 
{{< wasm path="/js/wasm/wasm.js" >}}

<figure> 
<p> 
 <p>Counter <span id="counter">0</span></p> 
 <button onClick="add_one()">ADD</button> 
</p> 
</figure> 

## links

- {{< link "emscripten" >}}
- {{< github "wasm" >}}
