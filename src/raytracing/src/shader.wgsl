// shader.wgsl

// This struct matches the Camera uniform buffer in our Rust code.
@group(0) @binding(0)
var<uniform> camera: mat4x4<f32>;

// Vertex shader input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// Instance shader input
// We receive the 4x4 model matrix in 4 separate vec4 locations.
struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
};

// This struct is the output of the vertex shader and the input to the fragment shader.
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    // Reconstruct the 4x4 model matrix from the instance input.
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    // Calculate the final position: Camera * Model * VertexPosition
    out.clip_position = camera * model_matrix * vec4<f32>(model.position, 1.0);
    // Pass the vertex color through to the fragment shader.
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // The fragment shader is very simple: it just outputs the color it received.
    return vec4<f32>(in.color, 1.0);
}
