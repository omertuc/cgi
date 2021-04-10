use nalgebra::{Matrix4, Perspective3};

pub fn perspective(aspect: f32) -> Matrix4<f32> {
    Perspective3::new(aspect, std::f32::consts::FRAC_PI_2, 0.1, 100000.0).to_homogeneous()
}
