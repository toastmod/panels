use winit::event::{Event, WindowEvent};
use crate::create_program_and_renderer;
use crate::programhook::ProgramHook;
use crate::proxyevents::ProxyEvent;
use crate::texturerenderer::{TextureIndex, TextureRenderer};
use crate::wgpustate::State;

pub enum EventLoopAction {
    None,

    /// Skips the next frame.
    SKIP_FRAME,

    /// Request to close the application.
    REQUEST_CLOSE,

}


pub enum CloseReqResponse {
    /// Close the program.
    ACCEPT,

    /// Don't close the prorgram.
    DENY
}

/// Programming for major events in the application.
pub trait AppConductor {

    /// Where you would initialize your programs and renderers, taking note of what program is at what location, etc.
    /// You can also set the initial EventLoop framerate with  `EventLoopAction::SET_FPS()`
    fn init_app(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook>>) -> EventLoopAction {
        panic!("Empty Conductor!")
    }

    /// The flow of `WindowEvent` traffic to each program, and poitentially the flow of output `EventLoopAction`s.
    fn event_mgmt(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook>>, event: WindowEvent) -> EventLoopAction {
        panic!("Empty Conductor!")
    }

    /// A program has requested to close the whole program.
    /// Preprare to close or choose not to close.
    fn on_close_request(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook>>) -> CloseReqResponse {
        panic!("Empty Conductor!")
    }

}

