use nalgebra::{Matrix4, Rotation3, Translation3, Vector3};

use crate::primitives::spatial::{Location, Orientation};

pub trait Model {
    fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>);
}

pub struct Spatial {
    pub(crate) location: Location,
    pub(crate) orientation: Orientation,
    pub(crate) scale: f32,
}

impl Spatial {
    pub(crate) fn new(location: Location, orientation: Orientation, scale: f32) -> Self {
        Self {
            location,
            orientation,
            scale,
        }
    }
}

impl Model for Spatial {
    fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>) {
        let rotation = Rotation3::from_euler_angles(
            // TODO: for some reason these make more sense when roll is pitch,
            // pitch is yaw, and yaw is roll. Should probably investigate why.
            self.orientation.pitch,
            self.orientation.yaw,
            self.orientation.roll,
        )
            .to_homogeneous();


        let translation = Translation3::from(Vector3::new(
            self.location.x,
            self.location.y,
            self.location.z,
        ))
            .to_homogeneous();

        return (self.scale, translation, rotation);
    }
}