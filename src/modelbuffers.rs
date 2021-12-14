/// Data for a 3D model, and it's format.
pub struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_format: wgpu::IndexFormat,
    pub offset_buffer: wgpu::Buffer,
    pub num_indices: u32
}

pub struct ModelBuilder;

impl Model {
    pub fn new() -> ModelBuilder {
        ModelBuilder
    }
}

//TODO: load obj wavefront and mtl files