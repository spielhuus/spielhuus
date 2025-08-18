+++
title = "Conway's Game of Life"
description = "Dive into the fascinating world of cellular automata with Conway's Game of Life! This classic simulation showcases emergent behavior from simple rules, making it a must-know for any aspiring developer. On this page, you'll find a comprehensive guide to programming the Game of Life, complete with pseudocode, algorithm explanations, and optimizations. Whether you're a seasoned coder or just starting out, you'll learn how to implement this iconic simulation in your favorite programming language. Get ready to bring digital life to your screen!"
date = 2025-08-16T19:35:00+02:00
draft = false 
tags = ['ca']
+++




<canvas id=canvas oncontextmenu=event.preventdefault()></canvas>

## Links:
- {{< link "wolfram" >}}.
- {{< github "game_of_life" >}}.

<script>
    let get_steps;

    function on_load() {
        const dpr = window.devicePixelRatio;
        let canvas = document.getElementById('canvas');

        // get_steps = Module.cwrap(
        //     "get_steps",
        //     null,
        //     []
        // );

        // solutions();
    }
    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
    };

    // function solutions() {
            // const rustMessage = Module.UTF8ToString(get_steps());
            // const jsArray = JSON.parse(rustMessage);
            // let table = "";
            // for (var i = 0; i < jsArray.length; i++) {
                    // table += "<tr>";
                    // let radio = "<td><input type='radio' id='solution" + i + "' name='selected' value='" + i + "' " + (i == 0 ? "checked" : "") + " /></td>";
                    // console.log(radio);
                    // table += radio;
                    // for (var j = 0; j<jsArray[i].length; j++) {
                            // table += "<td>" + jsArray[i][j] + "</td>";
                    //  
                    // } 
                    // table += "</tr>";
            // } 
            // document.getElementById("solutions").innerHTML = "<table border='1'>" + table + "</table>";
    // } 
</script>
{{< wasm path="game_of_life.js" >}}

