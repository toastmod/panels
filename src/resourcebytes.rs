
#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct Vertex {
    pub position: [f32;3],
    pub tex_coords: [f32;2]
}

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ],
        }
    }
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}


// pub const TRIANGLE_VERTICES: &[Vertex] = &[
//     Vertex {
//         position:[0.0,0.5,0.0], color: [1.0,0.0,0.0]
//     },
//     Vertex {
//         position: [-0.5,-0.5,0.0], color: [0.0,1.0,0.0]
//     },
//     Vertex {
//         position: [0.5,-0.5,0.0], color: [0.0,0.0,1.0]
//     },
// ];

pub const PENTAGON_VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 1.0-0.99240386], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 1.0-0.56958646], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 1.0-0.050602943], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 1.0-0.15267089], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 1.0-0.7347359], }, // E
];

pub const PENTAGON_INDICES: &[u16] = &[
    0,1,4,
    1,2,4,
    2,3,4,
    // padding
    0,
];

pub const diffuse_bytes: &[u8] = include_bytes!("happy-tree.png");

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
