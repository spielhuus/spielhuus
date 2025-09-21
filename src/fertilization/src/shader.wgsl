// Vertex shader

// Represents the data for a single vertex of our square
struct VertexInput {
    @location(0) position: vec2<f32>,
};

// Represents the per-instance data (one for each rectangle)
struct InstanceInput {
    @location(1) model_matrix_0: vec4<f32>,
    @location(2) model_matrix_1: vec4<f32>,
    @location(3) model_matrix_2: vec4<f32>,
    @location(4) model_matrix_3: vec4<f32>,
    @location(5) color: vec4<f32>,
};

// Data passed from the vertex shader to the fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

// A global uniform buffer for data that's the same for all instances in a draw call
@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.color = instance.color;
    // Transform the vertex position by the model matrix, then the projection matrix
    out.clip_position = projection * model_matrix * vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simply output the color passed from the vertex shader
    return in.color;
}
