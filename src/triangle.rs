use failure;
use gl;
use nalgebra::{Matrix4, Vector3, Vector4};

use crate::render_gl::data::f32_f32_f32_f32;
use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;

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

    pub fn draw(
        &self,
        gl: &gl::Gl,
        vertices: &Vec<Vertex>,
        model_translation: &Matrix4<f32>,
        model_rotation: &Matrix4<f32>,
    ) {
        self.program.set_used();

        self.program
            .set_mat4_uniform("model_translation", &model_translation)
            .unwrap();
        self.program
            .set_mat4_uniform("model_rotation", &model_rotation)
            .unwrap();

        self.vbo.bind();
        self.vbo.dynamic_draw_data(&vertices);

        self.vao.bind();

        unsafe {
            gl.DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
        }
    }

    pub fn set_view(&self, view_translation: &Matrix4<f32>, view_rotation: &Matrix4<f32>) {
        self.program
            .set_mat4_uniform("view_translation", &view_translation)
            .unwrap();
        self.program
            .set_mat4_uniform("view_rotation", &view_rotation)
            .unwrap();
    }

    pub fn set_projection(&self, projection: &Matrix4<f32>) {
        self.program
            .set_mat4_uniform("projection", &projection)
            .unwrap();
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
