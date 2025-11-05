+++
title = 'Mazes: Generation and Solutions'
description = 'Exploring graph algorithms to generate and solve perfect mazes, from simple crawlers to complex weavers.'
date = 2025-09-28T09:00:00-07:00
draft = false
tags = ['graph', 'algorithms', 'visualization']
script = "maze/js/main.ts"
links = [ 'bucklog', 'Disjoint_sets' ]
+++

My objective is to construct and navigate mazes using graph algorithms. Specifically, we'll focus on generating 'perfect mazes.' A perfect maze is a simple, elegant structure with a single, guaranteed solution path between any two points.

In graph theory terms, a perfect maze is a {{< wikipedia "Spanning tree" >}} of a grid. This means it connects every cell in the grid without creating any loops or cycles. Because there are no cycles, you can't walk in a circle and end up where you started without retracing your steps. This structure guarantees that there is one unique path from any cell to any other. Every cell is reachable, and there are no inaccessible areas.




<style>
#maze {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 1rem;
  align-items: center;
}
#maze .form-row {
  display: contents;
}
#maze label {
  text-align: right;
}
#maze .form-buttons {
  grid-column: 1 / -1;
  text-align: center;
  margin-top: 1rem;
}
</style>

<figure>
    <div class="two-cols">
      <canvas width="800" height="800" id="shader"></canvas>
       <div>
        <form action="#" id="maze">
            <div class="form-row">
                <label class="h2" for="size">size:</label>
                <input type="range" min="1" max="5" value="1" id="size"/>
            </div>
            <div class="form-row">
                <label class="h2" for="steps">steps per frame:</label>
                <input type="range" min="1" max="100" value="5" id="steps"/>
            </div>
            <div class="form-row">
                <label class="h2" for="generator">Generator:</label>
                <select id="generator">
                     <option value="1">Recursive Backtracker</option>
                     <option value="2">Kruskal</option>
                     <option value="3">Eller</option>
                     <option value="4">Prim</option>
                     <option value="5">Recursive Division</option>
                     <option value="6">Aldous & Broder</option>
                     <option value="7">Wilson</option>
                     <option value="8">Hunt and Kill</option>
                     <option value="9">Growing Tree</option>
                     <option value="10">Binary Tree</option>
                     <option value="11">Sidewinder</option>
                </select>
            </div>
            <div class="form-row">
                <label class="h2" for="solver">Solver:</label>
                <select id="solver">
                     <option value="1">Breadth-First Search</option>
                     <option value="2">Recursive Backtracker</option>
                     <option value="3">a*</option>
                     <option value="4">Dead-End Filling</option>
                     <option value="5">Wall Follower</option>
                     <option value="6">Genetic</option>
                </select>
            </div>
            <div class="form-buttons">
                <button id="generate" type="button">generate</button>
                <button id="solve" type="button">solve</button>
            </div>
        </form>
       </div>
      <!-- <figcaption>A maze generated and visualized on a canvas.</figcaption> -->
      <div id="maze-ui"></div>
    </div>
</figure>

## Generation Algorithms

Maze generation algorithms typically start with a grid of cells and carve passages by removing walls. Each algorithm has a distinct method, resulting in mazes with different textures and patterns.

### Recursive Backtracker (Depth-First Search)

This algorithm behaves like a person exploring a cave system, always going as deep as possible before backtracking. It creates long, winding corridors with few short dead ends.

**Method:**

1.  Start with a grid where all cells have walls between them.
2.  Choose a random starting `current_cell`, mark it as *visited*, and add it to a `stack`.
3.  While the `stack` is not empty:
    1.  Look at the `current_cell` on top of the `stack`.
    2.  If it has any unvisited neighbors:
        1.  Pick a random unvisited neighbor.
        2.  Remove the wall between the `current_cell` and the neighbor.
        3.  Mark the neighbor as *visited* and add it to the `stack`.
    3.  Else (if `current_cell` has no unvisited neighbors):
        1.  Remove the `current_cell` from the `stack` (this is the "backtracking" step).
4.  The maze is complete when the `stack` is empty.

### Kruskal’s Algorithm

This method thinks of the maze as a collection of walls. It randomly removes walls to connect different regions until everything is one single, connected maze.

**Method:**

1.  Create a list of all interior walls in the grid.
2.  Assign each cell to its own unique set or region.
3.  Shuffle the list of walls randomly.
4.  For each `wall` in the shuffled list:
    1.  Look at the two cells the `wall` separates.
    2.  If the cells are not in the same set:
        1.  Remove the `wall`.
        2.  Merge the sets of the two cells.
    3.  Otherwise, do nothing.
5.  The maze is complete when all cells are in a single set.

### Prim's Algorithm

Prim's algorithm starts from a single cell and grows the maze outward like a crystal, one cell at a time.

**Method:**

1.  Start with a grid full of walls.
2.  Choose a random starting cell, mark it as part of the maze.
3.  Add the walls of that cell to a `wall_list`.
4.  While the `wall_list` is not empty:
    1.  Pick a random `wall` from the `wall_list`.
    2.  Look at the two cells the `wall` divides. If only one of the cells is already in the maze:
        1.  Remove the `wall`, connecting the unvisited cell to the maze.
        2.  Mark the new cell as part of the maze.
        3.  Add the walls of the new cell to the `wall_list`.
    3.  Remove the chosen `wall` from the `wall_list`.

### Eller's Algorithm

Eller's algorithm builds the maze one row at a time, making decisions about horizontal and vertical connections as it goes. It uses disjoint sets to track which cells in the current row are connected.

**Method:**

1.  Initialize the first row, placing each cell in its own set.
2.  For each row (except the last):
    1.  **Horizontal Connections:** Go through the cells from left to right. For each cell, randomly decide whether to remove the wall between it and its right neighbor. *Rule: Never connect two cells that are already in the same set.* When a wall is removed, merge the sets of the two cells.
    2.  **Vertical Connections:** Go through the cells again. For each set of connected cells, you must make at least one vertical connection down to the next row. Randomly choose which cells in the set get a downward passage.
    3.  Move to the next row. Any cells that have a passage from the row above are now part of that set. Any new cells in this row start in their own set.
3.  For the **last row**:
    1.  Connect any adjacent cells that are not already in the same set. This ensures the entire maze is connected. Do not create any vertical connections.

### Recursive Division

This is the only algorithm on this list that adds walls instead of removing them. It starts with an empty space and recursively divides it with walls.

**Method:**

1.  Start with an empty grid with no interior walls.
2.  Draw a wall (either horizontal or vertical) that divides the current area.
3.  Punch one hole in that new wall to connect the two new sub-areas.
4.  Repeat this process for each new area until the areas are as small as possible (1 cell wide or high).

### Aldous-Broder Algorithm

This algorithm performs a "random walk" across the grid. It's simple but very inefficient, as it can take a long time to visit every cell. The resulting mazes have no bias and are uniform.

**Method:**

1.  Choose a random starting cell and mark it as *visited*.
2.  Choose a random neighbor and move to it.
3.  If the neighbor has not been visited before:
    1.  Remove the wall between the previous cell and the new one.
    2.  Mark the neighbor as *visited*.
4.  Repeat from step 2 until all cells in the grid have been visited.

### Wilson's Algorithm

Similar to Aldous-Broder, this method also uses a random walk but is much more efficient. It creates wonderfully unbiased mazes.

**Method:**

1.  Create a grid of cells.
2.  Choose a random cell and mark it as *visited* (part of the maze).
3.  Choose a new random, unvisited cell to start a "random walk".
4.  As you walk, keep track of the path. If your path ever crosses itself, erase the loop you just created and continue walking from the point of intersection.
5.  When the random walk finally bumps into any *visited* cell (part of the main maze):
    1.  Stop the walk.
    2.  Carve passages along the entire loop-erased path you just created.
    3.  Mark all cells on that path as *visited*.
6.  Repeat from step 3 until all cells are visited.

### Hunt-and-Kill Algorithm

This algorithm combines a simple random walk (the "Hunt") with a systematic scan (the "Kill") to ensure every cell is reached.

**Method:**

1.  **Hunt Mode:**
    1.  Choose a random starting cell.
    2.  Perform a random walk, carving passages to unvisited neighbors until you reach a cell with no unvisited neighbors (a dead end).
2.  **Kill Mode:**
    1.  Scan the entire grid, row by row, from the top-left.
    2.  Look for the first unvisited cell that is adjacent to a *visited* cell.
    3.  If you find one, remove the wall between them. This cell is your new starting point.
4.  Switch back to **Hunt Mode** (Step 1.2) from this new starting point.
5.  Repeat this Hunt-and-Kill cycle until the entire grid has been scanned and no more unvisited cells are found.

### Growing Tree Algorithm

This is a flexible algorithm that can mimic both the Recursive Backtracker and Prim's, depending on one simple rule.

**Method:**

1.  Create a list of active cells, and add a random starting cell to it.
2.  While the active list is not empty:
    1.  Choose a `cell` from the active list. (The selection method is key!)
    2.  Look for an unvisited neighbor of that `cell`.
    3.  If an unvisited neighbor is found:
        1.  Remove the wall between the `cell` and the neighbor.
        2.  Add the neighbor to the active list.
    4.  If no unvisited neighbors are found:
        1.  Remove the `cell` from the active list.

*   **Note:** If you always choose the newest cell from the list, this is identical to the Recursive Backtracker. If you choose a random cell, it behaves like Prim's.

### Binary Tree Algorithm

This is the simplest and fastest algorithm, but it creates heavily biased mazes with a diagonal pattern.

**Method:**

1.  For every cell in your grid:
2.  Randomly choose to carve a passage either **North** or **West**.
3.  That's it. For cells on the northern or western boundaries, you only have one choice.

### Sidewinder Algorithm

The Sidewinder algorithm is simple and fast, similar to the Binary Tree but works one row at a time.

**Method:**

1.  For each row in the grid:
    1.  Start a "run" of connected cells, beginning with the first cell.
    2.  For each cell in the row, randomly decide to either:
        1.  **Go East:** Carve a passage to the cell on the right, adding it to the current run.
        2.  **Go North:** Close the run. Pick a random cell from the current run and carve a passage North. Then, start a new run beginning with the next cell.

## Solver Algorithms

Once a maze is generated, we need a way to solve it. These algorithms find a path from a start point to an end point.

### Breadth-First Search (Dijkstra's Algorithm)

For an unweighted maze like ours, Dijkstra's algorithm simplifies to a Breadth-First Search (BFS). It explores the maze in layers, guaranteeing it will find the shortest possible path.

**Method:**

1.  Start at the `start_cell`. Mark its distance as 0 and add it to a queue.
2.  While the queue is not empty:
    1.  Take the next `cell` from the queue.
    2.  For each neighbor of that `cell`:
        1.  If the neighbor has not been visited:
            1.  Record its distance (distance of `cell` + 1).
            2.  Mark the `cell` as its parent (so we can trace the path back).
            3.  Add the neighbor to the queue.
3.  Once the `end_cell` is found, trace the path backward from the end to the start using the parent links.

### Recursive Backtracker (Depth-First Search)

This solver dives deep into one path, and if it hits a dead end, it backtracks to the last junction and tries a different way. It will find a solution, but not necessarily the shortest one.

**Method:**

1.  Start at the `start_cell` and mark it as visited.
2.  Follow any available path from the current cell.
3.  When you hit a junction, pick one unexplored path and follow it.
4.  If you hit a dead end, or a cell you've already visited, go back to the last junction that still has an unexplored path.
5.  Repeat until you find the `end_cell`.

### A* (A-Star)

A* is a smarter version of Dijkstra's. It uses a heuristic—an educated guess—to prioritize paths that seem to be heading in the right direction. For a grid maze, the heuristic is often the straight-line distance to the goal.

**Method:**

1.  It works like Dijkstra's, but when choosing which path to explore next, it doesn't just pick the one with the shortest path from the start.
2.  Instead, it picks the path that has the lowest value of: `(distance from start) + (estimated distance to goal)`.
3.  This makes it much faster because it avoids exploring paths that are obviously going the wrong way.

### Dead-End Filling

This is a simple and visual approach. It doesn't find a path from start to finish, but rather eliminates all the incorrect paths, leaving only the solution.

**Method:**

1.  Find every dead end in the maze (a cell with only one entrance).
2.  Fill the path from each dead end backwards until you reach a junction (a cell with three or more passages).
3.  This process may create new dead ends. Repeat until no more dead ends can be filled.
4.  The path (or paths) that remain are the solutions.

### Wall Follower

The classic, simple trick. It works for all perfect mazes but can fail in mazes with loops.

**Method:**

1.  Go to the `start_cell`.
2.  Place one hand on a wall (either the left or right).
3.  Start walking, keeping that hand on the wall at all times.
4.  Follow the wall as it turns corners. You will eventually be led to the exit.

### Genetic Algorithm

This is a more exotic, nature-inspired approach. It's not typically used for simple mazes but demonstrates a powerful optimization concept.

**Method:**

1.  **Population:** Generate a large number of random paths from start to end, even if they walk through walls.
2.  **Fitness:** Evaluate each path. A "fitter" path is one that crosses fewer walls and gets closer to the goal.
3.  **Selection & Breeding:**
    1.  Select the best paths (the "parents").
    2.  Create a new generation of paths by "breeding" them—for example, by combining the first half of one parent's path with the second half of another's.
    3.  Introduce random "mutations" (e.g., changing a random step in a path).
4.  **Repeat:** Repeat the process of evaluation and breeding for many generations. Eventually, a path will evolve that doesn't cross any walls and successfully reaches the goal.
