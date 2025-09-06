@group(0) @binding(0) var<uniform> u_params: Params;
@group(0) @binding(1) var source_texture: texture_storage_2d<r32uint, read>;
@group(0) @binding(2) var dest_texture: texture_storage_2d<r32uint, write>;
@group(0) @binding(1) var display_texture: texture_2d<u32>;

struct Params {
    // The ECA rule, from 0 to 255.
    rule: u32,
    width: u32,
    height: u32,
    size: u32,
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

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(in_vertex_index / 2u) * 4.0 - 1.0;
    let y = f32(in_vertex_index % 2u) * 4.0 - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(
        (out.clip_position.x + 1.0) / 2.0,
        1.0 - (out.clip_position.y + 1.0) / 2.0
    );
    return out;
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let texel_coords = vec2<i32>(frag_coord.xy / f32(u_params.size));
    let int_color = textureLoad(display_texture, texel_coords, 0).r;
    let color = f32(int_color);
    return vec4<f32>(0.9 * color, 0.0, 0.0, color);
}
