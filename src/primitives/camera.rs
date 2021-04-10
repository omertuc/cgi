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
}
