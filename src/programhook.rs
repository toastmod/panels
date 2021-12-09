use winit::event::WindowEvent;
use crate::appmgmt::EventLoopAction;
use crate::texturerenderer::TextureRenderer;
use crate::timing::{CallStatus, Timing};
use crate::wgpustate::State;

/// A struct that represents an applet that can be rendered to a texture.
pub trait ProgramHook {

    /// Initialize the program,
    /// this is where you would set the inital `Timing` for the update and render functions.
    /// This will be called when it has been loaded into memory and hooked to a `TextureRenderer`.
    fn init(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        renderer.set_update_timing(Timing::Never);
        renderer.set_render_timing(state, Timing::Never);
    }

    /// Sets the renderer hook/ID for this program.
    fn hook_renderer(&mut self, renderer_id: usize) {
        println!("[WARN] ProgramHook has no hook_renderer() function.");
    }

    /// The program is passed a reference to a `RenderPass` targetting it's `TextureRenderer` texture.
    /// Here you can render resources stored in the `State`.
    fn render<'a>(&self, renderer: &mut TextureRenderer, state: &'a mut State, render_pass: &mut wgpu::RenderPass<'a>) {}

    /// This is where the program can update it's data at a rate set by the `Timing` of the `CallStatus`.
    /// You can change the renderer hook, or do pretty much anything here.
    fn update(&mut self, renderer: &mut TextureRenderer, state: &mut State) -> EventLoopAction {
        EventLoopAction::None
    }

    /// The program recieves input from the `AppConductor`.
    /// You can change the renderer hook, or do pretty much anything here.
    fn input(&mut self, renderer: &mut TextureRenderer, state: &mut State, event: &WindowEvent) -> EventLoopAction {
        match event {
            WindowEvent::CloseRequested => EventLoopAction::REQUEST_CLOSE,
            _ => EventLoopAction::None
        }
    }

    /// This function should prepare the program to stop.
    fn stop_program(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        panic!("Empty ProgramHook! (called stop_program)")
    }

    // fn on_redraw_request

    // fn on_main_events_cleared

    // fn on_close(&mut self, renderer &mut TextureRenderer, state: &mut State, event: &WindowEvent) {
    //     panic!("Empty ProgramHook! (called on_close)")
    // }

}