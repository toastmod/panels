#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// WGPU Render Manager
// by toastmod
// Made by following this tutorial
// https://sotrh.github.io/learn-wgpu

mod camera;
pub mod modelbuffers;
mod panel;
mod panelmgmt;
pub mod rect;
mod renderable;
pub mod renderobj;
pub mod resourcebytes;
pub mod texture;
pub mod texturerenderer;
mod bindgroupreg;
mod transform2d;
pub mod util;
pub mod timing;
pub mod wgpustate;
pub mod programhook;
mod renderablestate;
mod rendererinit;
mod apiloop;
mod programreg;
mod proxyevents;
pub mod appmgmt;
mod schedule;

use std::borrow::Borrow;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use crate::renderobj::{Position, RenderObject};
use crate::transform2d::Transform2D;
pub use wgpu;
use wgpustate::*;
pub use winit::*;
use crate::programhook::ProgramHook;
use crate::renderablestate::RenderableState;
use crate::rendererinit::{ApplicationMut, RendererInit};
use crate::texturerenderer::{TextureIndex, TextureRenderer, TextureViewQuery};
use crate::timing::{Timing, CallStatus};
use crate::util::fps_to_dur;
use crate::apiloop::*;
use crate::appmgmt::{AppConductor, CloseReqResponse, EventLoopAction};
use crate::proxyevents::ProxyEvent;
use std::ops::Add;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
pub use bytemuck;

// TODO: create a Texture resource if the Surface's Texture has not been reserved by a renderer.

/// Creates a paired program and renderer, and returns their IDs respectively.
/// The pair is not organized into a struct, incase the uesr wants multiple programs aimed at a renderer or vice versa.
/// Creating an object that keeps track of pairing is up to the user this way.
pub fn create_program_and_renderer(nametag: &str, (state, renderers, programs): ApplicationMut, phook: Box<dyn ProgramHook>) -> (usize, usize) {
    // first we create a ProgramHook object
    // let mut phook = Box::new(Panel::new());

    // ... and create a corresponding TextureRenderer, attatched with the ProgramHook.
    // in this case we are creating a renderer/program for the main Surface.
    let mut my_renderer = TextureRenderer::new(nametag,None, TextureIndex::Surface);

    // get the new IDs for the program and renderer
    let program_id = programs.len();
    let renderer_id = renderers.len();

    // push the objects into memory at thier ID locations
    programs.push(phook);
    renderers.push(my_renderer);

    // hook the renderer and program
    programs[program_id].hook_renderer(renderer_id);
    renderers[renderer_id].hook_program(program_id);

    programs[program_id].init(&mut renderers[renderer_id], state);

    (program_id, renderer_id)
}


fn redraw_if_ready(
    renderers: &mut Vec<TextureRenderer>,
    state: &mut State,
    programs: &mut Vec<Box<dyn ProgramHook>>,
) {
    let mut encoder: Option<wgpu::CommandEncoder> = None;
    let mut surface_texture: Option<wgpu::SurfaceTexture> = None;

    let mut surface_view: Option<wgpu::TextureView> = None;
    let mut texture_views: Vec<wgpu::TextureView> = vec![];

    let mut surface_accessed = false;

    // note: scope here for renderpass ownership
    {
        for tex_rend in renderers {
            // print!("Renderer: {} | Status:", tex_rend.name);
            if tex_rend.should_call_drawf(false) {

                // initialize encoder if needed
                if match encoder {
                    None => true,
                    Some(_) => false
                } {
                    encoder = Some(state
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }));
                }


                // println!(" Rendering!");
                // get the correct TextureView to this renderer's Texture
                let view_ref = match match tex_rend.get_textureview(state){
                    Ok(tv) => tv,
                    Err(e) => {
                        panic!("ERROR: Could not get TextureView for \"{}\"",tex_rend.name)
                    }
                } {
                    TextureViewQuery::RequestSurfaceView => {

                        // TODO: only get surface texture if it hasn't already been loaded

                        surface_texture = Some(match state.surface.get_current_texture() {
                            Ok(s) => s,
                            Err(e) => {
                                panic!("ERROR: Could not get surface texture! ({})", e)
                            }
                        });
                        surface_view = Some(surface_texture.as_ref().unwrap().texture.create_view(&wgpu::TextureViewDescriptor::default()));
                        surface_view.as_ref().unwrap()
                    },
                    TextureViewQuery::View(v) => {
                        let view_id = texture_views.len();
                        texture_views.push(v);
                        &texture_views[view_id]
                    }
                };

                // create a RenderPass based on the TextureRenderer's preference
                let mut render_pass = encoder.as_mut().unwrap().begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: view_ref,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: tex_rend.clear_or_load.clone(),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                // mutate the render_pass according to the program
                programs[tex_rend.program_id.unwrap().clone()].render(
                    tex_rend,
                    state,
                    &mut render_pass,
                );
            } else {
                // println!(" Not rendering!");
            }
        }
    };


    // if the encoder was created, then work was submitted.
    match encoder {
        Some(e) => {
            state.queue.submit(std::iter::once(e.finish()));
        }
        None => {}
    };

    // if the next surface texture was accessed, it needs to be presented.
    match surface_texture {
        Some(st) => {
            // println!("presenting!");
            st.present();
        }
        None => {}
    };

}

pub fn start(mut conductor: Box<dyn AppConductor>) {
    env_logger::init();
    let event_loop: EventLoop<ProxyEvent> = EventLoop::with_user_event();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut proxy = Arc::new(event_loop.create_proxy());

    // A full application needs to start with it's WGPU State, it's TextureRenderers, and it's ProgramHooks
    let (mut state, mut renderers, mut programs): RendererInit = (
        pollster::block_on(wgpustate::State::new(&window)),
        vec![],
        vec![]
    );

    // it is assumed that programs and renderers will be populated here.
    conductor.init_app(&mut renderers, &mut state, &mut programs);

    let mut last_rendered = Instant::now();
    // now to define the event loop
    let mut skip_frame = false;
    event_loop.run(move |event, _, control_flow|{
        match event {

            Event::WindowEvent { window_id, event } =>{
                if window.id() == window.id() {
                    match conductor.event_mgmt(&mut renderers, &mut state, &mut programs, event) {
                        EventLoopAction::None => {}
                        EventLoopAction::SKIP_FRAME => {
                            skip_frame = true;
                        }
                        EventLoopAction::REQUEST_CLOSE => {
                            println!("Closing application...");
                            proxy.send_event(ProxyEvent::CLOSE_REQUEST).unwrap_or(panic!("EventLoopProxy Error! Could not send Close Request!"));
                        }
                    }
                }
            }

            Event::UserEvent(pe) => {
                match pe {
                    ProxyEvent::UPDATE(rend_id) => {
                        let renderer = &mut renderers[rend_id];
                        programs[renderer.program_id.unwrap().clone()].update(renderer, &mut state);
                    }
                    ProxyEvent::CLOSE_REQUEST => {
                        match conductor.on_close_request(&mut renderers, &mut state, &mut programs) {
                            CloseReqResponse::ACCEPT => {
                                *control_flow = ControlFlow::Exit;
                            }
                            CloseReqResponse::DENY => {}
                        }
                    }
                }
            }


            Event::MainEventsCleared => {
                // update
                for renderer in &mut renderers {
                    if renderer.should_call_updatef() {
                        let ela = programs[renderer.program_id.unwrap().clone()].update(renderer, &mut state);
                    }
                }
                // TODO: Use a different EventLoop for Android and iOS
                //  redraw_request is not supported properly on these platforms.
                if !skip_frame{
                    window.request_redraw();
                }
            }

            Event::RedrawRequested(_) => {
                // TOOD: make this actually work
                if !skip_frame {
                    redraw_if_ready(&mut renderers, &mut state, &mut programs);
                    last_rendered = Instant::now();
                }else{
                    skip_frame = false;
                }
            }

            Event::NewEvents(_) => {}
            Event::DeviceEvent { .. } => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::RedrawEventsCleared => {

            }
            Event::LoopDestroyed => {}
        };

        // schedule next frame if there are framerate Timings
        // if a program/renderer's update Timing is not threaded, it will update at the same FPS as the surface this way.
        match state.loop_fps {
            None => {
                *control_flow = ControlFlow::Wait;
            }
            Some(fps) => {
                let fps_ns = fps_to_dur(fps);
                let mut rendertime = last_rendered.add(fps_ns);
                // println!("waiting {:?}", fps_ns);
                *control_flow = ControlFlow::WaitUntil(rendertime);
            }
        }

    }
    );
}
