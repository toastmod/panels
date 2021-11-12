use crate::rect::*;
use crate::renderobj::RenderObject;
use crate::transform2d::Transform2D;
use crate::wgpustate::State;


struct Panel {
    world_rect: WorldRectangle,
    wgpu_state_obj: usize,
}

impl Panel {
    /// Creates a new panel attatched with the ID of a new renderobject
    fn new_with_renderobject(state: &mut State) -> Self {

        let objid = state.objects.len();

        state.objects.push(RenderObject{
            position: Transform2D::new(0.0,0.0,0.0),
            pipeline: 0,
            bind_group: 0,
            model: 0
        });

        Self {
            world_rect: WorldRectangle {
                pos: WorldPoint::new(0f32,0f32),
                width: 0.0,
                height: 0.0
            },
            wgpu_state_obj: objid
        }

    }
}