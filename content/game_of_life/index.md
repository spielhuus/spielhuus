+++
title = "Conway's Game of Life"
description = "Dive into the fascinating world of cellular automata with Conway's Game of Life! This classic simulation showcases emergent behavior from simple rules, making it a must-know for any aspiring developer. On this page, you'll find a comprehensive guide to programming the Game of Life, complete with pseudocode, algorithm explanations, and optimizations. Whether you're a seasoned coder or just starting out, you'll learn how to implement this iconic simulation in your favorite programming language. Get ready to bring digital life to your screen!"
date = 2025-09-12T19:35:00+02:00
draft = false
tags = ['ca']
script = "game_of_life/js/main.ts"
links = [ 'gameoflife', 'moore-neighborhood', 'conwaylife' ]
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

<figure class="fullwidth">
  <canvas id="shader" class="fullwidth"></canvas>
</figure>

<label for="targetInput">B:</label>
<input type="text" id="birth" value="2">
<label for="targetInput">S:</label>
<input type="text" id="survive" value="34">
<br>
<label class="h2" form="rules">size:</label>
<input type="range" min="1" max="10" value="4" id="size"/>

There are some 

- HighLife (B36/S23): Creates interesting "replicator" patterns.
- Day & Night (B3678/S34678): Often results in symmetrical, crystal-like structures.
- Seeds (B2/S): Every live cell dies each generation, and new ones are born from any empty cell with exactly 2 neighbors. (An empty 'S' means nothing survives).
