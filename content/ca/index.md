+++
title = 'Cellular automata'
description = 'Draw a 1-dimensional cellular automaton for all rules.'
date = 2025-09-05T19:35:00+02:00
draft = false
tags = ['ca']
script = "ca/js/main.ts"
links = [ 'wolfram' ]
+++

Cellular automata consist of a grid of cells, each having a finite set of
states. For each cell, a set of neighbors is defined based on which a rule set
determines the new state of the cell. This simple concept can simulate patterns
found in nature.

This document demonstrates the creation of the most basic form of cellular
automata, with only one dimension. The neighbors for each cell are defined as
the cell immediately to the left and the right.

{{< figure src="cells.svg" caption="the initial state of the cells" >}}

To apply a rule, the first three cells are selected: `010`

{{< figure src="cells_1.svg" caption="the first three cells" >}}

Converting `010` from binary to decimal results in `2`. This number is used to
index the rule. If the bit at this index is set, the cell's new state is
active; otherwise, it remains inactive. The rule index ranges from `0` to `7`,
meaning the rule must be a number from `0` to `255`.

{{< figure src="rule_90.svg" caption="Rule 90: Neighborhood to State" >}}

For example, applying Rule 90, which is represented as a bit array, results in
an 'off' state for `010`.

Subsequently, the 'window' of cells is shifted by one position, and the process
is repeated.

{{< figure src="cells_2.svg" caption="the next three cells" >}}

Addressing edge cases, specifically the first and last cell of the row, is
essential, as they only have a single neighbor. A common method to resolve this
issue is to wrap around the cells such that the neighbor for the first cell is
the last one, and the neighbor for the last cell is the first one in the grid.

<figure class="fullwidth">
    <canvas id="shader" class='fullwidth'></canvas>
</figure>

<div><form action="#" id="rules">
    <label class="h2" form="rules">Rule:</label>
    <input type="number" value="90" name="rule" id="rule" min="0" max="255" required>
    <br>
    <label class="h2" form="rules">size:</label>
    <input type="range" min="1" max="10" value="4" id="size"/>
    <br>
    <label class="h2" form="rules">Initial State:</label>
    <select id="initial">
      <option value="1">Middle</option>
      <option value="2">Left</option>
      <option value="3">Right</option>
      <option value="4">Random</option>
    </select>
</form></div>

Another crucial aspect is defining the initial state of the cells. Commonly,
the central cell of the row is selected as the initial active cell. Alternative
possibilities include initiating the activity at the left or right side of the
row, choosing random initial active cells, or selecting multiple active cells 
as per the specific requirements or conditions of the simulation.

## Implementation Overview

This Rust implementation leverages `wgpu` for GPU-accelerated computation and rendering of the cellular automaton.
It uses a compute shader to calculate the next generation of cells based on the defined rule and a render pipeline
to visualize the grid on the screen. The core components include storage textures to hold the automaton's state,
uniform buffers to pass parameters like the rule number and grid dimensions to the shaders, and bind groups to
manage the resources bound to the compute and render pipelines.  The `State` struct encapsulates the `wgpu`
context and resources, while the `App` struct manages the application lifecycle and user interactions,
enabling dynamic modification of simulation parameters such as the rule, cell size, and initial state. 
