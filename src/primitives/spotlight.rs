use crate::models::cube::Cube;
use crate::primitives::light::Color;

pub struct Spotlight {
    pub(crate) cube: Cube,
    pub(crate) color: Color,
    pub(crate) spot_radius: f32,
}

pub fn spot_radius_to_cube_scale(spot_radius: f32) -> f32 {
    spot_radius / 50.0
}

impl Spotlight {
    pub(crate) fn new(color: Color, spot_radius: f32) -> Self {
        Self {
            cube: Cube::new(color),
            spot_radius,
            color,
        }
    }

    #[allow(dead_code)]
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
