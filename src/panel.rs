use crate::rect::*;
use crate::renderobj::{RenderObject, RenderState};
use crate::transform2d::Transform2D;
use crate::wgpustate::State;


/// The logical side of the Panel, containing position data and rendering state.
/// * Note: a Panel's RenderState/Object is only it's Panel canvas texture.
struct Panel {
    world_rect: WorldRectangle,
    render_state: RenderState,
}

impl Panel {
    /// Creates a new panel attatched with the ID of a new renderobject
    fn new_with_renderobject(state: &mut State) -> Self {

        let objid = state.objects.len();

        state.objects.push(RenderObject{
            position: Transform2D::new(0.0,0.0,0.0),
            pipeline: 0,
            bind_group: 0,
            model: 0,
            uniforms: vec![]
        });

        Self {
            world_rect: WorldRectangle {
                pos: WorldPoint::new(0f32,0f32),
                width: 0.0,
                height: 0.0
            },
            render_state: RenderState::Rendering(objid)
        }

    }
}