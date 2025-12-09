+++
title = "Manddelbrot"
description = "Draw the mandelbrot set"
date = 2025-09-28T19:35:00+02:00
draft = true
tags = ['fractal']
script = "mandelbrot/js/main.ts"
#links = [ '' ]
+++


In this article i want to draw the famous mandelbrot set. possibly the nicest
visualisation of mathematics.

<math xmlns="http://www.w3.org/1998/MathML">
  <msub>
    <mi>z</mi>
    <mrow>
      <mi>n</mi>
      <mo>+</mo>
      <mn>1</mn>
    </mrow>
  </msub>
  <mo>=</mo>
  <msubsup>
    <mi>z</mi>
    <mi>n</mi>
    <mn>2</mn>
  </msubsup>
  <mo>+</mo>
  <mi>c</mi>
</math>


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

