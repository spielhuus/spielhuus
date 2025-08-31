// Uniforms that will be the same for all shader invocations.
@group(0) @binding(0) var<uniform> u_params: Params;

// The previous generation's state (read-only).
// We use a texture_storage_2d because we need integer coordinates and no filtering.
@group(0) @binding(1) var source_texture: texture_storage_2d<r32uint, read>;

// The new generation's state (write-only).
@group(0) @binding(2) var dest_texture: texture_storage_2d<r32uint, write>;

// For the render pipeline
@group(0) @binding(3) var display_texture: texture_2d<u32>;


struct Params {
    // The ECA rule, from 0 to 255.
    rule: u32,
    // The width and height of our automaton grid.
    width: u32,
    height: u32,
    current_generation: u32, 
};

// This function takes the 3-cell neighborhood (left, center, right)
// and looks up the new state based on the rule.
fn get_rule_output(left: u32, center: u32, right: u32, rule: u32) -> u32 {
    // Combine the 3 cells into a 3-bit number (0-7).
    let index = (left << 2) | (center << 1) | right;
    // Use the index to check the corresponding bit in the 8-bit rule.
    return (rule >> index) & 1u;
}

// -------------------- COMPUTE SHADER --------------------

// Our workgroup size. Can be tuned for performance. 64 is a good start.
@compute @workgroup_size(64, 1, 1)
fn compute_main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let x = global_id.x;
    let y = u_params.current_generation;

    // Stop if we're past the texture dimensions
    if (x >= u_params.width || y >= u_params.height) {
        return;
    }

    // This is the generation we are *writing to*.
    let current_gen = y;
    // We read from the generation *before* it.
    let prev_gen = y - 1u;

    // Handle the first generation (y=0) separately, as it has no parent.
    // We let it be initialized by the CPU and do no work here.
    if (current_gen == 0) {
        return;
    }

    // Get the texture dimensions for wrapping logic
    let dims = textureDimensions(source_texture);

    // Read the 3-cell neighborhood from the previous generation (prev_gen).
    // We use modulo arithmetic for wrapping (toroidal) boundary conditions.
    let left_x = (x + u_params.width - 1u) % u_params.width;
    let center_x = x;
    let right_x = (x + 1u) % u_params.width;

    // textureLoad wants an ivec2, so we cast.
    let left_val = textureLoad(source_texture, vec2<i32>(i32(left_x), i32(prev_gen))).r;
    let center_val = textureLoad(source_texture, vec2<i32>(i32(center_x), i32(prev_gen))).r;
    let right_val = textureLoad(source_texture, vec2<i32>(i32(right_x), i32(prev_gen))).r;
    
    // Calculate the new cell state.
    let new_state = get_rule_output(left_val, center_val, right_val, u_params.rule);

    // Write the new state to the destination texture.
    textureStore(dest_texture, vec2<i32>(i32(x), i32(current_gen)), vec4<u32>(new_state, 0, 0, 0));
}


// -------------------- RENDER SHADER --------------------

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

// A simple vertex shader that creates a full-screen triangle.
// This is more efficient than a quad as it uses 3 vertices instead of 4/6.
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // Map vertex index to a full-screen triangle
    let x = f32(in_vertex_index / 2u) * 4.0 - 1.0;
    let y = f32(in_vertex_index % 2u) * 4.0 - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(
        (out.clip_position.x + 1.0) / 2.0,
        1.0 - (out.clip_position.y + 1.0) / 2.0 // Flip Y
    );
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Get the dimensions of the texture so we can map our
    // normalized (0.0-1.0) tex_coords to integer pixel coordinates.
    let dims = textureDimensions(display_texture);

    // Calculate the integer coordinates.
    let coords = vec2<i32>(floor(in.tex_coords * vec2<f32>(dims)));

    // FIX: Use textureLoad to fetch the exact integer value from the texture.
    // The second argument to textureLoad is the integer coordinate, the third is the mip level (0 for us).
    let int_color = textureLoad(display_texture, coords, 0).r;

    // Cast the u32 value (0 or 1) to an f32 (0.0 or 1.0).
    let color = f32(int_color);

    // Now construct the final f32 color for output.
    return vec4<f32>(color, color, color, 1.0);
}
