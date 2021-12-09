use std::time::Instant;
use panels::appmgmt::EventLoopAction;
use panels::event::WindowEvent;
use panels::rect::{WorldPoint, WorldRectangle};
use panels::texturerenderer::TextureRenderer;
use panels::timing::Timing;
use panels::wgpustate::State;
use panels::wgpu;
use panels::bytemuck;
use panels::programhook::ProgramHook;

/// The logical side of the Panel, containing position data and rendering state.
/// * Note: a Panel's RenderState/Object is only it's Panel canvas texture.
pub struct Panel {
    /// The Rect of the Panel on the screen.
    /// Optionally, use this to check if the mouse is in this `Panel`, and direct `WindowEvents` to it.
    world_rect: WorldRectangle,

    /// The `TextureRenderer` index on the State's `texture_renderers` list.
    renderer_id: usize,

    /// The bindgroup locations for animations on this renderer
    animation_bindgroups: Vec<usize>
}

impl Panel {
    /// Creates a new panel that can be attatched to a TextureRenderer
    pub fn new() -> Self {
        Self {
            world_rect: WorldRectangle {
                pos: WorldPoint::new(0f32, 0f32, 0f32),
                width: 0.0,
                height: 0.0,
            },
            renderer_id: 0,
            animation_bindgroups: vec![]
        }
    }
}

impl ProgramHook for Panel {
    fn init(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        renderer.set_update_timing(Timing::Framerate { last_rendered_at: Instant::now(), desired_framerate: 30f64 });
        renderer.set_render_timing(state, Timing::Framerate { last_rendered_at: Instant::now(), desired_framerate: 30f64 });
        //renderer.my_objects.push()
    }

    fn hook_renderer(&mut self, renderer_id: usize) {
        self.renderer_id = renderer_id;
    }

    fn render<'a>(
        &self,
        renderer: &mut TextureRenderer,
        _state: &'a mut State,
        render_pass: &mut wgpu::RenderPass<'a>
    ) {

        for obj in &renderer.my_objects {
            obj.render_this(_state, render_pass);
        }

        // render any subprograms to thier own textures if they are ready (check Timing)

        // present those textures by rendering them

        // for owned_renderer in renderer.owned_elements {
        //
        // }

    }

    fn update(&mut self, renderer: &mut TextureRenderer, _state: &mut State) -> EventLoopAction {
        // let obj = &renderer.my_objects[0];
        // _state.queue.write_buffer(
        //     &_state.models[obj.model].offset_buffer,
        //     0,
        //     bytemuck::cast_slice(&[obj.position]),
        // );
        EventLoopAction::None
    }

    fn input(&mut self, renderer: &mut TextureRenderer, _state: &mut State, event: &WindowEvent) -> EventLoopAction {
        EventLoopAction::None
    }

    fn stop_program(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        ()
    }
}
