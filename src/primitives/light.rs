use crate::render_gl::data::u2_u10_u10_u10_rev_float;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub mod consts {
    use crate::primitives::light::Color;

    pub const RED: Color = Color::new(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn new_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Color {
            r: other.0,
            g: other.1,
            b: other.2,
            a: other.3,
        }
    }
}

impl From<&[f32]> for Color {
    fn from(other: &[f32]) -> Self {
        Color {
            r: other[0],
            g: other[1],
            b: other[2],
            a: other[3],
        }
    }
}

impl From<u2_u10_u10_u10_rev_float> for Color {
    fn from(other: u2_u10_u10_u10_rev_float) -> Self {
        Color {
            r: other.inner.x(),
            g: other.inner.y(),
            b: other.inner.z(),
            a: other.inner.w(),
        }
    }
}
