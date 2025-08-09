+++
title = 'wasm'
date = 2023-01-15T09:00:00-07:00
draft = false
tags = ['basics']
+++

In this article we want to create wasm apps with rust and emscripten. 


<script>
    let fn_add; 
    function on_load() {
        const dpr = window.devicePixelRatio;
        let canvas = document.getElementById('canvas');

        fn_add = Module.cwrap(
            "add",
            null,
            ["number","number"]
        );

        // let on_resize = Module.cwrap(
        //     "on_resize",
        //     null,
        //     ["number","number"]
        // );
        //
        // let resize_handler = () => {
        //     const width = this.canvas.width
        //         = window.innerWidth * dpr;
        //     const height = this.canvas.height
        //         = window.innerHeight * dpr;
        //     on_resize(width, height);
        // };
        //
        // window.addEventListener(
        //     "resize",
        //     resize_handler,
        //     true
        // );

        // resize_handler();
    }

    var Module = {
        postRun: [ on_load ],
        canvas: document.getElementById('canvas'),
    };

    function add_one() {
            let text = document.getElementById("counter").innerText;
            let val = fn_add(text, 1);
            document.getElementById("counter").innerText = val;
    }
</script>
<script src="/wasm.js"></script>
<h1>WASM Example</h1>
<p>
 <p>Counter <span id="counter">0</span></p>
</p>
<p>
    <button onClick="add_one()">ADD</button>
</p>
