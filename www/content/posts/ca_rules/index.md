+++
title = 'Cellular automata'
description = 'Draw a 1-dimensional cellular automaton for all rules.'
date = 2023-01-15T09:00:00-07:00
draft = false
tags = ['ca']
+++

cellular automata rules

<canvas id=canvas oncontextmenu=event.preventdefault()></canvas>

## Links:
- {{< link "wolfram" >}}.


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
<script src="/ca.js"></script>
