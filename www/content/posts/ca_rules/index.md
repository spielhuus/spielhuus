+++
title = 'Cellular automata'
description = 'Draw a 1-dimensional cellular automaton for all rules.'
date = 2025-08-14T19:35:00+02:00
draft = false
tags = ['ca']
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

<figure>
<canvas id=canvas oncontextmenu=event.preventdefault()></canvas>
<form action="#" id="rules">
    <label class="h2" form="rules">Rule:</label>
    <input type="number" value="90" name="rule" id="rule" min="0" max="255" required>
    <br>
    <label class="h2" form="rules">Size:</label>
    <input type="range" min="1" max="10" value="4" id="size"/>
    <br>
    <label class="h2" form="rules">Initial State:</label>
    <select id="initial">
      <option value="1">Middle</option>
      <option value="2">Left</option>
      <option value="3">Right</option>
      <option value="4">Random</option>
    </select>
</select>
</form>
</figure>


Another crucial aspect is defining the initial state of the cells. Commonly,
the central cell of the row is selected as the initial active cell. Alternative
possibilities include initiating the activity at the left or right side of the
row, choosing random initial active cells, or selecting multiple active cells 
as per the specific requirements or conditions of the simulation.

## Links:
- {{< link "wolfram" >}}.

<script>
    let set_rule;
    let set_size;
    let set_initial;

    function on_load() {
        const dpr = window.devicePixelRatio;
        let canvas = document.getElementById('canvas');

        set_rule = Module.cwrap(
            "set_rule",
            null,
            ["number"]
        );
        set_size = Module.cwrap(
            "set_size",
            null,
            ["number"]
        );
        set_initial = Module.cwrap(
            "set_initial",
            null,
            ["number"]
        );
    }
    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
    };

    const ruleInput = document.querySelector('#rule');
    function handleRuleChange(event) {
        const newValue = event.target.value;
        set_rule(newValue);
    }
    ruleInput.addEventListener('input', handleRuleChange);

    const sizeInput = document.querySelector('#size');
    function handleSizeChange(event) {
        const newValue = event.target.value;
        set_size(newValue);
    }
    sizeInput.addEventListener('input', handleSizeChange);

    const initialInput = document.querySelector('#initial');
    function handleInitChange(event) {
        const newValue = event.target.value;
        set_initial(newValue);
    }
    initialInput.addEventListener('input', handleInitChange);
</script>
{{< wasm path="ca.js" >}}
