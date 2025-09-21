struct OurVertexShaderOutput {
  @builtin(position) position: vec4f,
};

struct Vertex {
  @location(0) position: vec2f,
};
struct OurStruct {
  offset: vec3f,
  color: vec4f,
};

@group(0) @binding(0) var<storage, read> ourStructs: array<OurStruct>;

@vertex fn vs_main(vert: Vertex, @builtin(instance_index) instanceIndex: u32) -> OurVertexShaderOutput {
  let pos = array(
    vec2f( 0.0,  0.5),  // top center
    vec2f(-0.5, -0.5),  // bottom left
    vec2f( 0.5, -0.5)   // bottom right
  );
  var vsOutput: OurVertexShaderOutput;
  vsOutput.position = vec4f(vert[instanceIndex], 0.0, 1.0);
  return vsOutput;
}

@fragment fn fs_main(fsInput: OurVertexShaderOutput) -> @location(0) vec4f {

        let red = vec4f(1, 0, 0, 1);
        let cyan = vec4f(0, 0, 1, 1);

        let grid = vec2u(fsInput.position.xy) / 16;
        let checker = (grid.x + grid.y) % 2 == 1;

        return select(red, cyan, checker);
}
