+++
title = 'Ahoi WebGPU!'
description = 'A dynamic WebGPU playground with a live shader editor.'
date = 2025-08-24T13:35:00+02:00
draft = false
tags = ['WebGPU']
script = "ahoi_wgpu/js/main.ts"
links = [ "webgpu-fundamentals", "wgpu", "learn-wgpu" ]
+++

This project is a "Hello World" for modern graphics programming, demonstrating how to create a raw WebGPU rendering context using TypeScript. Unlike standard pre-compiled demos, this page integrates the **Monaco Editor**, allowing you to modify the shader code in real-time and see the results instantly on the GPU.

## The Geometry

To execute a shader across the entire screen, we must first provide the GPU with geometry to draw. 
The most efficient method to cover a rectangular viewport is to render a single large triangle (or two triangles) that covers the clip space.

In this implementation, the vertex shader generates the geometry procedurally using the `vertex_index`, so no vertex buffers are required for the geometry itself.

{{< figure src="triangles.svg" >}}

## Live Shader Editor

Below is the interactive canvas. You can edit the code in the editor beneath it to change how the pixels are colored.

### Available Uniforms

To make the animation dynamic, the TypeScript host application sends data to the GPU via a Uniform Buffer every frame. You can use these variables in your shader code:

1.  **`u.resolution`** (`vec2<f32>`): The width and height of the canvas in pixels.
2.  **`u.time`** (`f32`): The elapsed time in seconds since the loop started.
3.  **`u.deltaTime`** (`f32`): The time in seconds it took to render the last frame.

The structure available in the shader looks like this:

```wgsl
struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
  deltaTime: f32,
};
@group(0) @binding(0) var<uniform> u: Uniforms;
```

### The Fragment Shader

The fragment shader is the heart of the visual effect. It runs in parallel for every pixel on the canvas. Its job is to return a `vec4<f32>` representing the **Red**, **Green**, **Blue**, and **Alpha** channels.

## Result

<div class="gpu_canvas-wrapper" id="gpu_wrapper">
  <canvas id="gpu_shader" width="1280" height="860"></canvas>
  <div class="gpu_canvas-controls">
    <button id="gpu_btn-play-pause" title="Play/Pause">⏸</button>
    <span id="gpu_fps-counter">00 FPS</span>
    <button id="gpu_btn-reset" title="Reset Code">↺</button>
    <button id="gpu_btn-fullscreen" title="Toggle Fullscreen">⛶</button>
  </div>
</div>
<div id="monaco-container"></div>

