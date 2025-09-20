struct Uniforms {
  resolution: vec2<f32>,
  grid_size: vec2<f32>,
  time: f32,
  birth_rule: u32,
  survive_rule: u32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2f,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> cellStateIn: array<u32>;
@group(0) @binding(2) var<storage, read_write> cellStateOut: array<u32>;


fn cellIndex(cell: vec2u) -> u32 {
   let clamped_cell = min(cell, vec2u(uniforms.grid_size) - vec2u(1,1));
   return clamped_cell.y * u32(uniforms.grid_size.x) + clamped_cell.x;
}

fn cellActive(x: u32, y: u32) -> u32 {
   let wrapped_x = (x + u32(uniforms.grid_size.x)) % u32(uniforms.grid_size.x);
   let wrapped_y = (y + u32(uniforms.grid_size.y)) % u32(uniforms.grid_size.y);
   
   if (cellStateIn[cellIndex(vec2(wrapped_x, wrapped_y))] > 0u) {
     return 1u;
   }
   return 0u;
}

fn is_in_rule(neighbors: u32, rule_mask: u32) -> bool {
    // e.g., if neighbors is 3 and mask is for B3 (..001000),
    // (mask >> 3) is (..000001).
    // (..000001) & 1u is 1u, so it returns true.
    return ((rule_mask >> neighbors) & 1u) == 1u;
}

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
//    if (state > 0u) { // If the cell is ALIVE
//      // A living cell survives if it has 2 or 3 neighbors.
//      if (activeNeighbors == 2u || activeNeighbors == 3u) {
//        // It survives, so increment its age.
//        cellStateOut[i] = state + 1u;
//      } else {
//        // It dies from over/underpopulation.
//        cellStateOut[i] = 0u;
//      }
//    } else { // If the cell is DEAD
//      // A dead cell is born if it has exactly 3 neighbors.
//      if (activeNeighbors == 3u) {
//        // It's born! Start its age at 1.
//        cellStateOut[i] = 1u;
//      } else {
//        // It stays dead.
//        cellStateOut[i] = 0u;
//      }
//    }

   if (state > 0u) { // If the cell is ALIVE
     // A living cell survives if the number of neighbors is in the survive rule.
     if (is_in_rule(activeNeighbors, uniforms.survive_rule)) {
       // It survives, so increment its age.
       cellStateOut[i] = state + 1u;
     } else {
       // It dies from over/underpopulation.
       cellStateOut[i] = 0u;
     }
   } else { // If the cell is DEAD
     // A dead cell is born if the number of neighbors is in the birth rule.
     if (is_in_rule(activeNeighbors, uniforms.birth_rule)) {
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
    vert: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    // Map vertex positions from [-1, 1] to UV coords [0, 1]
    output.uv = vert.position.xy * 0.5 + 0.5;
    output.position = vec4f(vert.position, 1.0);
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
   // Calculate which cell this pixel belongs to.
    let cell = vec2u(floor(in.uv * uniforms.grid_size));
    
    // Find the index of that cell and get its state.
    let state = f32(cellStateIn[cellIndex(cell)]);

    // The rest of the coloring logic is the same as before.
    if (state < 1.0) {
        // Discarding is an option, but returning transparent black is fine.
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Return black for dead cells
    }

    let max_age = 20.0;
    let t = clamp(state / max_age, 0.0, 1.0);

    let color_young = vec3<f32>(1.0, 0.0, 0.0); // Red
    let color_old = vec3<f32>(0.4, 0.0, 0.8);   // Dark Purple

    let final_color = mix(color_young, color_old, t);
    return vec4<f32>(final_color, 1.0);
}
 
