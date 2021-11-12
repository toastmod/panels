use crate::renderobj::RenderObject;
use crate::wgpustate::State;

pub struct TextureRenderer {
    pub texture: usize,
    pub objects: Box<Vec<RenderObject>>,
    pub drawf: Box<Fn(&Vec<RenderObject>, &State, &mut wgpu::RenderPass)>
}

impl TextureRenderer {
    /// Creates a `TextureRenderer` that uses the given routine to draw elements.
    pub fn new(tex_index: usize, drawf: Box<Fn(&Vec<RenderObject>, &State, &mut wgpu::RenderPass)>) -> Self {
        Self {
            texture: tex_index,
            objects: Box::new(vec![]),
            drawf
        }
    }

    pub fn render(&self, state: &State, render_pass: &mut wgpu::RenderPass ) {
        let objref = self.objects.as_ref();
        (self.drawf)(objref, state, render_pass);
        drop(objref);
    }

}