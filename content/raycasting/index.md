+++
title = 'Raycasting'
description = 'The small house of Nikolaus is a drawing puzzle for children. The house must be drawn in a single stroke using eight lines without tracing any line twice. To do this, one says, “Das klei-ne Haus des Ni-ko-laus.”'
date = 2025-09-27T18:35:00+02:00
draft = true
tags = ['graph']
script = "raycasting/js/main.ts"
links = [ 'Digital_differential_analyzer', 'lode_raycasting', 'playfuljs_raycasting' ]
+++

## Digital Differential Analyzer (DDA) algorithm

DDA (Digital Differential Analyzer) is a line drawing algorithm to generate a line segment between
two specified endpoints. It is a simple and efficient algorithm that works by using the incremental
difference between the x-coordinates and y-coordinates of the two endpoints to plot the line. The canvas below shows a simple implementation where a line is drawn from the center to your mouse, with dots indicating where the line crosses grid boundaries.

<ol>
        <li>Calculate the differences in x and y coordinates between the two endpoints: <br>
                <code>dx = x2 — x1</code><br>
                <code>dy = y2 — y1</code></li>
        <li>Determine the number of steps required to draw the line. The number of steps is equal to the
                larger of |dx| and |dy| since the algorithm needs to iterate over the greater axis.</li>
        <li>Calculate the increments along the x and y axes: x_increment = dx / steps y_increment = dy /
                steps</li>
        <li>Starting from the initial point (x1, y1), successively add the increments (x_increment,
                y_increment) to reach each subsequent point on the line. Round the coordinates to the
                nearest integer to ensure they align with the discrete pixels on the digital screen.
        </li>
        <li>Repeat Step 4 until you reach the final point (x2, y2).</li>
</ol>

<figure>
    <canvas id="grid-canvas"></canvas>
</figure>

## Raycasting with DDA

<canvas id="view-canvas"></canvas>
