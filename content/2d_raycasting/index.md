+++
title = '2D-Raycasting'
description = ''
date = 2025-09-21T12:35:00+02:00
draft = true
tags = ['graph']
script = "2d_raycasting/js/main.ts"
links = [ 'line-line-intersection' ]
+++

## 2D Raycasting

We begin with raycasting doing some creative coding excercise. we create a room with random walls
and a point that moves around. From this point we draw rays to the next wall that the ray intercepts
with.


## Drawing walls

first we define a Line class and define some random walls.

```typescript
class Line {
  constructor(public start: Vector2, public end: Vector2, public width: number, public  color: string) {}
}

walls = [
  // the border around the canvas
  new Line(new Vector2(0, 0), new Vector2(canvas.width, 0), 2, "white"),
  new Line(new Vector2(canvas.width, 0), new Vector2(canvas.width, canvas.height), 1, "#00000000"),
  new Line(new Vector2(canvas.width, canvas.height), new Vector2(0, canvas.height), 1, "#00000000"),
  new Line(new Vector2(0, canvas.height), new Vector2(0, 0), 2, "orange"),
];

// some lines
for (let _i = 0; _i < WALLS_COUNT; _i++) {
  let x1 = Math.floor(Math.random() * canvas.width);
  let y1 = Math.floor(Math.random() * canvas.height);
  let x2 = Math.floor(Math.random() * canvas.width);
  let y2 = Math.floor(Math.random() * canvas.height);
  this.walls.push(new Line(new Vector2(x1, y1), new Vector2(x2, y2), 4, "white"));
}
```

the border of the canvas is also added as lines. later we want them to as intersection lines for the rays.


<figure class='fullwidth'>
    <canvas id="wall-canvas" class='fullwidth'></canvas>
</figure>


### Creating a random walker

Creating random walls is simple. we can use the javascript `Math.random` function and multiply the 
value with the width and height of the canvas. We also draw walls around the canvas so that the
light always hits a wall.

```typescript
class RayAnimator {
  walls: Line[];
  position: Vector2;

  constructor(private canvas: HTMLCanvasElement, private ctx: CanvasRenderingContext2D) {
      this.walls = [
        // the border around the canvas
        new Line(new Vector2(0, 0), new Vector2(canvas.width, 0), 2, "orange"),
        new Line(new Vector2(canvas.width, 0), new Vector2(canvas.width, canvas.height), 2, "orange"),
        new Line(new Vector2(canvas.width, canvas.height), new Vector2(0, canvas.height), 2, "orange"),
        new Line(new Vector2(0, canvas.height), new Vector2(0, 0), 2, "orange"),
      ];

      // some lines
      for (let _i = 0; _i<10; _i++) {
        let x1 = Math.floor(Math.random() * canvas.width);
        let y1 = Math.floor(Math.random() * canvas.height);
        let x2 = Math.floor(Math.random() * canvas.width);
        let y2 = Math.floor(Math.random() * canvas.height);
        this.walls.push(new Line(new Vector2(x1, y1), new Vector2(x2, y2), 2, "orange"));
      }


      // set the initial position of the light source
      this.position = new Vector2(canvas.width / 2, canvas.height / 2);
  }
}
```

<figure class='fullwidth'>
    <canvas id="mover-canvas" class='fullwidth'></canvas>
</figure>

## The final raycasting

finally draw the rays. therefore we create lines around the `position` and draw 
a line until we intersect a wall or border line.

first we have to calculate `t` and `u`:

<math display="block">
  <mi>t</mi>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>4</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
    </mrow>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>4</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
    </mrow>
  </mfrac>
</math>

<math display="block">
  <mi>u</mi>
  <mo>=</mo>
  <mo>-</mo>
  <mfrac>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>)</mo>
    </mrow>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>4</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
    </mrow>
  </mfrac>
</math>

we can check this conditions to check if a the lines have an interception point.

<math display="block">
  <mrow>
    <mn>0</mn>
    <mo>≤</mo>
    <mi>t</mi>
    <mo>≤</mo>
    <mn>1</mn>
  </mrow>
</math>

<math display="block">
  <mrow>
    <mn>0</mn>
    <mo>≤</mo>
    <mi>u</mi>
    <mo>≤</mo>
    <mn>1</mn>
  </mrow>
</math>

with the final formula we can get the interception point.


<math display="block">
  <msub>
    <mi>P</mi>
    <mi>x</mi>
  </msub>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub><msub><mi>y</mi><mn>2</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>1</mn></msub><msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub><msub><mi>y</mi><mn>4</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>3</mn></msub><msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
    </mrow>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>4</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
    </mrow>
  </mfrac>
</math>

<math display="block">
  <msub>
    <mi>P</mi>
    <mi>y</mi>
  </msub>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub><msub><mi>y</mi><mn>2</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>1</mn></msub><msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>4</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub><msub><mi>y</mi><mn>4</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>3</mn></msub><msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
    </mrow>
    <mrow>
      <mo>(</mo>
      <msub><mi>x</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>4</mn></msub>
      <mo>)</mo>
      <mo>-</mo>
      <mo>(</mo>
      <msub><mi>y</mi><mn>1</mn></msub>
      <mo>-</mo>
      <msub><mi>y</mi><mn>2</mn></msub>
      <mo>)</mo>
      <mo>(</mo>
      <msub><mi>x</mi><mn>3</mn></msub>
      <mo>-</mo>
      <msub><mi>x</mi><mn>4</mn></msub>
      <mo>)</mo>
    </mrow>
  </mfrac>
</math>

Here are the forumulas programmed in typescript:

```typescript
const x1 = wall.start.x;
const y1 = wall.start.y;
const x2 = wall.end.x;
const y2 = wall.end.y;
const x3 = this.position.x;
const y3 = this.position.y;
const x4 = this.position.x + dir.x;
const y4 = this.position.y + dir.y;

const den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
if (den == 0) {
  return;
}

const t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
const u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;
if (t > 0 && t < 1 && u > 0) {
  const pt = new Vector2(
    x1 + t * (x2 - x1),
    y1 + t * (y2 - y1)
  );
  return pt;
} else {
  return;
}
```

<figure class='fullwidth'>
  <canvas id="ray-canvas" class='fullwidth'></canvas>
</figure>

