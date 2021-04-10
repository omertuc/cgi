use nalgebra::{Matrix4, Rotation3, Translation3, Vector3};

use crate::primitives::spatial::{Location, Orientation};

#[derive(Debug)]
pub(crate) struct Camera {
    pub location: Location,
    pub orientation: Orientation,
}

impl Camera {
    pub fn normalize(&self) -> Camera {
        Camera {
            location: self.location,
            orientation: Orientation {
                pitch: self.orientation.pitch % (2f32 * std::f32::consts::PI),
                roll: self.orientation.roll % (2f32 * std::f32::consts::PI),
                yaw: self.orientation.yaw % (2f32 * std::f32::consts::PI),
            },
        }
    }

    pub fn view(&self) -> Matrix4<f32> {
        let rotation = Rotation3::from_euler_angles(
            // TODO: for some reason these make more sense when roll is pitch,
            // pitch is yaw, and yaw is roll. Should probably investigate why.
            -self.orientation.pitch,
            -self.orientation.yaw,
            -self.orientation.roll,
        )
        .to_homogeneous();

        let translation = Translation3::from(Vector3::new(
            -self.location.x,
            -self.location.y,
            -self.location.z,
        ))
        .to_homogeneous();

        rotation * translation
    }
}
