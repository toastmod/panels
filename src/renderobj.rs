use crate::rect::WorldPoint;
use crate::transform2d::Transform2D;
use crate::wgpustate::State;

pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x,y,z }
    }
}

pub struct RenderObject {
    pub(crate) position: Transform2D,
    pub(crate) pipeline: usize,
    pub(crate) bind_group: usize,
    pub(crate) model: usize,
}

impl RenderObject {
    // pub fn render<'a>(&self, state: &'a State, render_pass: &'a mut wgpu::RenderPass<'a>) {
    //
    //     let my_model = &state.models[self.model];
    //
    //     render_pass.set_pipeline(&state.render_pipelines[self.pipeline]);
    //     render_pass.set_bind_group(0, &state.bind_groups[self.bind_group], &[]);
    //     render_pass.set_vertex_buffer(0, my_model.vertex_buffer.slice(..));
    //     render_pass.set_index_buffer(my_model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //     render_pass.draw_indexed(0..my_model.num_indices, 0, 0..1);
    // }
}