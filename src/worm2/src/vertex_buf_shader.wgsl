struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) model_pos: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.model_pos = model.position.xy;
    return out;
}

fn rotate(p: vec2<f32>, a: f32) -> vec2<f32> {
  return vec2<f32>(p.x * cos(a) - p.y * sin(a),
                   p.x * sin(a) + p.y * cos(a));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  var p = in.model_pos * 0.2;

  let aspect = uniforms.resolution.x / uniforms.resolution.y;
  p.x = p.x * aspect;

  let t = uniforms.time;

  let p0 = vec2<f32>(sin(p.x * 10.0), cos(p.y * 10.0));
  let p1 = rotate(p0, t * 0.5) * sin(t * 0.1);

  let red   = sin(abs(p1.x * p1.y * p.y) * 50.0 + 2.0 + t * 2.0);
  let green = cos(p1.x * 10.0 + t) * sin(p1.y * 10.0 + t);
  let blue  = sin((p.x + p1.y) * 0.5 + t);

  return vec4<f32>(red, green, blue, 1.0);
}

