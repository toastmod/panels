#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// Basic 2D Renderer in WGPU
// by toastmod
// Made by following this tutorial
// https://sotrh.github.io/learn-wgpu

mod camera;
mod modelbuffers;
mod panel;
mod panelmgmt;
mod rect;
mod renderable;
mod renderobj;
mod resourcebytes;
mod texture;
mod texturerenderer;
mod bindgroupreg;
mod transform2d;
mod util;
mod timing;
mod wgpustate;
mod programhook;
mod renderablestate;
mod rendererinit;
mod apiloop;
mod programreg;

use std::borrow::Borrow;
use std::time::Instant;
use crate::renderobj::{Position, RenderObject};
use crate::transform2d::Transform2D;
use wgpu::SurfaceError;
use wgpustate::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::panel::Panel;
use crate::programhook::ProgramHook;
use crate::renderablestate::RenderableState;
use crate::rendererinit::{ApplicationMut, RendererInit};
use crate::texturerenderer::{TextureIndex, TextureRenderer};
use crate::timing::{Timing, CallStatus};
use crate::util::fps_to_dur;
use crate::apiloop::*;

// TODO: create a Texture resource if the Surface's Texture has not been reserved by a renderer.
pub fn create_program_and_renderer((state, renderers, programs): ApplicationMut) {
    // first we create a ProgramHook object
    let mut panel = Box::new(Panel::new());


    // ... and create a corresponding TextureRenderer, attatched with the ProgramHook.
    // in this case we are creating a renderer/program for the main Surface.
    let mut my_renderer = TextureRenderer::new(None, TextureIndex::Surface);

    // get the new IDs for the program and renderer
    let program_id = programs.len();
    let renderer_id = renderers.len();

    // push the objects into memory at thier ID locations
    programs.push(panel);
    renderers.push(my_renderer);

    // hook the renderer and program
    programs[program_id].hook_renderer(renderer_id);
    renderers[renderer_id].hook_program(program_id);

}

pub fn start() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // A full application needs to start with it's WGPU State, it's TextureRenderers, and it's ProgramHooks
    let (mut state, mut renderers, mut programs): RendererInit = (
        pollster::block_on(wgpustate::State::new(&window)),
        vec![],
        vec![]
    );

    // now we can add renderers and programs to our application
    create_program_and_renderer((&mut state, &mut renderers, &mut programs));

    // now to define the event loop
    event_loop.run(move |event, _, control_flow|
        match event {
            Event::RedrawRequested(_) => {
                // use the api_loop we defined ourselves in the event loop
                match state.api_loop(false, &mut renderers, &mut programs) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("ERROR: {:?}", e);
                    }
                }
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    state.resize(new_size.clone())
                }
                WindowEvent::Moved(_) => {}
                WindowEvent::Destroyed => {}
                WindowEvent::DroppedFile(_) => {}
                WindowEvent::HoveredFile(_) => {}
                WindowEvent::HoveredFileCancelled => {}
                WindowEvent::ReceivedCharacter(_) => {}
                WindowEvent::Focused(_) => {}
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
                WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {

                }
            },
            _ => {}
        }
    );
}
