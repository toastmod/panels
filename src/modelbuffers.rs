pub struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub offset_buffer: wgpu::Buffer,
    pub num_indices: u32
}