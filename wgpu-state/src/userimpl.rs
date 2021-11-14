use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;

/// An interface between the Winit window's event loop and the WGPU state.
/// This is where the user will define thier own WGPU code having to do with their program.
trait WGPUState {
    /// Resize the elements accordingly, and (optionally) resize the WGPU Surface with `State::resize(..)`
    fn resize(&mut self, new_size: PhysicalSize<u32>);
    fn input(&mut self, event: &WindowEvent);
    fn update(&mut self);
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}