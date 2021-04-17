use nalgebra::{Matrix4, Orthographic3, Perspective3};

pub fn perspective(aspect: f32) -> Matrix4<f32> {
    Perspective3::new(aspect, std::f32::consts::FRAC_PI_2, 0.1, 10000.0).to_homogeneous()
}

#[allow(dead_code)]
pub fn orthographic(aspect: f32) -> Matrix4<f32> {
    let base = 10f32;
    Orthographic3::new(base * aspect, -base * aspect, base, -base, 0.1, 10000.0).to_homogeneous()
}
