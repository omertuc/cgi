use nalgebra::{Matrix4, Rotation3, Translation3, Vector3, Unit, Matrix3};

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

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let yaw = Rotation3::from_axis_angle(&Vector3::y_axis(), -self.orientation.yaw).to_homogeneous();
        let pitch = Rotation3::from_axis_angle(&Vector3::x_axis(), -self.orientation.pitch).to_homogeneous();
        let roll = Rotation3::from_axis_angle(&Vector3::z_axis(), -self.orientation.roll).to_homogeneous();

        yaw * pitch * roll
    }

    pub fn rotation_matrix(&self) -> Matrix3<f32> {
        let yaw = Rotation3::from_axis_angle(&Vector3::y_axis(), self.orientation.yaw).matrix().clone();
        let pitch = Rotation3::from_axis_angle(&Vector3::x_axis(), self.orientation.pitch).matrix().clone();
        let roll = Rotation3::from_axis_angle(&Vector3::z_axis(), self.orientation.roll).matrix().clone();

        yaw * pitch * roll
    }

    pub fn translation_matrix(&self) -> Matrix4<f32> {
        Translation3::from(Vector3::new(
            -self.location.x,
            -self.location.y,
            -self.location.z,
        ))
            .to_homogeneous()
    }

    pub fn view(&self) -> (Matrix4<f32>, Matrix4<f32>, Vector3<f32>) {
        (self.view_matrix(), self.translation_matrix(), self.location.into())
    }
}
