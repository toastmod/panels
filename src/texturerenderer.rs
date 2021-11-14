use std::time::{Duration, Instant};
use wgpu::SurfaceTexture;
use winit::event::WindowEvent;
use crate::programhook::ProgramHook;
use crate::renderobj::RenderObject;
use crate::texture::Texture;
use crate::timing::CallStatus;
use crate::transform2d::Transform2D;
use crate::wgpustate::State;


pub enum TextureIndex {
    Surface,
    Index(usize)
}

/// The bridge between Textures and program functionality.
pub struct TextureRenderer {
    pub texture: TextureIndex,
    pub clear_or_load: wgpu::LoadOp<wgpu::Color>,
    pub this_object: RenderObject,
    /// Other `TextureRenderer`s representing subprograms.
    pub owned_elements: Vec<TextureRenderer>,

    /// The index of the `ProgramHook` associated with this `TextureRenderer`
    pub program_id: Option<usize>,

    ///// The function that will be called according to `Timing` or on a (forced) redraw request.
    //pub drawf: Box<DrawFunc>,
    pub drawf_status: CallStatus,

    ///// A passive callback function that will be called according to some `Timing`
    //pub updatef: Box<UpdateFunc>,
    pub updatef_status: CallStatus,

    ///// The function that will be called when a WindowEvent is fired in this renderer.
    //pub inputf: Box<InputFunc>
}

impl TextureRenderer {
    /// Creates a `TextureRenderer` that uses the given routine to draw elements.
    pub fn new(
        program_id: Option<usize>,
        tex_index: TextureIndex,
    ) -> Self {
        // create texture
        // create uniforms
        // create bindgroup for both

        let mut this_object = RenderObject{
            position: Transform2D::new(0.0,0.0,0.0),
            pipeline: 0,
            bind_group: 0,
            model: 0,
            uniforms: vec![]
        };

        Self {
            texture: tex_index,
            clear_or_load: wgpu::LoadOp::Load,
            this_object,
            owned_elements: vec![],
            program_id,
            // drawf,
            drawf_status: CallStatus::Inactive,
            // updatef,
            updatef_status: CallStatus::Inactive,
            // inputf
        }
    }

    pub fn hook_program(&mut self, program_id: usize) {
        self.program_id = Some(program_id);
    }

    pub fn set_clear(&mut self, loadop: wgpu::LoadOp<wgpu::Color>) {
        self.clear_or_load = loadop;
    }

    // pub fn render(self, state: &mut State, encoder: &mut wgpu::CommandEncoder ) {
    //     self.program_hook.render(self, state, encoder)
    // }
    //
    // pub fn update(&mut self, state: &mut State) {
    //     self.program_hook.update(self, state)
    // }
    //
    // pub fn input(&mut self, state: &mut State, event: &WindowEvent) {
    //     self.program_hook.input(self, state, event)
    // }

    pub fn get_textureview(&self, state: &mut State) -> Result<wgpu::TextureView, wgpu::SurfaceError> {
        match self.texture {
            TextureIndex::Surface => {
                Ok(state.surface.get_current_texture()?.texture.create_view(&wgpu::TextureViewDescriptor::default()))
            }
            TextureIndex::Index(i) => {
                Ok(state.textures[i].texture.create_view(&wgpu::TextureViewDescriptor::default()))
            }
        }
    }

    /// Set the `CallStatus` directly.
    pub fn set_draw_status(&mut self, rs: CallStatus) {
        self.drawf_status = rs;
    }

    /// Get a mutable reference to the draw function's `CallStatus` (useful for std::mem:swap)
    pub fn mut_draw_status(&mut self) -> &mut CallStatus {
        &mut self.drawf_status
    }

    /// Get an immutable reference to the draw function's `CallStatus` (useful for analysis).
    pub fn ref_draw_status(&self) -> &CallStatus {
        &self.drawf_status
    }
}