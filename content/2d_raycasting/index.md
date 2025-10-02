+++
title = '2D Raycasting'
description = 'A creative coding exercise demonstrating 2D raycasting from a moving particle, with calculations offloaded to the GPU using the WebGPU API.'
date = 2025-09-21T12:35:00+02:00
draft = false
tags = ['graph', 'webgpu', 'creative-coding']
script = "2d_raycasting/js/main.ts"
links = [ 'line-line-intersection' ]
+++

We begin with raycasting as a creative coding exercise. We create a room with
random walls and a point particle that moves around. From this particle, we
cast hundreds of rays in all directions to find the nearest wall each ray
intersects with.

To handle the large number of intersection calculations (360 rays against many
walls per frame) efficiently, this simulation is implemented using **WebGPU**.
A compute shader calculates all ray-wall intersections in parallel on the GPU,
and a render pipeline then draws the resulting rays and walls.

## Drawing the Walls

First, we define the boundaries for our simulation. This includes walls
randomly placed within the scene and invisible walls along the canvas border to
ensure the rays always hit a surface.

```typescript
const WIDTH = 1280; 
const HEIGHT = 860; 
const TRANSPARENT = "#00000000"; 
const WALLS_COUNT = 5; 
 
// The walls array holds the start and end points for each wall segment. 
this.walls = [ 
  // Invisible canvas border walls 
  { start: new Vector2(1, 1), end: new Vector2(WIDTH - 1, 1), color: TRANSPARENT }, 
  { start: new Vector2(WIDTH - 1, 1), end: new Vector2(WIDTH - 1, HEIGHT - 1), color: TRANSPARENT }, 
  { start: new Vector2(WIDTH - 1, HEIGHT - 1), end: new Vector2(1, HEIGHT - 1), color: TRANSPARENT }, 
  { start: new Vector2(1, HEIGHT - 1), end: new Vector2(1, 1), color: TRANSPARENT }, 
]; 
 
// Randomly placed inner walls 
for (let i = 0; i < WALLS_COUNT; i++) { 
  this.walls.push({ 
    start: new Vector2(Math.random() * WIDTH, Math.random() * HEIGHT), 
    end: new Vector2(Math.random() * WIDTH, Math.random() * HEIGHT), 
    color: this.black 
  }); 
} 
```

These wall coordinates are then passed to the GPU to be used in the raycasting
calculations and for rendering.

<figure class='fullwidth'>
<canvas id="wall-canvas" class='fullwidth'></canvas>
</figure>

## Creating a Mover with Wander Behavior

Instead of simple random movement, the particle (or "mover") uses a classic
"Wander" steering behavior to create a more natural and fluid motion. This is
achieved by:
1.  Projecting a virtual circle in front of the mover, in the direction of its current velocity.
2.  Selecting a slightly randomized target point on the circumference of this circle.
3.  Calculating a steering force that directs the mover towards this target point.

This logic is implemented in the `move()` and `calculateForce()` methods, which
run on the CPU each frame to update the mover's position.

```typescript
private calculateForce() { 
    // Project a circle in front of the mover based on its velocity 
    this.circleCenter = this.velocity.length() > 0 
      ? this.velocity.norm().mul_scalar(CIRCLE_DISTANCE) 
      : new Vector2(CIRCLE_DISTANCE, 0); 
 
    // Add a small random displacement to the angle 
    this.theta += (Math.random() * 2 - 1) * 0.3; 
     
    // Calculate the new target on the circle's circumference 
    this.target = new Vector2( 
      Math.cos(this.theta) * CIRCLE_RADIUS, Math.sin(this.theta) * CIRCLE_RADIUS 
    ); 
} 
 
private move() { 
    // The wander force is the vector towards the target on the circle 
    const wanderForce = this.circleCenter.add(this.target); 
 
    // Apply physics: acceleration, velocity, position 
    this.acceleration = this.acceleration.add(wanderForce); 
    this.velocity = this.velocity.add(this.acceleration); 
    this.velocity = this.velocity.limit(this.maxSpeed); 
    this.position = this.position.add(this.velocity); 
    this.acceleration = this.acceleration.mul_scalar(0); // Reset acceleration 
 
    // Wrap around canvas edges 
    if (this.position.x > WIDTH) this.position.x = 0; 
    if (this.position.x < 0) this.position.x = WIDTH; 
    if (this.position.y > HEIGHT) this.position.y = 0; 
    if (this.position.y < 0) this.position.y = HEIGHT; 
} 
```

The visualization of the wander behavior (the circle, target, and vectors) is
drawn on a separate 2D canvas that is overlaid on top of the main WebGPU
canvas.

<figure class='fullwidth'>
<canvas id="mover-canvas" class='fullwidth'></canvas>
</figure>

## Raycasting on the GPU

Finally, we cast the rays. The intersection logic for two line segments is
based on a parametric formula. For a wall segment from P1 to P2 and a ray from
P3 to P4, we find scalar values `t` and `u` such that the intersection point
`P` satisfies `P = P1 + t * (P2 - P1)` and `P = P3 + u * (P4 - P3)`.

The intersection point lies on both segments if `0 <= t <= 1` and `u >= 0`
(since the ray extends infinitely in one direction). We calculate `u` for every
wall and find the smallest positive value to identify the closest intersection
point.

The formulas for `t` and `u` are:

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

This entire calculation is performed in parallel for every ray inside a WGSL
compute shader.

Here is the core logic from the shader, which iterates through all walls for a
single ray to find the closest intersection point. The final intersection point
is then calculated simply by `moverPosition + rayDirection * closest_u`.

```wgsl
@compute @workgroup_size(64) 
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) { 
    let ray_index = global_id.x; 
    if (ray_index >= params.rayCount) { 
        return; 
    } 
 
    let ray_angle = (f32(ray_index) / f32(params.rayCount)) * 2.0 * PI; 
    let ray_dir = vec2<f32>(cos(ray_angle), sin(ray_angle)); 
 
    var closest_t = 100000.0; // Represents the 'u' value for the ray 
 
    // Check intersection with every wall 
    for (var i: u32 = 0u; i < params.wallCount; i = i + 1u) { 
        let wall = walls[i]; 
        let x1 = wall.p1.x; let y1 = wall.p1.y; 
        let x2 = wall.p2.x; let y2 = wall.p2.y; 
 
        let x3 = params.moverPos.x; let y3 = params.moverPos.y; 
        let x4 = params.moverPos.x + ray_dir.x; let y4 = params.moverPos.y + ray_dir.y; 
 
        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4); 
        if (den == 0.0) { 
            continue; // Lines are parallel 
        } 
 
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den; 
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den; 
 
        // Check if intersection is on the wall segment and in front of the ray's origin 
        if (t > 0.0 && t < 1.0 && u > 0.0) { 
            if (u < closest_t) { 
                closest_t = u; // Found a closer wall 
            } 
        } 
    } 
 
    // Calculate the intersection point and write it to the vertex buffer 
    let intersection_point = params.moverPos + ray_dir * closest_t; 
 
    // Each ray is a line segment from the mover to the intersection point 
    rayVertices[ray_index * 2u] = params.moverPos; 
    rayVertices[ray_index * 2u + 1u] = intersection_point; 
} 
```

The final result, combining the GPU-rendered walls and rays with the
CPU-rendered mover details, creates the complete, interactive visualization.

<figure class='fullwidth'>
<canvas id="webgpu-canvas" class='fullwidth'></canvas>
</figure>

