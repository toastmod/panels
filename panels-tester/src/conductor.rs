// example panels app

use panels::appmgmt::{AppConductor, CloseReqResponse, EventLoopAction};
use panels::event::WindowEvent;
use panels::programhook::ProgramHook;
use panels::texturerenderer::{TextureIndex, TextureRenderer};
use panels::wgpustate::State;

use crate::panel::Panel;
use crate::surface::SurfaceManager;
use panels::renderobj::RenderObject;

pub struct PanelsApp {
    pub window_focused: bool,
    pub focused_panel: usize
}

impl AppConductor for PanelsApp {

    type Message = ();

    fn init_app(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook<Message = Self::Message>>>) -> EventLoopAction<Self::Message> {
        // now we can add renderers and programs to our application
        // this renderer will render to the surface and render each panel
        panels::create_program_and_renderer("manager", (state, renderers, programs), TextureIndex::Surface, Box::new(SurfaceManager::new(vec![
            RenderObject::new_placeholder_rect()
        ])));
        state.set_fps(None);
        EventLoopAction::None
    }

    fn event_mgmt(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook<Message = Self::Message>>>, event: WindowEvent) -> EventLoopAction<Self::Message> {
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

            e => {
                for r in renderers {
                    programs[r.program_id.unwrap().clone()].input(r, state, &e);
                }
                EventLoopAction::None
            }
        }
    }

    fn on_message(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook<Message = Self::Message>>>, msg: Self::Message) {
        ()
    }

    fn on_close_request(&mut self, renderers: &mut Vec<TextureRenderer>, state: &mut State, programs: &mut Vec<Box<dyn ProgramHook<Message = Self::Message>>>) -> CloseReqResponse {
        CloseReqResponse::ACCEPT
    }
}
