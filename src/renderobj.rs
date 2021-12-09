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
//
// /// Acts as an interface between the application's data and the rendering object.
// /// Provides access to the WGPU state data associated with a RenderObject, and to interact with the master State.
// pub enum RenderState {
//     /// The pointer is currently stored in the master State's objects array for rendering.
//     Rendering(usize),
//
//     /// The object's pointer is stored here and is not actively rendering.
//     NotRendering(Box<RenderObject>)
// }

// impl RenderState {
//     // /// Access the RenderObject's data, write to it's buffers, etc.
//     // pub fn edit_uniforms<'a>(
//     //     &self,
//     //     state: &'a mut State,
//     //     f: Box<Fn(
//     //         Vec<&'a mut wgpu::Buffer>
//     //     ) -> ()>
//     // ) {
//     //     let mut unifs: Vec<&'a mut wgpu::Buffer> = vec![];
//     //     match self {
//     //         RenderState::Rendering(roid) => {
//     //             // let ro = state.objects.get(roid).unwrap()
//     //             // for i in 0..ro.uniforms.len() {
//     //             //     unifs.push(state.uniform_buffers.get_mut(i).unwrap())
//     //             // }
//     //         }
//     //         RenderState::NotRendering(ro) => {
//     //             for i in 0..ro.uniforms.len() {
//     //                 unifs.push(state.uniform_buffers.get_mut(i).unwrap())
//     //             }
//     //         }
//     //     }
//     //
//     // }
// }

/// Data for a renderable object.
pub struct RenderObject {
    pub position: WorldPoint,
    pub pipeline: usize,
    pub bind_group: usize,
    pub model: usize,
    pub uniforms: Vec<usize>,
}

impl RenderObject {

    pub fn new(state: &mut State) -> Self {
        // create/choose pipeline
        // create/choose bindgroup
        // create/choose model
        // create uniforms

        Self {
            position: WorldPoint::new(0.0,0.0,0.0),
            pipeline: 0,
            bind_group: 0,
            model: 0,
            uniforms: vec![],
        }
        
    }

    pub fn new_placeholder_rect() -> Self {
        RenderObject {
            // default 0.0,0.0 position
            position: WorldPoint::new(0.0, 0.0, 0.0),
            // default textured verticies pipeline/shader
            pipeline: 0,
            // default tree_texure and position uniforms
            bind_group: 0,
            // default rect model (vert/index buffer)
            model: 0,
            uniforms: vec![],
        }
    }

    pub fn render_this<'a>(&self, state: &'a State, render_pass: &mut wgpu::RenderPass<'a>) {

        let my_model = &state.models[self.model];

        render_pass.set_pipeline(&state.render_pipelines[self.pipeline]);
        render_pass.set_bind_group(0, &state.bind_groups[self.bind_group], &[]);
        render_pass.set_vertex_buffer(0, my_model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(my_model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..my_model.num_indices, 0, 0..1);
    }
}