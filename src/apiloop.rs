use crate::programhook::ProgramHook;
use crate::renderablestate::RenderableState;
use crate::rendererinit::ApplicationMut;
use crate::texturerenderer::TextureRenderer;
use crate::timing::{CallStatus, Timing};
use crate::util::fps_to_dur;
use crate::wgpustate::State;

impl RenderableState for State {

    type ErrType = wgpu::SurfaceError;
}