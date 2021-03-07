use failure;
use gl;

use crate::render_gl::data::f32_f32_f32_f32;
use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;
use nalgebra::{Rotation, Vector3, Vector4};

pub type Location = (f32, f32, f32);

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: data::f32_f32_f32_f32,
    pub clr: data::u2_u10_u10_u10_rev_float,
}

pub struct TrianglesDraw {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl TrianglesDraw {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<TrianglesDraw, failure::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vbo = buffer::ArrayBuffer::new(&gl);
        let vao = buffer::VertexArray::new(&gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        let triangle = TrianglesDraw { program, vbo, vao };

        Ok(triangle)
    }

    pub fn draw(&self, gl: &gl::Gl, vertices: Vec<Vertex>) {
        self.vbo.bind();
        self.vbo.dynamic_draw_data(&vertices);

        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl.DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
        }
    }
}

#[derive(Clone)]
pub struct Triangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
    roll: f32,
    yaw: f32,
    pitch: f32,
    location: Location,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex, location: Location) -> Triangle {
        Triangle {
            a,
            b,
            c,
            roll: 0f32,
            yaw: 0f32,
            pitch: 0f32,
            location,
        }
    }

    pub fn rotated(self) -> Triangle {
        let homogeneous_matrix =
            Rotation::from_euler_angles(self.roll, self.yaw, self.pitch).to_homogeneous();

        let mut cloned = self.clone();

        cloned.a.pos = (&homogeneous_matrix * Vector4::from(self.a.pos)).into();
        cloned.b.pos = (&homogeneous_matrix * Vector4::from(self.b.pos)).into();
        cloned.c.pos = (&homogeneous_matrix * Vector4::from(self.c.pos)).into();
        cloned.roll = 0f32;
        cloned.yaw = 0f32;
        cloned.pitch = 0f32;

        cloned
    }

    pub fn add_roll(self, angle: f32) -> Triangle {
        let mut cloned = self.clone();
        cloned.roll += angle;
        cloned
    }

    pub fn add_pitch(self, angle: f32) -> Triangle {
        let mut cloned = self.clone();
        cloned.pitch += angle;
        cloned
    }

    pub fn add_yaw(self, angle: f32) -> Triangle {
        let mut cloned = self.clone();
        cloned.yaw += angle;
        cloned
    }

    pub fn vertices(self) -> Vec<Vertex> {
        vec![self.a, self.b, self.c]
    }
}

impl From<Vector4<f32>> for f32_f32_f32_f32 {
    fn from(vector: Vector4<f32>) -> Self {
        (vector.x, vector.y, vector.z, vector.w).into()
    }
}

impl From<f32_f32_f32_f32> for nalgebra::Vector4<f32> {
    fn from(f: f32_f32_f32_f32) -> Self {
        Vector4::new(f.d0, f.d1, f.d2, f.d3)
    }
}

impl From<f32_f32_f32_f32> for nalgebra::Vector3<f32> {
    fn from(f: f32_f32_f32_f32) -> Self {
        Vector3::new(f.d0, f.d1, f.d2)
    }
}
