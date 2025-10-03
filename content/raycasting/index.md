+++
title = 'Raycasting'
description = 'The small house of Nikolaus is a drawing puzzle for children. The house must be drawn in a single stroke using eight lines without tracing any line twice. To do this, one says, “Das klei-ne Haus des Ni-ko-laus.”'
date = 2025-09-27T18:35:00+02:00
draft = true
tags = ['graph']
script = "raycasting/js/main.ts"
links = [ 'Digital_differential_analyzer', 'lode_raycasting', 'playfuljs_raycasting' ]
+++


We are going to implement a retro first person shooter. The first shooters were built on the limited hardware
of the time. Back in the days, they used a simplified raycasting method that is very efficient.

## Digital Differential Analyzer (DDA) algorithm

In raycasting, we use an algorithm based on the DDA (Digital Differential Analyzer). While the 
original DDA was for drawing pixels on a screen, we adapt it to efficiently find which grid
cells a ray passes through. It works by incrementally stepping from one grid line to the next,
always choosing the closest one.

We have to create an array with the grid data. It is a two-dimensional grid with numbers, where
zero means the cell is empty and any positive number indicates some type of wall. A position on
the map is represented by two real (floating-point) numbers for X and Y. Each cell boundary is a
natural number (0, 1, 2...). This is important for simplifying the calculation process later.

We need a player position on the map and a direction where the player is looking. In this first
example, the player is somewhere in the middle of the map and is looking towards the mouse cursor.
We can calculate a vector in this direction.

{{< figure src="dda.svg" caption="the dda algorithm" >}}

To determine the ray's direction, we'll use the mouse cursor's position. Since the player's position
(this.player.pos) is in map coordinates (e.g., 5.5, 5.5) and the mouse position is in screen coordinates
(pixels), we first need to convert the player's position to screen coordinates by multiplying it by the
size of a grid cell (cellSize). Then, we can find the direction vector by subtracting the player's
screen position from the mouse's screen position. Finally, we normalize this vector to get a unit
vector, rayDir, which only represents direction.

```typescript
let mapPos = new Vector2(Math.floor(this.player.pos.x), Math.floor(this.player.pos.y));
let rayDir = mousePos.sub(this.player.pos.mul(cellSize));
rayDir = rayDir.norm();
```

Next, we calculate some essential values for the DDA loop.

`deltaDist`: This is not the distance between grid lines, but rather how far the ray must travel 
to cross one full grid unit in the X or Y direction. It's calculated as the absolute value 
of `1 / rayDir.x` and `1 / rayDir.y`. If a ray is mostly horizontal, its `deltaDist.x` will be
small, and its `deltaDist.y` will be large. We calculate this once and reuse it in every step of the loop.

`step`: This simply tells us if we should step in the positive or negative direction along 
an axis (+1 or -1). It depends on the direction of `rayDir`.

`sideDist`: This holds the total distance from the ray's starting position to the first grid
line it crosses, for both the `X` and `Y` axes. We calculate this initial distance 
based on the fractional part of the player's starting position.

```typescript
let deltaDist = new Vector2(
  (rayDir.x == 0) ? 1e30 : Math.abs(1 / rayDir.x),
  (rayDir.y == 0) ? 1e30 : Math.abs(1 / rayDir.y),
);

let sideDist = new Vector2(0, 0);
let step = new Vector2(0, 0);

//calculate step and initial sideDist
if (rayDir.x < 0) {
  step.x = -1;
  sideDist.x = (this.player.pos.x - mapPos.x) * deltaDist.x;
}
else {
  step.x = 1;
  sideDist.x = (mapPos.x + 1.0 - this.player.pos.x) * deltaDist.x;
}
if (rayDir.y < 0) {
  step.y = -1;
  sideDist.y = (this.player.pos.y - mapPos.y) * deltaDist.y;
}
else {
  step.y = 1;
  sideDist.y = (mapPos.y + 1.0 - this.player.pos.y) * deltaDist.y;
}
```

Now we can start the DDA loop. In each iteration, we perform the following:

Compare sideDist.x and sideDist.y to see which grid line (vertical or horizontal)
is closer. We use <= instead of < to create a consistent tie-breaker, which
prevents errors when the ray passes very close to a grid corner.

If the vertical line (`sideDist.x`) is closer, we increment `mapPos.x` by `step.x`
and add `deltaDist.x` to `sideDist.x` to find the distance to the next vertical line.
Otherwise, we do the same for the Y-axis.

After taking a step, we check if the new `mapPos` is a wall tile. If it is, we
set hit = true and the loop terminates.

```typescript
while (!hit) {
  if (sideDist.x <= sideDist.y) {
    sideDist.x += deltaDist.x;
    mapPos.x += step.x;
    side = 0;
  }
  else {
    sideDist.y += deltaDist.y;
    mapPos.y += step.y;
    side = 1;
  }

  if (mapPos.x < 0 || mapPos.y < 0 || mapPos.x >= this.map.width || mapPos.y >= this.map.height) {
    break;
  }

  //Check if ray has hit a wall
  if (this.map.data[mapPos.y][mapPos.x] > 0) {
    hit = true;
  }
}
```

The canvas below shows a simple implementation where a line is drawn from the center in the direction of 
the mouse, with dots indicating where the line crosses grid boundaries.

<figure>
    <canvas id="grid-canvas"></canvas>
</figure>

## Projection plane

The rendering of the screen is done with a projection plane or near clipping plane. The plane is calculated
from the player position (`pos`) and the direction of view (`dir`). the center of the projection plane
is `pos` + `dir`. From here we the field of view is from this position plus or minus the length of the
plane. `P1 = pos + dir + plane`, `P2 = pos + dir - plane`.

```typescript
let planeX = 0, planeY = 0.66;
const WIDTH = 100; //example screen width
for(let x = 0; x < WIDTH; x++)
{
  //calculate ray position and direction
  let cameraX = 2 * x / WIDTH - 1; //x-coordinate in camera space
  let ray = new Vector2(
   rayDir.x + planeX * cameraX,
   rayDir.y + planeY * cameraX,
  );
  const startPoint = this.player.pos.mul(cellSize);
  const endPoint = this.player.pos.add(ray).mul(cellSize);

  ctx.strokeStyle = 'green'; 
  ctx.lineWidth = 4;

  ctx.beginPath();
  ctx.moveTo(startPoint.x, startPoint.y);
  ctx.lineTo(endPoint.x, endPoint.y);
  ctx.stroke();
}
```

<figure>
  <canvas id="view-canvas"></canvas>
</figure>

## Draw the 3d scene

Now we can draw the 3d scene. 

<figure>
  <canvas id="wall-canvas" tabindex="0"></canvas>
</figure>

## Add Textures to the walls

Now we can draw the 3d scene. 

<figure>
  <canvas id="final-canvas" tabindex="0"></canvas>
</figure>

