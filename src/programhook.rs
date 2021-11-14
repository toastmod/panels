use winit::event::WindowEvent;
use crate::texturerenderer::TextureRenderer;
use crate::timing::{CallStatus, Timing};
use crate::wgpustate::State;



/// A struct that represents an applet that can be rendered to a texture.
pub trait ProgramHook {

    /// Initialize the program,
    /// this is where you would set the inital `Timing` for the update and render functions.
    /// This will be called when it has been loaded into memory and hooked to a `TextureRenderer`.
    fn init(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        panic!("Can't initialize empty ProgramHook!")
    }

    /// Sets the renderer hook/ID for this program.
    fn hook_renderer(&mut self, renderer_id: usize) {
        panic!("Cannot hook empty ProgramHook!")
    }

    /// This is where the program takes control of the encoder and can render to it's texture.
    fn render(&self, renderer: &mut TextureRenderer, state: &mut State, encoder: &mut wgpu::CommandEncoder) {
        panic!("Empty ProgramHook! (called render)")
    }

    /// This is where the program can update it's data at a rate set by the `Timing` of the `CallStatus`.
    /// You can change the renderer hook, or do pretty much anything here.
    fn update(&mut self, renderer: &mut TextureRenderer, state: &mut State) {
        panic!("Empty ProgramHook! (called update)")
    }

    /// You can change the renderer hook, or do pretty much anything here.
    fn input(&mut self, renderer: &mut TextureRenderer, state: &mut State, event: &WindowEvent) {
        panic!("Empty ProgramHook! (called input)")
    }

}