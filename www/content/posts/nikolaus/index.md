+++
title = 'Das kleine Haus des Nikolaus'
description = 'The small house of Nikolaus is a drawing puzzle for children. The house must be drawn in a single stroke using eight lines without tracing any line twice. To do this, one says, “Das klei-ne Haus des Ni-ko-laus.”'
date = 2023-08-07T09:00:00-07:00
draft = false
tags = ['graph']
+++

Das kleine haus des nikolaus ist ein zeichen raetsel fuer kinder. Das haus muss in einem Linienzug aus acht strecken
gezeichnet werden ohne ohne eine Strecke zweimal zu durchlaufen. Dazu sagt man „Das klei-ne Haus des Ni-ko-laus.“

Dabei handelt es sich um einen Eulerweg. Es gibt zwei Knoten von ungeradem Grad (die Knoten 1 und 5). Komplett mit nur einer 
Linie kann das Haus nur vom Knoten 1 oder vom Knoten 5 aus gezeichnet werden.

{{< figure src="nikolaus.svg" caption="Das kleine Haus des Nikolaus" width=100 height=100 >}}

Basically the problem could be solved using backtraching. but we want to find all solutions so we have to find all possible solutions.


the posible moves from the nodes:

1)  [2, 5]
2)  [1, 3, 4, 5]
3)  [2, 4]
4)  [1, 2, 3, 6]
5)  [1, 4]

we start at 1

```
[
    [1]
]
```

next possible steps are 2 and 5

```
[
    [1, 2]
    [1, 5]
]
```


We start at the node 1. 
the possible roads are 2 and 5

from 2 we can go to 1, 3, 4 and 5. but we have already visited 1

<figure>
  <canvas id=canvas oncontextmenu=event.preventdefault()></canvas>
</figure>

<div>
    <table id="solutions"></table>
</div>

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
            let table = "";
            for (var i = 0; i < jsArray.length; i++) {
                    table += "<tr>";
                    let radio = "<td><input type='radio' id='solution" + i + "' name='selected' value='" + i + "' " + (i == 0 ? "checked" : "") + " /></td>";
                    console.log(radio);
                    table += radio;
                    for (var j = 0; j<jsArray[i].length; j++) {
                            table += "<td>" + jsArray[i][j] + "</td>";
                    
                    }
                    table += "</tr>";
            }
            document.getElementById("solutions").innerHTML = "<table border='1'>" + table + "</table>";
    }
</script>
<script src="{{ .Site.BaseURL }}/nikolaus.js"></script>
