
use bytemuck::*;
#[repr(C)]
#[derive(Debug,Copy,Clone,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Transform2D {
    pub(crate) pos: [f32; 3]
}

/// Transform vs WorldPosition:
/// The difference is that Transform2D represents the Scale, Rotation, and Trasformation through a matrix.
impl Transform2D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: [x,y,z]
        }
    }
}