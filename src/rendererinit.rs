use crate::programhook::ProgramHook;
use crate::texturerenderer::TextureRenderer;
use crate::wgpustate::State;

pub type RendererInit<T> = (State, Vec<TextureRenderer>, Vec<Box<dyn ProgramHook<Message = T>>>);

pub type ApplicationMut<'a, T> = (&'a mut State, &'a mut Vec<TextureRenderer>, &'a mut Vec<Box<dyn ProgramHook<Message = T>>>);
