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
    pub name: String,
    pub texture: TextureIndex,
    pub clear_or_load: wgpu::LoadOp<wgpu::Color>,

    /// Misc storage for separate objects that can be accessed at rendertime.
    pub my_objects: Vec<RenderObject>,

    // Other `TextureRenderer`s representing subprograms.
    // pub owned_elements: Vec<TextureRenderer>,

    /// The index of the `ProgramHook` associated with this `TextureRenderer`
    pub program_id: Option<usize>,

    /// The `Timing` frequency for the draw call.
    pub drawf_status: Timing,

    /// The `Timing` frequency for the update call.
    pub updatef_status: Timing,
    ///// The function that will be called when a WindowEvent is fired in this renderer.
    //pub inputf: Box<InputFunc>

}

impl TextureRenderer {
    /// Creates a `TextureRenderer` that uses the given routine to draw elements.
    pub fn new(nametag: &str, program_id: Option<usize>, tex_index: TextureIndex) -> Self {

        // create texture
        // create uniforms
        // create bindgroup for both

        // by default the textured rect is added as a placeholder
        let mut this_object = RenderObject::new_placeholder_rect();

        Self {
            name: String::from(nametag),
            texture: tex_index,
            clear_or_load: wgpu::LoadOp::Load,
            my_objects: vec![this_object],
            //owned_elements: vec![],
            program_id,
            // drawf,
            drawf_status: Timing::Never,
            // updatef,
            updatef_status: Timing::Never,
            // inputf
        }
    }

    /// Adds a `RenderObject` to `my_objects` and returns it's index.
    pub fn add_renderobj(&mut self, obj: RenderObject) -> usize {
        self.my_objects.push(obj);
        self.my_objects.len()-1
    }

    pub fn set_update_timing(&mut self, timing: Timing) {
        match timing {
            Timing::ASAP => {}
            Timing::Framerate { .. } => {}
            Timing::SpecificTime { .. } => {
                // spawn a timer thread
            }
            Timing::Never => {}
        };

        self.updatef_status = timing;

    }


    pub fn set_render_timing(&mut self, state: &mut State, timing: Timing) {
        match &timing {
            Timing::ASAP => {}
            Timing::Framerate { last_rendered_at, desired_framerate } => {}
            Timing::SpecificTime { .. } => {
                // spawn a timer thread
            }
            Timing::Never => {}
        };

        self.drawf_status = timing;

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
            }
    }


    pub fn should_call_updatef(&mut self) -> bool {
        match &self.updatef_status {
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

    // /// Set the `CallStatus` directly.
    // pub fn set_draw_status(&mut self, rs: CallStatus) {
    //     self.drawf_status = rs;
    // }
    //
    // /// Get a mutable reference to the draw function's `CallStatus` (useful for std::mem:swap)
    // pub fn mut_draw_status(&mut self) -> &mut CallStatus {
    //     &mut self.drawf_status
    // }
    //
    // /// Get an immutable reference to the draw function's `CallStatus` (useful for analysis).
    // pub fn ref_draw_status(&self) -> &CallStatus {
    //     &self.drawf_status
    // }
}
