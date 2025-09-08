struct Uniforms {
  resolution: vec2<f32>,
  grid_size: vec2<f32>,
  time: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @builtin(instance_index) instance: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) cell: vec2f,
    @location(1) state: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> cellStateIn: array<u32>;
@group(0) @binding(2) var<storage, read_write> cellStateOut: array<u32>;


fn cellIndex(cell: vec2u) -> u32 {
   return (cell.y % u32(uniforms.grid_size.y)) * u32(uniforms.grid_size.x) +
           (cell.x % u32(uniforms.grid_size.x));
 }

fn cellActive(x: u32, y: u32) -> u32 {
   if (cellStateIn[cellIndex(vec2(x, y))] > 0u) {
     return 1u;
   }
   return 0u;}

@compute @workgroup_size(8, 8)
fn compute_main(@builtin(global_invocation_id) cell: vec3u) {
   // Determine how many active neighbors this cell has.
   let activeNeighbors = cellActive(cell.x+1, cell.y+1) +
                         cellActive(cell.x+1, cell.y) +
                         cellActive(cell.x+1, cell.y-1) +
                         cellActive(cell.x, cell.y-1) +
                         cellActive(cell.x-1, cell.y-1) +
                         cellActive(cell.x-1, cell.y) +
                         cellActive(cell.x-1, cell.y+1) +
                         cellActive(cell.x, cell.y+1);

   let i = cellIndex(cell.xy);

   // Conway's game of life rules:
//   switch activeNeighbors {
//     case 2: {
//       cellStateOut[i] = cellStateIn[i] + 1;
//     }
//     case 3: {
//       cellStateOut[i] = 1;
//     }
//     default: {
//       cellStateOut[i] = 0;
//     }
//   }

  let state = cellStateIn[i];
 // Correct Conway's game of life rules with aging:
   if (state > 0u) { // If the cell is ALIVE
     // A living cell survives if it has 2 or 3 neighbors.
     if (activeNeighbors == 2u || activeNeighbors == 3u) {
       // It survives, so increment its age.
       cellStateOut[i] = state + 1u;
     } else {
       // It dies from over/underpopulation.
       cellStateOut[i] = 0u;
     }
   } else { // If the cell is DEAD
     // A dead cell is born if it has exactly 3 neighbors.
     if (activeNeighbors == 3u) {
       // It's born! Start its age at 1.
       cellStateOut[i] = 1u;
     } else {
       // It stays dead.
       cellStateOut[i] = 0u;
     }
   }
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {

    let i = f32(model.instance);

    let cell = vec2<f32>(i % uniforms.grid_size.x, floor(i / uniforms.grid_size.x));
    let state = f32(cellStateIn[model.instance]); 
    let is_alive = select(0.0, 1.0, state > 0.0);

    let cell_offset = cell / uniforms.grid_size * 2.0;
    let gridPos = (model.position.xy * is_alive) / uniforms.grid_size - 1.0 + cell_offset;

    var output: VertexOutput;
    output.position = vec4f(gridPos, 0, 1);
    output.cell = cell;
    output.state = state;
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //return vec4<f32>(in.cell/uniforms.grid_size, 0, in.state);
    // return vec4<f32>(vec3<f32>(1.0, 0.0, 0.0) / in.state, in.state);

    // If the state is 0, the cell is dead, so we make it transparent.
    // The vertex shader already shrinks it to a point, but this is good practice.
    if (in.state < 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    let max_age = 20.0; // The age at which the color stops changing.

    // Normalize the age to a 0.0 - 1.0 range.
    // clamp() ensures the value doesn't go above 1.0.
    let t = clamp(in.state / max_age, 0.0, 1.0);

    // Define the start and end colors for our gradient.
    let color_young = vec3<f32>(1.0, 0.0, 0.0); // Bright Yellow
    let color_old = vec3<f32>(0.4, 0.0, 0.8);   // Dark Purple

    // Interpolate between the two colors based on the normalized age.
    let final_color = mix(color_young, color_old, t);

    // Return the final color with full opacity (alpha = 1.0).
    return vec4<f32>(final_color, 1.0);
}
 
