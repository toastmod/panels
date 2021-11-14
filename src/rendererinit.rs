use crate::programhook::ProgramHook;
use crate::texturerenderer::TextureRenderer;
use crate::wgpustate::State;

pub type RendererInit = (State, Vec<TextureRenderer>, Vec<Box<dyn ProgramHook>>);

pub type ApplicationMut<'a> = (&'a mut State, &'a mut Vec<TextureRenderer>, &'a mut Vec<Box<dyn ProgramHook>>);
