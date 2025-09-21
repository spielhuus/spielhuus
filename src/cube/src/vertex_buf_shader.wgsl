// src/vertex_buf_shader.wgsl

// This struct MUST match the `Uniforms` struct in your Rust code.
// The `mat4x4<f32>` is for the transformation matrix.
struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    // Note: No padding field is needed in WGSL. The memory layout rules handle it.
    transform: mat4x4<f32>,
};

// This struct defines the data that is passed from the Rust code for each vertex.
// The `@location` numbers MUST match the `shader_location` in your Vertex::desc().
struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

// This struct defines the data that the vertex shader will pass to the fragment shader.
// The `@builtin(position)` is the final clip-space coordinate of the vertex.
// We also pass the vertex color through to the fragment shader.
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

// We bind our uniform buffer to group 0, binding 0.
// This MUST match the `create_bind_group_layout` and `create_bind_group` calls.
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;


// --- VERTEX SHADER ---
// This function runs for every vertex in your vertex buffer.
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform the vertex's position from model space to clip space
    // using the matrix we passed in via the uniform buffer.
    out.clip_position = uniforms.transform * model.position;

    // Pass the vertex's color directly to the fragment shader.
    // The GPU will automatically interpolate this color across the triangle's surface.
    out.color = model.color;

    return out;
}


// --- FRAGMENT SHADER ---
// This function runs for every pixel on the screen that a triangle covers.
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // We simply return the interpolated color we received from the vertex shader.
    // This will create a cube with smoothly blended colors on each face.
    return in.color;
}
