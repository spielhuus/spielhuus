+++
title = 'Ahoi WebGPU!'
description = 'A dynamic, full-screen shader demonstration using Rust and WebGPU.'
date = 2025-08-24T13:35:00+02:00
draft = false
tags = ['Rust', 'WebGPU', 'wgpu', 'Wasm', 'Graphics']
+++

This project is a "Hello World" for modern graphics programming, demonstrating how to create a dynamic, 
psychedelic, full-screen animation using Rust, wgpu, and WebAssembly (Wasm). 

The wgpu crate provides a portable graphics API that targets WebGPU, Vulkan, Metal, and DirectX. 
This document outlines the core rendering logic, while the full initialization code can be found
in the linked source repository.

## create the triangles

To execute a shader across the entire screen, we must first provide the GPU with geometry to draw. 
The most efficient method to cover a rectangular viewport is to render two triangles that together
form a rectangle. This geometry is defined once in Normalized Device Coordinates (NDC), a
coordinate system where (-1, -1) is the bottom-left corner and (1, 1) is the top-right corner
of the screen.

{{< figure src="triangles.svg" >}}

The vertex shader's role is minimal: it simply passes these predefined vertex positions to the GPU's 
rasterizer. The rasterizer then determines which pixels on the screen are covered by these two 
triangles. This effectively turns the entire canvas into a drawing surface for the next 
stage: the fragment shader.

## pass the Uniforms to the shader.

To make the animation dynamic, we need to send data from our Rust application to the GPU shader on every
frame. This is done using a Uniform Buffer. A uniform is a piece of data that remains the same (is "uniform")
for every pixel being processed in a single frame. We send two key pieces of information: 

**time**: The elapsed time in seconds since the application started. This is the engine of our animation, 
allowing us to change patterns over time. 

**resolution**: The current width and height of the canvas. This is crucial for correcting
the aspect ratio, ensuring our animation isn't stretched on non-square screens.

Our Rust struct for this data looks like this, carefully padded to match the memory alignment rules of the GPU:

```rust
#[repr(C)]
struct Uniforms {
    resolution: [f32; 2],
    time: f32,
    _padding: u32,
}
```

## calculate the colors. 

The fragment shader is the heart of the project. It's a small program that runs in parallel
for every pixel on our canvas. Its only job is to return a color (`vec4<f32>`) for its specific
coordinate. 

Our shader performs the following steps for each pixel:

- Get the Position: It receives the pixel's interpolated coordinate on our quad (ranging from -1.0 to 1.0).
- Correct Aspect Ratio: It uses the resolution uniform to adjust the coordinates, preventing the image from looking stretched.
- Animate with Time: It uses the time uniform as an input to mathematical functions like sin() and cos(). This makes the patterns evolve and move with each frame.
- Create Patterns: It uses a rotate function and combines trigonometric functions to generate complex, repeating, and colorful patterns based on the pixel's final position and the current time.
- Return the Color: The final result of these calculations is used to construct the red, green, and blue channels of the output color.

Here is the core of the fragment shader:

```wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  // `in.model_pos` is the pixel's coordinate from [-1, 1]
  var p = in.model_pos * 0.5;

  // Correct for aspect ratio
  let aspect = uniforms.resolution.x / uniforms.resolution.y;
  p.x = p.x * aspect;

  // Use time for animation
  let t = uniforms.time;

  // Complex math to generate patterns
  let p0 = vec2<f32>(sin(p.x * 10.0), cos(p.y * 10.0));
  let p1 = rotate(p0, t * 0.5) * sin(t * 0.1);

  // Combine results into final RGB color
  let red   = sin(abs(p1.x * p1.y * p.y) * 50.0 + 2.0 + t * 2.0);
  let green = cos(p1.x * 10.0 + t) * sin(p1.y * 10.0 + t);
  let blue  = sin((p.x + p1.y) * 0.5 + t);

  return vec4<f32>(red, green, blue, 1.0);
}
```

## Result

<figure>
  <canvas width=1280 height=860 id="shader"></canvas>
  <p>press `F11` for fullscreen</p>
</figure>

## links

- {{< link "webgpu" >}}
- {{< link "webgpu-fundamentals" >}}
- {{< link "wgpu" >}}
- {{< link "learn-wgpu" >}}
- {{< github "ahoi_wgpu" >}}

{{< bindgen path="js/ahoi_wgpu/ahoi_wgpu.js" >}}
