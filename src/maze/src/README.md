# Maze generation and solving

## Maze generation

[detailed information about the algorithms from jamisbuck.](https://weblog.jamisbuck.org/2011/2/7/maze-generation-algorithm-recap.html)

**Recursive Backtracker:**  Recursively carves paths to unvisited neighbors,
backtracking when surrounded.

1. Start at random cell.
2. Recursively carve paths to unvisited neighbors.
3. Backtrack when surrounded by visited cells.

**Eller's Algorithm:**  Generates rows, merging cells horizontally and creating
vertical connections.

1. Initialize first row with unique sets.
2. Merge adjacent cells in each row (horizontally) randomly if in different
sets.
3. Carve at least one downward passage for each set, creating new sets for
unconnected cells.
4. Repeat 2-3 until last row, force-merging all adjacent cells in the final
row.

**Kruskal's Algorithm:** Adds edges of lowest weight connecting disjoint trees
until all vertices are connected.

1.  Add all edges to a set.
2.  Add edges of lowest weight connecting disjoint sets.

**Prim's Algorithm:** Grows a minimum spanning tree by adding edges connecting
a vertex in the tree to one outside.

1. Start with a random vertex.
2. Iteratively add the lowest-weight edge connecting a vertex in the tree to
one outside.

**Recursive Division:** Recursively bisects the maze with walls, adding
passages.

1. Start with empty field.
2. Recursively bisect with a wall (horizontal or vertical), adding a passage.

**Aldous-Broder algorithm:** Randomly walks until all vertices are visited,
adding edges to the spanning tree.

1. Random walk from a vertex, adding edges to unvisited neighbors.
2. Repeat until all vertices are visited.

**Wilson's algorithm:** Performs random walks from unvisited vertices until
they hit the spanning tree.

1. Start with a random vertex in the spanning tree (UST).
2. Random walk from an unvisited vertex until hitting the UST; add the walk to
the UST.
3. Repeat until all vertices are in UST.

**Hunt-and-Kill algorithm:** Random walk until stuck; then "hunt" for an
unvisited cell adjacent to a visited one.

1. Random walk, carving passages to unvisited neighbors until stuck.
2. "Hunt" for unvisited cell adjacent to a visited one; carve passage and
continue from there.

**Growing Tree algorithm:**  Selects a cell, carves paths to unvisited
neighbors, and repeats.

1. Start with a random cell.
2. Carve paths to unvisited neighbors; remove cells with no unvisited
neighbors.

**Binary Tree algorithm:** For every cell, randomly carve a passage either north, or west.

**Sidewinder algorithm:** Processes rows, creating runs of cells and carving
north passages.

1. Process rows; add current cell to a "run".
2. Carve east or choose a cell from the "run" and carve north.
3. Repeat until all rows processed.

## Maze solving



**Genetic algorithm:**
https://cnrs.hal.science/hal-03844521v1/file/PaperGeneticAlgorithm.pdf
