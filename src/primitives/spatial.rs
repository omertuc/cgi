use crate::render_gl::data::f32_f32_f32_f32;
use nalgebra::{Vector3, Vector4};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Location {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Orientation {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Orientation {
    pub fn default() -> Self {
        Orientation {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl From<(f32, f32, f32)> for Location {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Location {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl Into<Vector4<f32>> for Location {
    fn into(self) -> Vector4<f32> {
        Vector4::new(self.x, self.y, self.z, 1.0)
    }
}

impl From<f32_f32_f32_f32> for Location {
    fn from(tuple: f32_f32_f32_f32) -> Self {
        Location {
            x: tuple.d0,
            y: tuple.d1,
            z: tuple.d2,
        }
    }
}

impl From<Location> for Vector3<f32> {
    fn from(f: Location) -> Self {
        Vector3::<f32>::new(f.x, f.y, f.z)
    }
}
