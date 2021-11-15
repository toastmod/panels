use crate::programhook::ProgramHook;
use crate::renderobj::RenderObject;
use crate::texture::Texture;
use crate::timing::{CallStatus, Timing};
use crate::transform2d::Transform2D;
use crate::util::fps_to_dur;
use crate::wgpustate::State;
use std::time::{Duration, Instant};
use wgpu::SurfaceTexture;
use winit::event::WindowEvent;

pub enum TextureIndex {
    Surface,
    Index(usize),
}

pub enum TextureViewQuery {
    RequestSurfaceView,
    View(wgpu::TextureView)
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

    /// The `CallStatus` for the function that will submit the renderer's `CommandBuffer` to the command queue and commit rendering.
    pub drawf_status: CallStatus,

    /// The `CallStatus` for the function that will poitentially update the renderer's `CommandBuffer`
    pub updatef_status: CallStatus,
    ///// The function that will be called when a WindowEvent is fired in this renderer.
    //pub inputf: Box<InputFunc>
}

impl TextureRenderer {
    /// Creates a `TextureRenderer` that uses the given routine to draw elements.
    pub fn new(program_id: Option<usize>, tex_index: TextureIndex) -> Self {
        // create texture
        // create uniforms
        // create bindgroup for both

        let mut this_object = RenderObject {
            position: Transform2D::new(0.0, 0.0, 0.0),
            pipeline: 0,
            bind_group: 0,
            model: 0,
            uniforms: vec![],
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

    pub fn should_call_drawf(&mut self, redraw_request: bool) -> bool {

        // instant-yes if this is a surface getting a redraw_request
        if redraw_request {
            match self.texture {
                TextureIndex::Surface => {
                    return true;
                }
                _ => {}
            };
        }

        match &self.drawf_status {
            CallStatus::Awaiting(timing) => match timing {
                Timing::ASAP => true,
                Timing::Framerate {
                    last_rendered_at,
                    desired_framerate,
                } => {
                    if last_rendered_at.elapsed() >= fps_to_dur(desired_framerate.clone()) {
                        true
                    } else {
                        false
                    }
                }
                Timing::SpecificTime {
                    last_rendered_at,
                    desired_wait_time,
                } => {
                    if last_rendered_at.elapsed() >= desired_wait_time.clone() {
                        true
                    } else {
                        false
                    }
                }

                Timing::Never => false,
            },
            CallStatus::Inactive => false,
            CallStatus::JustCalled(_) => false,
        }
    }


    pub fn should_call_updatef(&mut self) -> bool {
        match &self.updatef_status {
            CallStatus::Awaiting(timing) => match timing {
                Timing::ASAP => true,
                Timing::Framerate {
                    last_rendered_at,
                    desired_framerate,
                } => {
                    if last_rendered_at.elapsed() >= fps_to_dur(desired_framerate.clone()) {
                        true
                    } else {
                        false
                    }
                }
                Timing::SpecificTime {
                    last_rendered_at,
                    desired_wait_time,
                } => {
                    if last_rendered_at.elapsed() >= desired_wait_time.clone() {
                        true
                    } else {
                        false
                    }
                }

                Timing::Never => false,
            },
            CallStatus::Inactive => false,
            CallStatus::JustCalled(_) => false,
        }
    }

    pub fn hook_program(&mut self, program_id: usize) {
        self.program_id = Some(program_id);
    }

    pub fn set_clear(&mut self, loadop: wgpu::LoadOp<wgpu::Color>) {
        self.clear_or_load = loadop;
    }

    /// Retrieves the `TextureView` corresponding to this renderer.
    pub fn get_textureview(
        &self,
        state: &mut State,
    ) -> Result<TextureViewQuery, wgpu::SurfaceError> {
        match self.texture {
            TextureIndex::Surface => Ok(TextureViewQuery::RequestSurfaceView),
            TextureIndex::Index(i) => Ok(TextureViewQuery::View(state.textures[i]
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()))),
        }
    }

    // /// Attempts to load the `TextureView` into this renderer's memory.
    // pub fn load_textureview(&mut self, state: &mut State) {
    //     self.texture_view = Some(self.get_textureview(state).unwrap());
    // }

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
