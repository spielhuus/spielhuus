+++
title = 'Das kleine Haus des Nikolaus'
description = 'The small house of Nikolaus is a drawing puzzle for children. The house must be drawn in a single stroke using eight lines without tracing any line twice. To do this, one says, “Das klei-ne Haus des Ni-ko-laus.”'
date = 2025-08-13T18:35:00+02:00
draft = false
tags = ['graph']
+++

"das kleine Haus des Nikolaus" (The little house of St. Nicholas) is a popular
children's puzzle in which the objective is to draw a single, continuous line
that traces eight segments without lifting the pencil and without retracing any
segment. This is achieved by saying, "das-klei-ne-Haus-des-Ni-ko-laus"

This puzzle is an example of an Eulerian path. An Eulerian path is a trail in a
graph that visits every edge exactly once. In this case, there are two nodes
with an odd degree (nodes 1 and 5). Consequently, the house can only be
completely drawn in one line starting from either node 1 or node 5.

{{< figure src="nikolaus.svg" caption="Das kleine Haus des Nikolaus" >}}

Basically, the problem could be solved using backtracking. However, since we
want to find all solutions, we need to identify all possible solutions.

First, we create an array containing all possible moves from the junctions.

```rust
const MOVES: [[usize; 5]; 5] = [
    [0, 1, 0, 1, 1],
    [1, 0, 1, 1, 1],
    [0, 1, 0, 1, 0],
    [1, 1, 1, 0, 1],
    [1, 1, 0, 1, 0],
];
```

We start at node `1`, posible moves are `2` and `5`. We add them to the path. we
then iterate over the result array and add the next path if it is not already in the 
list. when it is we delete the entire solution from the result array.

first we create a result array with the possible moves from the start node.

```rust
pub fn nikolaus(start: usize) -> Result {
    // add the possible moves from the `start` node.
    let mut current_results: Result = MOVES[start]
        .iter()
        .enumerate()
        .filter_map(|(node, &possible_move)| {
            if possible_move == 1 {
                Some((vec![node], vec![(start, node)]))
            } else {
                None
            }
        })
        .collect();

    for _ in 0..7 {
        current_results = extend_paths(current_results);
    }

    current_results
}
```

we are collecting all the possible next moves and remove the item if it does not exist.

```rust
fn extend_paths(current_paths: Result) -> Result {
    current_paths
        .into_iter()
        .flat_map(|path_item| {
            let last_node = *path_item.0.last().expect("Path should not be empty.");
            MOVES[last_node]
                .iter()
                .enumerate()
                .filter_map(move |(next_node, &can_move)| {
                    if can_move == 1
                        && !path_item.1.contains(&(last_node, next_node))
                        && !path_item.1.contains(&(next_node, last_node))
                    {
                        let mut new_path_item = path_item.clone();
                        new_path_item.0.push(next_node);
                        new_path_item.1.push((last_node, next_node));
                        Some(new_path_item)
                    } else {
                        None
                    }
                })
        })
        .collect()
}
```

e voila, we have all the possible solutions. which should be 44 for node `1` and `5`.

<style>
  .flex-container {
    display: flex;
    padding: 10px;
  }
  .flex-container > div {
    padding: 20px;
    margin: 5px;
  }
</style>

<figure>
  <canvas id=canvas oncontextmenu=event.preventdefault()></canvas>
</figure>

<div class="flex-container" id="solutions"></div>

<script>
    let get_steps;

    function on_load() {
        const dpr = window.devicePixelRatio;
        let canvas = document.getElementById('canvas');

        get_steps = Module.cwrap(
            "get_steps",
            null,
            []
        );

        solutions();
    }
    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
    };

    function solutions() {
            const rustMessage = Module.UTF8ToString(get_steps());
            const jsArray = JSON.parse(rustMessage);
            let content = "<div>";
            for (var i = 0; i < jsArray.length; i++) {
                console.log(i + " " + Math.floor(jsArray.length / 3) + " == " + Math.floor(i % (jsArray.length / 3)));
                if (Math.floor(i % (Math.floor(jsArray.length / 3) + 1) == 0)) {
                    content += "</div><div>";
                }
                content += "<p><input type='radio' id='solution" + i + "' name='selected' value='" + i + "' " + (i == 0 ? "checked" : "") + " />";
                for (var j = 0; j<jsArray[i].length; j++) {
                  content += jsArray[i][j];
                }
                content += "</p>";
            }
            content += "</div>";
            document.getElementById("solutions").innerHTML = content;
    }
</script>
{{< script "nikolaus.js" >}}
