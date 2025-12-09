+++
title = 'Palette'
description = 'Calculate color palettes'
date = 2025-12-07T16:35:00+02:00
draft = false
tags = ['graph']
script = "palette/js/main.ts"
github = 'palette'
links = ["palettes"]
+++

<style>
  #rules {
    display: grid;
    grid-template-columns: 1fr;
    gap: 25px;
    padding: 20px 0;
  }
  .param-group {
    display: grid;
    grid-template-columns: 150px repeat(3, 1fr);
    grid-template-rows: auto auto;
    gap: 10px 15px;
    align-items: center;
    grid-template-areas:
      "label input1 input2 input3"
      "label slide1 slide2 slide3";
  }
  .param-group .h2 { grid-area: label; }
  .param-group input[type="number"]:nth-of-type(1) { grid-area: input1; }
  .param-group input[type="number"]:nth-of-type(2) { grid-area: input2; }
  .param-group input[type="number"]:nth-of-type(3) { grid-area: input3; }
  .param-group input[type="range"]:nth-of-type(1) { grid-area: slide1; }
  .param-group input[type="range"]:nth-of-type(2) { grid-area: slide2; }
  .param-group input[type="range"]:nth-of-type(3) { grid-area: slide3; }
  .param-group input {
    width: 100%;
    box-sizing: border-box;
    margin: 0;
  }
  .param-group input[type="number"] {
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
    text-align: center;
  }
</style>

A utility application for procedurally creating and testing color palettes in a live shader environment.
Edit WGSL shader code and manipulate uniform parameters in real-time to instantly visualize complex
color gradients powered by WebGPU.

<figure class="fullwidth">
    <div class="gpu_canvas-wrapper" id="gpu_wrapper">
      <canvas id="gpu_shader" width="1280" height="200"></canvas>
      <div class="gpu_canvas-controls">
        <button id="gpu_btn-reset" title="Reset Code">↺</button>
      </div>
    </div>
</figure>
<div>
  <form action="#" id="rules">
    <div class="param-group">
      <label class="h2">Param 1:</label>
      <input type="number" value="0.5" id="p1_num1" min="0" max="1" step="0.01" required>
      <input type="number" value="0.5" id="p1_num2" min="0" max="1" step="0.01" required>
      <input type="number" value="0.5" id="p1_num3" min="0" max="1" step="0.01" required>
      <input type="range" min="0" max="1" step="0.01" value="0.5" id="p1_slider1"/>
      <input type="range" min="0" max="1" step="0.01" value="0.5" id="p1_slider2"/>
      <input type="range" min="0" max="1" step="0.01" value="0.5" id="p1_slider3"/>
    </div>
    <div class="param-group">
      <label class="h2">Param 2:</label>
      <input type="number" value="0.5" id="p2_num1" min="0" max="1" step="0.01" required>
      <input type="number" value="0.5" id="p2_num2" min="0" max="1" step="0.01" required>
      <input type="number" value="0.5" id="p2_num3" min="0" max="1" step="0.01" required>
      <input type="range" min="0" max="1" step="0.01" value="0.5" id="p2_slider1"/>
      <input type="range" min="0" max="1" step="0.01" value="0.5" id="p2_slider2"/>
      <input type="range" min="0" max="1" step="0.01" value="0.5" id="p2_slider3"/>
    </div>
    <div class="param-group">
      <label class="h2">Param 3:</label>
      <input type="number" value="1.0" id="p3_num1" min="0" max="2" step="0.01" required>
      <input type="number" value="1.0" id="p3_num2" min="0" max="2" step="0.01" required>
      <input type="number" value="1.0" id="p3_num3" min="0" max="2" step="0.01" required>
      <input type="range" min="0" max="2" step="0.01" value="1.0" id="p3_slider1"/>
      <input type="range" min="0" max="2" step="0.01" value="1.0" id="p3_slider2"/>
      <input type="range" min="0" max="2" step="0.01" value="1.0" id="p3_slider3"/>
    </div>
    <div class="param-group">
      <label class="h2">Param 4:</label>
      <input type="number" value="0.00" id="p4_num1" min="0" max="1" step="0.01" required>
      <input type="number" value="0.33" id="p4_num2" min="0" max="1" step="0.01" required>
      <input type="number" value="0.67" id="p4_num3" min="0" max="1" step="0.01" required>
      <input type="range" min="0" max="1" step="0.01" value="0.00" id="p4_slider1"/>
      <input type="range" min="0" max="1" step="0.01" value="0.33" id="p4_slider2"/>
      <input type="range" min="0" max="1" step="0.01" value="0.67" id="p4_slider3"/>
    </div>
  </form>
</div>

<div id="monaco-container"></div>

# Result;

<pre id="result-code">
const a = vec3<f32>(0.5, 0.5, 0.5);
const b = vec3<f32>(0.5, 0.5, 0.5);
const c = vec3<f32>(1.0, 1.0, 1.0);
const d = vec3<f32>(0.00, 0.33, 0.67);
</pre>

