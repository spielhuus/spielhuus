+++
title = 'Voronoi'
description = 'Drawing Voronoi diagrams'
date = 2025-09-28T11:35:00+02:00
draft = true
tags = ['ca']
script = "voronoi/js/main.ts"
links = ["voronoi", "jfa"]
+++

## The flooding

Drawing Voronoi diagrams is very simple. You take some random points on a canvas and 
assign a color to this point. Then you iterate each pixel and search for the point
that is the closest. Then you take the color of the point for this pixel. This approach 
is simple but very slow. For each pixel it iterates all points. Another approach 
could be that we start at the points and select the neighbours when they are not 
already visited. This way the diagram would grow from the points. 

<figure>
  <canvas id="raw-canvas"></canvas>
</figure>

## Jump Flood Algorithm (JFA)

This is also very slow, an improvemnt of this algorithm is the Jump Flood Algorithm (JFA) 
algorithm. instead of growing pixel by pixel we start with a very large step and decrease
the step size in every loop until the step size is 1.

The steps are:
- create two arrays that hold the distance to the nearest seed
- calculate the initial distnace, width or height of the screen divided by two
- start the flooding loop
  - for every pixel, get the eight pixels that are within the distance
  - if the pixel is a seed or an already visited pixel calculate the distance
  - when the distance is smaller then old distance of the pixel store it to the pixel
  - divide the distance by two and go to the next pixel

<figure>
  <canvas id="jfa-canvas"></canvas>
</figure>

## JFA on the GPU

The JFA can run on the GPU to parallelize the computation.

<figure>
  <canvas id="jfa-gpu"></canvas>
  <div class="controls">
    <button id="btn-cells">Cells</button>
    <button id="btn-grayscale">Grayscale Distance</button>
    <button id="btn-heatmap">Heatmap</button>
    <button id="btn-contours">Contours</button>
  </div>
</figure>




