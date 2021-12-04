// example panels app

use panels::appmgmt::{AppConductor, CloseReqResponse, EventLoopAction};
use panels::event::WindowEvent;
use panels::programhook::ProgramHook;
use panels::texturerenderer::TextureRenderer;
use panels::wgpustate::State;

use crate::panel::Panel;

pub struct PanelsApp {
    pub window_focused: bool,
    pub focused_panel: usize
}

impl AppConductor for PanelsApp {
    fn init_app(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook>>) -> EventLoopAction {
        // now we can add renderers and programs to our application
        panels::create_program_and_renderer("master", (state, renderers, programs), Box::new(Panel::new()));
        state.set_fps(None);
        EventLoopAction::None
    }

    fn event_mgmt(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook>>, event: WindowEvent) -> EventLoopAction {
        match event {
            WindowEvent::Resized(new_size) => {
                state.resize(new_size);
                EventLoopAction::SKIP_FRAME
            }

            WindowEvent::Moved(_) => {
                EventLoopAction::SKIP_FRAME
            }

            WindowEvent::CloseRequested => {
                EventLoopAction::REQUEST_CLOSE
            }

            _ => EventLoopAction::None
        }
    }

    fn on_close_request(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook>>) -> CloseReqResponse {
        CloseReqResponse::ACCEPT
    }
}
