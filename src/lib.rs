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
mod transform2d;
mod rect;
mod panel;
mod util;
mod wgpustate;
mod resourcebytes;
mod texture;
mod renderobj;
mod modelbuffers;
mod renderable;
use wgpu::SurfaceError;
use wgpustate::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::renderobj::{Position, RenderObject};
use crate::transform2d::Transform2D;

pub fn start() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = pollster::block_on(wgpustate::State::new(&window));

    state.objects.push(RenderObject{
        position: Transform2D::new(0.0,0.0,0.0),
        pipeline: 0,
        bind_group: 0,
        model: 0
    });
    
    event_loop.run(move |event, _, control_flow| match event {

        Event::RedrawRequested(_) => {
            // state.update();
            match state.render() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("ERROR: {:?}",e);
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
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
