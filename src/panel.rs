use crate::programhook::*;
use crate::rect::*;
use crate::renderobj::RenderObject;
use crate::texturerenderer::{TextureIndex, TextureRenderer};
use crate::timing::{CallStatus, Timing};
use crate::transform2d::Transform2D;
use crate::wgpustate::State;
use winit::event::WindowEvent;

/// The logical side of the Panel, containing position data and rendering state.
/// * Note: a Panel's RenderState/Object is only it's Panel canvas texture.
pub struct Panel {
    /// The Rect of the Panel on the screen.
    /// Optionally, use this to check if the mouse is in this `Panel`, and direct `WindowEvents` to it.
    world_rect: WorldRectangle,

    /// The `TextureRenderer` index on the State's `texture_renderers` list.
    renderer_id: usize,
}

impl Panel {
    /// Creates a new panel that can be attatched to a TextureRenderer
    pub fn new() -> Self {
        Self {
            world_rect: WorldRectangle {
                pos: WorldPoint::new(0f32, 0f32),
                width: 0.0,
                height: 0.0,
            },
            renderer_id: 0,
        }
    }
}

impl ProgramHook for Panel {
    fn hook_renderer(&mut self, renderer_id: usize) {
        self.renderer_id = renderer_id;
    }

    fn render(
        &self,
        renderer: &mut TextureRenderer,
        _state: &mut State,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        // create renderpass
        let view = renderer.get_textureview(_state).unwrap();
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        let obj = &renderer.this_object;
        let my_model = &_state.models[obj.model];
        render_pass.set_pipeline(&_state.render_pipelines[obj.pipeline]);
        render_pass.set_bind_group(0, &_state.bind_groups[obj.bind_group], &[]);
        render_pass.set_vertex_buffer(0, my_model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(my_model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..my_model.num_indices, 0, 0..1);

        // render any subprograms to thier own textures if they are ready (check Timing)

        // present those textures by rendering them

        // for owned_renderer in renderer.owned_elements {
        //
        // }

    }

    fn update(&mut self, renderer: &mut TextureRenderer, _state: &mut State) {
        let obj = &renderer.this_object;
        println!("update");
        _state.queue.write_buffer(
            &_state.models[obj.model].offset_buffer,
            0,
            bytemuck::cast_slice(&[obj.position]),
        );

    }

    fn input(&mut self, renderer: &mut TextureRenderer, _state: &mut State, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {}
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::ReceivedCharacter(_) => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
        }
    }
}
