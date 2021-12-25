use panels::programhook::ProgramHook;
use panels::texturerenderer::TextureRenderer;
use panels::wgpustate::State;
use panels::wgpu::RenderPass;
use panels::appmgmt::EventLoopAction;
use panels::event::WindowEvent;
use panels::timing::Timing;
use panels::renderobj::RenderObject;
use panels::dpi::PhysicalPosition;
use panels::transform2d::Transform2D;
use panels::rect::{WorldPoint, ScreenPoint};
use std::time::Instant;

/// The renderer for the Surface texture, and manager of `Panels`.
pub struct SurfaceManager {
    panel_renderobjs: Vec<RenderObject>,
    lastpos: WorldPoint,
    renderer_id: usize,
}

impl SurfaceManager {
    /// Creates a new Surface renderer.
    pub fn new(panel_renderobjs: Vec<RenderObject>) -> Self {
        Self {
            panel_renderobjs,
            lastpos: WorldPoint::new(0.0,0.0,0.0),
            renderer_id: 0
        }
    }

    pub fn move_panel(&self, state: &State, panel_id: usize, worldpoint: WorldPoint) {
        let obj = &self.panel_renderobjs[panel_id];
        state.queue.write_buffer(
            &state.models[obj.model].offset_buffer,
            0,
            panels::bytemuck::cast_slice(&[worldpoint]),
        );
    }
}

impl ProgramHook for SurfaceManager {
    
    type Message = ();
    
    fn init(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        renderer.set_update_timing(Timing::Framerate { last_rendered_at: Instant::now(), desired_framerate: 60.0 });
        renderer.set_render_timing(state, Timing::Framerate { last_rendered_at: Instant::now(), desired_framerate: 60.0 });
        renderer.set_clear(panels::wgpu::LoadOp::Clear(panels::wgpu::Color{
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0
        }));

    }

    fn hook_renderer(&mut self, renderer_id: usize) {
       self.renderer_id = renderer_id;
    }

    fn stop_program(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        println!("Stopping Surface manager");
    }

    fn input(&mut self, renderer: &mut TextureRenderer, state: &mut State, event: &WindowEvent) -> EventLoopAction<Self::Message> {
        match event {
            WindowEvent::CursorMoved { device_id, position, modifiers } => {
                // println!("moving to [x: {}/{} | y: {}/{}]",position.x,state.size.width,position.y,state.size.height);
                self.lastpos = WorldPoint::from_mouse(&state.size, position);
                EventLoopAction::None
            }
            _ => EventLoopAction::None
        }
    }

    fn update(&mut self, renderer: &mut TextureRenderer, state: &mut State) -> EventLoopAction<Self::Message> {
        self.move_panel(state, 0usize, self.lastpos*WorldPoint::new(1.0,-1.0,1.0));
        EventLoopAction::None
    }

    fn render<'a>(&self, renderer: &mut TextureRenderer, state: &'a mut State, render_pass: &mut RenderPass<'a>) {
        // render all panels
        for obj in &self.panel_renderobjs {
            obj.render_this(state, render_pass);
        }
    }
}