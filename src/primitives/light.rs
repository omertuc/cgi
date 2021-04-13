use crate::render_gl::data::u2_u10_u10_u10_rev_float;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
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
