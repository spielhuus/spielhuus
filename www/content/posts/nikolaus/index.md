+++
title = 'Das kleine Haus des Nikolaus'
description = 'The small house of Nikolaus is a drawing puzzle for children. The house must be drawn in a single stroke using eight lines without tracing any line twice. To do this, one says, “Das klei-ne Haus des Ni-ko-laus.”'
date = 2025-08-13T18:35:00+02:00
draft = false
tags = ['graph']
+++

## "Das Kleine Haus des Nikolaus" Puzzle

**"Das kleine Haus des Nikolaus"** is a popular children's puzzle where the
objective is to draw a continuous line that traces eight segments without
lifting the pencil and without retracing any segment. This can be accomplished
while verbalizing the phrase, "das-klei-ne-Haus-des-Ni-ko-laus."

This puzzle exemplifies an Eulerian path, which is a path in a graph that
visits every edge exactly once. In this case, the nodes `0` and `4` each have
an odd degree, implying that an Eulerian path, if it exists, must start at one
of these nodes and end at the other.

{{< figure src="nikolaus.svg" caption="Das kleine Haus des Nikolaus" >}}

To solve the puzzle algorithmically, we could use a backtracking approach.
However, since we aim to find all possible solutions, it is necessary to
identify all valid Eulerian paths.

The initial step involves creating an array that lists all possible moves from
the junctions.

```rust
const MOVES: [[usize; 5]; 5] = [
    [0, 1, 0, 1, 1],
    [1, 0, 1, 1, 1],
    [0, 1, 0, 1, 0],
    [1, 1, 1, 0, 1],
    [1, 1, 0, 1, 0],
];
```

We begin at node `0`. Possible moves are nodes `1`, `3`, and `4`. We add these
to our path array.

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

Next, we iterate over this path array, adding subsequent nodes only if they
haven't already been included in the path. If adding a node results in a path
that is already in our result list, we skip that solution.

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

## Result 

 E Voilà, we've collected all possible solutions, resulting in 

**44 distinct Eulerian paths**

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


## links

- {{< github "nikolaus" >}}

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
                if (i > 0 && Math.floor(i % (Math.floor(jsArray.length / 4) + 1) == 0)) {
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
{{< wasm path="js/nikolaus/nikolaus.js" >}}
