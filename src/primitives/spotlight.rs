use crate::models::cube::Cube;
use crate::primitives::light::Color;
use crate::primitives::spatial::{Location, Orientation};

pub struct Spotlight {
    pub(crate) cube: Cube,
    pub(crate) color: Color,
    pub(crate) location: Location,
    pub(crate) spot_radius: f32,
}

fn spot_radius_to_cube_scale(spot_radius: f32) -> f32 {
    return spot_radius / 10.0;
}

impl Spotlight {
    pub(crate) fn new(initial_location: Location, color: Color, spot_radius: f32) -> Self {
        Self {
            cube: Cube::new(
                initial_location,
                Orientation::default(),
                spot_radius_to_cube_scale(spot_radius),
                Default::default(),
            ),
            location: initial_location,
            spot_radius,
            color,
        }
    }

    #[allow(dead_code)]
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
