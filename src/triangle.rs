use failure;
use gl;
use nalgebra::{Perspective3, Rotation3, Translation3, Vector3, Vector4};

use crate::primitives::spatial::{Location, Orientation};
use crate::render_gl::data::f32_f32_f32_f32;
use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pub pos: data::f32_f32_f32_f32,
    pub clr: data::u2_u10_u10_u10_rev_float,
}

impl Vertex {
    pub(crate) fn oriented(self, orientation: Orientation) -> Self {
        let homogeneous_matrix = Rotation3::from_euler_angles(
            // TODO: for some reason these make more sense when roll is pitch,
            // pitch is yaw, and yaw is roll. Should probably investigate why.
            orientation.pitch,
            orientation.yaw,
            orientation.roll,
        )
        .to_homogeneous();

        let mut cloned = self.clone();

        cloned.pos = (&homogeneous_matrix * Vector4::from(self.pos)).into();

        cloned
    }

    pub(crate) fn translated(self, location: Location) -> Self {
        let homogeneous_matrix =
            Translation3::from(Vector3::new(location.x, location.y, location.z)).to_homogeneous();

        let mut cloned = self.clone();

        cloned.pos = (&homogeneous_matrix * Vector4::from(self.pos)).into();

        cloned
    }

    pub fn projected(self, aspect: f32) -> Self {
        let homogeneous_matrix =
            Perspective3::new(aspect, std::f32::consts::PI / 2f32, 0.1, 100000.0).to_homogeneous();

        let mut cloned = self.clone();

        cloned.pos = (&homogeneous_matrix * Vector4::from(self.pos)).into();

        cloned
    }

    pub(crate) fn view_from(self, location: Location, orientation: Orientation) -> Self {
        let cloned = self.clone();
        cloned
            .translated((-location.x, -location.y, -location.z).into())
            .oriented(Orientation {
                pitch: -orientation.pitch,
                roll: -orientation.roll,
                yaw: -orientation.yaw,
            })
    }
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

#[derive(Debug, Clone)]
pub(crate) struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Triangle {
        Triangle { a, b, c }
    }

    pub fn vertices(&self) -> Vec<Vertex> {
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
