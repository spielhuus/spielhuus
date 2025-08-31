// Vertex shader

struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
  center: vec2<f32>,
  radius: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if length(in.clip_position.xy - uniforms.center) < uniforms.radius {
        return vec4<f32>(0.0);
    } else {
        return vec4<f32>(1.0);
    }
}
 
