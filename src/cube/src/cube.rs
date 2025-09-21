const cubeVertexSize: usize = 4 * 10; // Byte size of one cube vertex.
const cubePositionOffset: usize = 0;
const cubeColorOffset: usize = 4 * 4; // Byte offset of cube vertex color attribute.
const cubeUVOffset: usize = 4 * 8;
const cubeVertexCount: usize = 36;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
    uv: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

// prettier-ignore
// export const cubeVertexArray = new Float32Array([
//   // float4 position, float4 color, float2 uv,
//   1, -1, 1, 1,   1, 0, 1, 1,  0, 1,
//   -1, -1, 1, 1,  1, 0, 1, 1,  1, 1,
//   -1, -1, -1, 1, 1, 0, 1, 1,  1, 0,
//   1, -1, -1, 1,  1, 0, 1, 1,  0, 0,
//   1, -1, 1, 1,   1, 0, 1, 1,  0, 1,
//   -1, -1, -1, 1, 1, 0, 1, 1,  1, 0,
//
//   1, 1, 1, 1,    1, 1, 1, 1,  0, 1,
//   1, -1, 1, 1,   1, 0, 1, 1,  1, 1,
//   1, -1, -1, 1,  1, 0, 0, 1,  1, 0,
//   1, 1, -1, 1,   1, 1, 0, 1,  0, 0,
//   1, 1, 1, 1,    1, 1, 1, 1,  0, 1,
//   1, -1, -1, 1,  1, 0, 0, 1,  1, 0,
//
//   -1, 1, 1, 1,   0, 1, 1, 1,  0, 1,
//   1, 1, 1, 1,    1, 1, 1, 1,  1, 1,
//   1, 1, -1, 1,   1, 1, 0, 1,  1, 0,
//   -1, 1, -1, 1,  0, 1, 0, 1,  0, 0,
//   -1, 1, 1, 1,   0, 1, 1, 1,  0, 1,
//   1, 1, -1, 1,   1, 1, 0, 1,  1, 0,
//
//   -1, -1, 1, 1,  0, 0, 1, 1,  0, 1,
//   -1, 1, 1, 1,   0, 1, 1, 1,  1, 1,
//   -1, 1, -1, 1,  0, 1, 0, 1,  1, 0,
//   -1, -1, -1, 1, 0, 0, 0, 1,  0, 0,
//   -1, -1, 1, 1,  0, 0, 1, 1,  0, 1,
//   -1, 1, -1, 1,  0, 1, 0, 1,  1, 0,
//
//   1, 1, 1, 1,    1, 1, 1, 1,  0, 1,
//   -1, 1, 1, 1,   0, 1, 1, 1,  1, 1,
//   -1, -1, 1, 1,  0, 0, 1, 1,  1, 0,
//   -1, -1, 1, 1,  0, 0, 1, 1,  1, 0,
//   1, -1, 1, 1,   1, 0, 1, 1,  0, 0,
//   1, 1, 1, 1,    1, 1, 1, 1,  0, 1,
//
//   1, -1, -1, 1,  1, 0, 0, 1,  0, 1,
//   -1, -1, -1, 1, 0, 0, 0, 1,  1, 1,
//   -1, 1, -1, 1,  0, 1, 0, 1,  1, 0,
//   1, 1, -1, 1,   1, 1, 0, 1,  0, 0,
//   1, -1, -1, 1,  1, 0, 0, 1,  0, 1,
//   -1, 1, -1, 1,  0, 1, 0, 1,  1, 0,
// ]);

pub const VERTICES: &[Vertex] = &[
    // Bottom face (Y = -1)
    Vertex {
        position: [1.0, -1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    // Right face (X = 1)
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    // Top face (Y = 1)
    Vertex {
        position: [-1.0, 1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0, 1.0],
        color: [0.0, 0.0, 1.0, 1.0],
        uv: [1.0, 0.0],
    },
    // Left face (X = -1)
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [0.0, 1.0, 1.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0, 1.0],
        color: [0.0, 1.0, 1.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [0.0, 1.0, 1.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0, 1.0],
        color: [0.0, 1.0, 1.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [0.0, 1.0, 1.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [0.0, 1.0, 1.0, 1.0],
        uv: [1.0, 0.0],
    },
    // Front face (Z = 1)
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [1.0, 1.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0, 1.0],
        color: [1.0, 1.0, 0.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [1.0, 1.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0, 1.0],
        color: [1.0, 1.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0, 1.0],
        color: [1.0, 1.0, 0.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0, 1.0],
        color: [1.0, 1.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    // Back face (Z = -1)
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 1.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 1.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [1.0, 0.0, 1.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0, 1.0],
        color: [1.0, 0.0, 1.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0, 1.0],
        color: [1.0, 0.0, 1.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0, 1.0],
        color: [1.0, 0.0, 1.0, 1.0],
        uv: [1.0, 0.0],
    },
];
