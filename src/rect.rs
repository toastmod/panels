use winit::dpi::PhysicalSize;
use bytemuck::*;
use crate::dpi::PhysicalPosition;

/// A point in the worldspace, in world coordinates.

#[repr(C)]
#[derive(Debug,Copy,Clone,bytemuck::Pod,bytemuck::Zeroable)]
pub struct WorldPoint {
    x: f32,
    y: f32,
    z: f32,
}

impl WorldPoint {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn from_screen_point(screensize: &PhysicalSize<u32>, p: ScreenPoint) -> Self {
        Self {
            x: (((p.x as f32 /screensize.width as f32 )*2.0)-1.0),
            y: (((p.y as f32 /screensize.height as f32)*2.0)-1.0),
            z: 0.0
        }
    }

    /// Equivalent to `WorldPoint::from_screen_point(&screensize, ScreenPoint::from_mouse(&mouse))`
    pub fn from_mouse(screensize: &PhysicalSize<u32>, mouse: &PhysicalPosition<f64>) -> Self {
        WorldPoint::from_screen_point(screensize, ScreenPoint::from_mouse(mouse))
    }

}

impl std::ops::Mul for WorldPoint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        WorldPoint {
            x: self.x*rhs.x,
            y: self.y*rhs.y,
            z: self.z*rhs.z
        }
    }
}

/// A Rectangle meausred in WGPU world units.
pub struct WorldRectangle {
    pub pos: WorldPoint,
    pub width: f32,
    pub height: f32,
}

impl WorldRectangle {

    pub fn pos_in(&self, point: &WorldPoint) -> WorldPoint {
        WorldPoint {
            x: point.x-self.pos.x,
            y: point.y-self.pos.y,
            z: 0.0
        }
    }

    pub fn from_screen_rect(screensize: &PhysicalSize<u32>, sr: &ScreenRectangle) -> Self {
        Self {
            pos: WorldPoint::new(sr.pos.x as f32 /screensize.width as f32 , sr.pos.y as f32 / screensize.height as f32, 0.0),
            width: (sr.width as f32 /screensize.width as f32),
            height: sr.height as f32 /screensize.height as f32
        }
    }
}

/// A point on the screen, in pixels.
pub struct ScreenPoint {
    x: u32,
    y: u32,
}

impl ScreenPoint {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn from_mouse(point: &PhysicalPosition<f64>) -> Self {
        Self {
            x: point.x as u32,
            y: point.y as u32
        }
    }

    pub fn from_world_point(screensize: &PhysicalSize<u32>, p: &WorldPoint) -> Self {
        Self {
            x: (p.x as u32 * screensize.width),
            y: p.y as u32 * screensize.height
        }
    }
}

/// A Rectangle meausred in screen pixels
pub struct ScreenRectangle {
    pub pos: ScreenPoint,
    pub width: u32,
    pub height: u32,
}

impl ScreenRectangle {

    pub fn pos_in(&self, point: &ScreenPoint) -> ScreenPoint {
        ScreenPoint {
            x: point.x-self.pos.x,
            y: point.y-self.pos.y
        }
    }

    pub fn from_world_rect(screensize: &PhysicalSize<u32>, wr: &WorldRectangle) -> Self {
        Self {
            pos: ScreenPoint::from_world_point(screensize, &wr.pos),
            width: wr.width as u32 * screensize.width,
            height: wr.height as u32 * screensize.height
        }
    }
    
    pub fn from_size(width: u32, height: u32) -> Self {
        Self {
            pos: ScreenPoint::new(0,0),
            width,
            height
        }
    }
}