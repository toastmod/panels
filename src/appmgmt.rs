use winit::event::{Event, WindowEvent};
use crate::create_program_and_renderer;
use crate::programhook::ProgramHook;
use crate::proxyevents::ProxyEvent;
use crate::texturerenderer::{TextureIndex, TextureRenderer};
use crate::wgpustate::State;

pub enum EventLoopAction<T> {
    None,

    /// Skips the current event loop frame's `RedrawRequested` routine.
    /// This may delay rendering for any program that is ready to draw.
    SKIP_FRAME,

    // Render the program that called this action.
    //TODO: RENDER_ME



    /// Request to close the application.
    REQUEST_CLOSE,

    /// A User-defined message to the global conductor.
    MSG(T)
}


pub enum CloseReqResponse {
    /// Close the program.
    ACCEPT,

    /// Don't close the prorgram.
    DENY
}

/// Programming for major events in the application.
pub trait AppConductor {

    type Message;

    /// Where you would initialize your programs and renderers, taking note of what program is at what location, etc.
    /// You can also set the initial EventLoop framerate with  `EventLoopAction::SET_FPS()`
    fn init_app(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook<Message = Self::Message>>>) -> EventLoopAction<Self::Message> {
        state.set_fps(None);
        EventLoopAction::None
    }

    /// The flow of `WindowEvent` traffic to each program, and poitentially the flow of output `EventLoopAction`s.
    fn event_mgmt(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook<Message = Self::Message>>>, event: WindowEvent) -> EventLoopAction<Self::Message> {
        match event {
            WindowEvent::CloseRequested => {
                EventLoopAction::REQUEST_CLOSE
            }
            _ => EventLoopAction::None
        }
    }

    /// A program has requested to close the whole program.
    /// Preprare to close or choose not to close.
    fn on_close_request(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook<Message = Self::Message>>>) -> CloseReqResponse {
        panic!("Empty Conductor!")
    }

}

