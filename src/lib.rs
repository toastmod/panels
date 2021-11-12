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
mod rect;
mod renderable;
mod renderobj;
mod resourcebytes;
mod texture;
mod texturerenderer;
mod bindgroupreg;
mod transform2d;
mod util;
mod wgpustate;
use crate::renderobj::{Position, RenderObject};
use crate::transform2d::Transform2D;
use wgpu::SurfaceError;
use wgpustate::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn start() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = pollster::block_on(wgpustate::State::new(&window));

    state.objects.push(RenderObject {
        position: Transform2D::new(0.0, 0.0, 0.0),
        pipeline: 0,
        bind_group: 0,
        model: 0,
        uniforms: vec![]
    });

    event_loop.run(move |event, _, control_flow|
        match event {
            Event::RedrawRequested(_) => {
                // state.update();
                match state.render() {
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
