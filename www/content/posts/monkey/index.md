+++
title = 'Infinite monkey'
description = 'The **Infinite Monkey Theorem** states that a monkey typing randomly on a typewriter will eventually type the complete works of Shakespeare. This is a concept in probability theory. However, it would take an infinite amount of time, and the monkey would not live long enough to achieve this goal. Using a generative algorithm, we could significantly speed up the process.'
date = 2023-08-07T09:00:00-07:00
draft = false
tags = ['GA']
+++

<div id="result"> </div>
<div>MaxFitness: <span id="maxFitness"></span></div>
<div>Round: <span id="round"></span></div>
<div id="log"> </div>
<div id="log"> </div>

<label for="targetInput">Text to write:</label>
<input type="text" id="target" value="" onsubmit="start()">
<input type="submit" id="targetSubmit" value="Submit" onsubmit="start()">


# links

- {{< link "infinite_monkey" >}}.

<script>
    let set_target;
    function on_load() {
        const dpr = window.devicePixelRatio;
        let canvas = document.getElementById('canvas');

        set_target = Module.cwrap(
            "set_target",
            null,
            ["string"]
        );
    }


    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
    };

    function start() {
        let target = document.getElementById('target');
        console.log("New Target: '" + target.value +  "'")
    }

    document.getElementById('target').addEventListener('keydown', function(event) {
      if (event.key === 'Enter') {
        const inputValue = event.target.value;
        set_target(inputValue);
      }
    });
    document.getElementById('targetSubmit').addEventListener('click', function(event) {
        const inputValue =document.getElementById('target'); 
        set_target(inputValue);
    });
</script>
<script src="/monkey.js"></script>
