use crate::rect::WorldPoint;
use crate::transform2d::Transform2D;
use crate::wgpustate::State;
use crate::modelbuffers::Model;
use wgpu::Buffer;

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

/// Acts as an interface between the application's data and the rendering object.
/// Provides access to the WGPU state data associated with a RenderObject, and to interact with the master State.
pub enum RenderState {
    /// The pointer is currently stored in the master State's objects array for rendering.
    Rendering(usize),

    /// The object's pointer is stored here and is not actively rendering.
    NotRendering(Box<RenderObject>)
}

impl RenderState {
    // /// Access the RenderObject's data, write to it's buffers, etc.
    // pub fn edit_uniforms<'a>(
    //     &self,
    //     state: &'a mut State,
    //     f: Box<Fn(
    //         Vec<&'a mut wgpu::Buffer>
    //     ) -> ()>
    // ) {
    //     let mut unifs: Vec<&'a mut wgpu::Buffer> = vec![];
    //     match self {
    //         RenderState::Rendering(roid) => {
    //             // let ro = state.objects.get(roid).unwrap()
    //             // for i in 0..ro.uniforms.len() {
    //             //     unifs.push(state.uniform_buffers.get_mut(i).unwrap())
    //             // }
    //         }
    //         RenderState::NotRendering(ro) => {
    //             for i in 0..ro.uniforms.len() {
    //                 unifs.push(state.uniform_buffers.get_mut(i).unwrap())
    //             }
    //         }
    //     }
    //
    // }
}

pub struct RenderObject {
    pub(crate) position: Transform2D,
    pub(crate) pipeline: usize,
    pub(crate) bind_group: usize,
    pub(crate) model: usize,
    pub(crate) uniforms: Vec<usize>
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