+++
title = 'Das kleine Haus des Nikolaus'
description = 'The small house of Nikolaus is a drawing puzzle for children. The house must be drawn in a single stroke using eight lines without tracing any line twice. To do this, one says: “Das klei-ne Haus des Ni-ko-laus.”'
date = 2025-08-13T18:35:00+02:00
draft = false
tags = ['graph']
script = "nikolaus/js/main.ts"
github = 'nikolaus'
+++

<style>
  .flex-container {
    display: flex;
    padding: 10px;
  }
  .flex-container > div {
    padding: 20px;
    margin: 5px;
  }
  #solutions { display: grid; grid-template-columns: repeat(4, 1fr); }
</style>

## "Das Kleine Haus des Nikolaus" Puzzle

**"Das kleine Haus des Nikolaus"** is a popular children's puzzle where the
objective is to draw a house with a continuous line that traces eight segments without
lifting the pencil and without retracing any segment. This can be accomplished
while verbalizing the phrase, "das-klei-ne-Haus-des-Ni-ko-laus."

This puzzle exemplifies an Eulerian path, which is a path in a graph that
visits every edge exactly once. In this case, the nodes `0` and `4` each have
an odd degree, implying that an Eulerian path, if it exists, must start at one
of these nodes and end at the other.

<figure>
  <canvas id="static-canvas" width="200" height="200"></canvas>
</figure>

To solve the puzzle algorithmically, we could use a backtracking approach.
However, since we aim to find all possible solutions, it is necessary to
identify all valid Eulerian paths.

The initial step involves creating an array that lists all possible moves from
the junctions.

```typescript
const MOVES = [
  [0, 1, 0, 1, 1],
  [1, 0, 1, 1, 1],
  [0, 1, 0, 1, 0],
  [1, 1, 1, 0, 1],
  [1, 1, 0, 1, 0],
];
```

We begin at node `0`. Possible moves are nodes `1`, `3`, and `4`. We add these
to our path array.

```typescript
function nikolaus(start: number): Result {
  let result: Result = [];
  for (let i: number = 0; i < MOVES[start].length; i++) {
    if (MOVES[start][i] == 1) {
      let item: ResultItem = [
        [start, i],
        [[start, i], [i, start]]
      ];
      result.push(item);
    }
  }
  for (let i = 0; i < 7; i++) {
    result = extend_paths(result);
  }
  return result;
}
```

Next, we iterate over this path array, adding subsequent nodes only if they
haven't already been included in the path. If adding a node results in a path
that is already in our result list, we skip that solution.

```typescript
function extend_paths(current_paths: Result): Result {
  const new_paths: Result = [];

  for (const current_path_item of current_paths) {
    const path_nodes = current_path_item[0];
    const path_edges = current_path_item[1];
    const last_node = path_nodes[path_nodes.length - 1];

    for (let next_node = 0; next_node < MOVES[last_node].length; next_node++) {
      const can_move = MOVES[last_node][next_node] === 1;

      const edge_exists = path_edges.some(
        edge => (edge[0] === last_node && edge[1] === next_node) ||
          (edge[0] === next_node && edge[1] === last_node)
      );

      if (can_move && !edge_exists) {
        const new_path_nodes = [...path_nodes];
        const new_path_edges = [...path_edges];

        new_path_nodes.push(next_node);
        new_path_edges.push([last_node, next_node]);

        new_paths.push([new_path_nodes, new_path_edges]);
      }
    }
  }

  return new_paths;
}
```

## Result 

Voilà, we've collected all possible solutions, resulting in

**44 distinct Eulerian paths**

<figure>
  <canvas id="draw-canvas" width="200" height="200"></canvas>
</figure>

<div class="flex-container" id="solutions"></div>


