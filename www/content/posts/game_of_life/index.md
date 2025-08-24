+++
title = "Conway's Game of Life"
description = "Dive into the fascinating world of cellular automata with Conway's Game of Life! This classic simulation showcases emergent behavior from simple rules, making it a must-know for any aspiring developer. On this page, you'll find a comprehensive guide to programming the Game of Life, complete with pseudocode, algorithm explanations, and optimizations. Whether you're a seasoned coder or just starting out, you'll learn how to implement this iconic simulation in your favorite programming language. Get ready to bring digital life to your screen!"
date = 2025-08-16T19:35:00+02:00
draft = true
tags = ['ca']
+++

Conway's Game of Life or simply Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970.[1] It is a zero-player game,[2][3] meaning that its evolution is determined by its initial state, requiring no further input. One interacts with the Game of Life by creating an initial configuration and observing how it evolves. It is Turing complete and can simulate a universal constructor or any other Turing machine.

## Moore neighborhood


there are different neighbourhoods for a celualar automata. the game of life uses the moore 
neighborhood. In the moore neighborhood we are using the 8 neighbor cells and count the active
ones.

Figure here

## Conways game of life

there are different rules for a life celualar automata. for conways game of life it is:

1) Any live cell with fewer than two live neighbours dies, as if by underpopulation.
1) Any live cell with two or three live neighbours lives on to the next generation.
1) Any live cell with more than three live neighbours dies, as if by overpopulation.
1) Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

these rules can also be written as B2S23  ("B" for birth, "S" for survival). 

**B3**   Any Dead Cell with exactly three neighbours become a live cell  
**S23**  Any live cell with two or three neighbours survives
         Any Other cell will result in a dead cell.


https://conwaylife.com/wiki/List_of_Generations_rules


<figure>
  <canvas id=canvas oncontextmenu=event.preventdefault()></canvas>

  <label for="targetInput">B:</label>
  <input type="text" id="birth" value="2">
  <label for="targetInput">S:</label>
  <input type="text" id="survive" value="34">
  <br>
  <label class="h2" form="rules">size:</label>
  <input type="range" min="1" max="10" value="4" id="size"/>
</figure>

There are some 

- HighLife (B36/S23): Creates interesting "replicator" patterns.
- Day & Night (B3678/S34678): Often results in symmetrical, crystal-like structures.
- Seeds (B2/S): Every live cell dies each generation, and new ones are born from any empty cell with exactly 2 neighbors. (An empty 'S' means nothing survives).



## Links:
- {{< link "gameoflife" >}}.
- {{< link "moore-neighborhood" >}}.
- {{< link "conwaylife" >}}.
- {{< github "game_of_life" >}}.

<script>
    let set_size;
    let set_birth;
    let set_survive;

    function on_load() {
        const dpr = window.devicePixelRatio;
        let canvas = document.getElementById('canvas');

        set_size = Module.cwrap(
            "set_size",
            null,
            ["number"]
        );
        set_birth = Module.cwrap(
            "set_birth",
            null,
            ["string"]
        );
        set_survive = Module.cwrap(
            "set_survive",
            null,
            ["string"]
        );

    }
    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
        // doNotCaptureKeyboard: true,
    };


    document.getElementById('birth').addEventListener('keydown', function(event) {
      console.log("set new birth: ", event.target.value);
      if (event.key === 'Enter') {
        const inputValue = event.target.value;
        set_birth(inputValue);
      }
    });
    document.getElementById('survive').addEventListener('keydown', function(event) {
        const inputValue = event.target.value;
        set_survive(inputValue);
    });
    const sizeInput = document.querySelector('#size');
    function handleSizeChange(event) {
        const newValue = event.target.value;
        set_size(newValue);
    }
    sizeInput.addEventListener('input', handleSizeChange);

    window.addEventListener('keydown', function(event) {
      if (event.keyCode === 8 || event.keyCode === 9) {
          // event.stopPropagation(); 
        event.stopImmediatePropagation(); 
      }
    }, true);
    window.addEventListener('keyup', function(event) {
        event.stopImmediatePropagation(); 
    }, true);

</script>
{{< wasm path="game_of_life.js" >}}

