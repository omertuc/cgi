use nalgebra::Matrix4;

use crate::models::world_model::{Model, Spatial};
use crate::primitives::spatial::{Location, Orientation};
use crate::primitives::spotlight::{spot_radius_to_cube_scale, Spotlight};

pub struct GameLight {
    pub(crate) spotlight: Spotlight,
    pub(crate) angle: f32,
    pub(crate) center: Location,
    pub(crate) location: Location,
    spin_radius: f32,
    pub(crate) spin_speed: f32,
    pub(crate) x_speed: f32,
    pub(crate) y_speed: f32,
    pub(crate) z_speed: f32,
}

impl Model for GameLight {
    fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>) {
        Spatial {
            location: self.location,
            orientation: Orientation {
                roll: self.angle,
                pitch: 0.0,
                yaw: 0.0,
            },
            scale: spot_radius_to_cube_scale(self.spotlight.spot_radius),
        }
        .model()
    }
}

impl GameLight {
    fn location_from_angle(center: Location, angle: f32, spin_radius: f32) -> Location {
        center
            + Location {
                x: center.x + spin_radius * angle.cos(),
                y: center.y + spin_radius * angle.sin(),
                z: center.z,
            }
    }

    fn refresh_location(&mut self) {
        self.location = Self::location_from_angle(self.center, self.angle, self.spin_radius);
    }

    pub(crate) fn new(angle: f32, center: Location, spin_radius: f32, spin_speed: f32, spotlight: Spotlight) -> Self {
        Self {
            spotlight,
            angle,
            center,
            spin_radius,
            spin_speed,
            x_speed: 0.0,
            y_speed: 0.0,
            location: Self::location_from_angle(center, angle, spin_radius),
            z_speed: 0.0,
        }
    }

    pub(crate) fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
        self.refresh_location()
    }

    pub(crate) fn set_location(&mut self, center: Location) {
        self.center = center;
        self.refresh_location()
    }
}
