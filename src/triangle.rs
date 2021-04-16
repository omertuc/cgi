use failure;
use gl;
use nalgebra::{Matrix4, Vector3, Vector4};

use crate::primitives::light::Color;
use crate::primitives::spatial::Location;
use crate::render_gl::{self, buffer, data};
use crate::render_gl::data::{f32_f32_f32, f32_f32_f32_f32};
use crate::resources::Resources;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexData {
    pub pos: data::f32_f32_f32,
    pub clr: data::u2_u10_u10_u10_rev_float,
    pub norm: data::f32_f32_f32,
}

impl VertexData {
    fn new(vertex: Vertex, normal: Vector3<f32>) -> VertexData {
        VertexData {
            pos: vertex.pos.into(),
            clr: vertex.clr.into(),
            norm: normal.into(),
        }
    }
}

pub(crate) struct Vertex {
    pub pos: Location,
    pub clr: Color,
}

pub struct TrianglesDraw {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    model_scale_uniform_loc: i32,
    model_translation_uniform_loc: i32,
    model_rotation_uniform_loc: i32,
    view_translation_uniform_loc: i32,
    view_rotation_uniform_loc: i32,
    projection_uniform_loc: i32,
}

impl TrianglesDraw {
    pub fn new(res: &Resources, gl: &gl::Gl, verticies: Vec<VertexData>) -> Result<TrianglesDraw, failure::Error> {
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vbo = buffer::ArrayBuffer::new(&gl);
        let vao = buffer::VertexArray::new(&gl);

        vao.bind();
        vbo.bind();
        VertexData::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        let model_scale_uniform_loc = program.get_uniform_loc("model_scale")?;
        let model_translation_uniform_loc = program.get_uniform_loc("model_translation")?;
        let model_rotation_uniform_loc = program.get_uniform_loc("model_rotation")?;
        let view_rotation_uniform_loc = program.get_uniform_loc("view_rotation")?;
        let view_translation_uniform_loc = program.get_uniform_loc("view_translation")?;
        let projection_uniform_loc = program.get_uniform_loc("projection")?;

        let triangles_draw = TrianglesDraw {
            program,
            vbo,
            vao,
            model_scale_uniform_loc,
            model_translation_uniform_loc,
            model_rotation_uniform_loc,
            view_rotation_uniform_loc,
            view_translation_uniform_loc,
            projection_uniform_loc,
        };

        triangles_draw.program.set_used();

        triangles_draw.vbo.bind();
        triangles_draw.vbo.static_draw_data(&verticies);
        triangles_draw.vao.bind();

        Ok(triangles_draw)
    }

    pub fn draw(
        &self,
        gl: &gl::Gl,
        model_scale: f32,
        model_translation: &Matrix4<f32>,
        model_rotation: &Matrix4<f32>,
        num_vertices: usize,
        offset: usize,
    ) {
        self.program
            .set_float_uniform(self.model_scale_uniform_loc, model_scale)
            .unwrap();
        self.program
            .set_mat4_uniform(self.model_translation_uniform_loc, model_translation)
            .unwrap();
        self.program
            .set_mat4_uniform(self.model_rotation_uniform_loc, model_rotation)
            .unwrap();

        unsafe {
            gl.DrawArrays(gl::TRIANGLES, offset as i32, num_vertices as i32);
        }
    }

    pub fn set_view(&self, view_translation: &Matrix4<f32>, view_rotation: &Matrix4<f32>) {
        self.program
            .set_mat4_uniform(self.view_translation_uniform_loc, &view_translation)
            .unwrap();
        self.program
            .set_mat4_uniform(self.view_rotation_uniform_loc, &view_rotation)
            .unwrap();
    }

    pub fn set_projection(&self, projection: &Matrix4<f32>) {
        self.program
            .set_mat4_uniform(self.projection_uniform_loc, &projection)
            .unwrap();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Triangle {
    pub a: VertexData,
    pub b: VertexData,
    pub c: VertexData,
}

impl Triangle {
    pub fn new(a: Vertex, b: Vertex, c: Vertex) -> Triangle {
        let avec: Vector3<f32> = a.pos.into();
        let bvec: Vector3<f32> = b.pos.into();
        let cvec: Vector3<f32> = c.pos.into();

        let normal = (bvec - avec).cross(&(cvec - &avec)).normalize();

        Triangle {
            a: VertexData::new(a, normal),
            b: VertexData::new(b, normal),
            c: VertexData::new(c, normal),
        }
    }

    pub fn vertices(&self) -> Vec<VertexData> {
        vec![self.a, self.b, self.c].into()
    }
}

impl From<Vector4<f32>> for f32_f32_f32_f32 {
    fn from(vector: Vector4<f32>) -> Self {
        (vector.x, vector.y, vector.z, vector.w).into()
    }
}

impl From<Vector3<f32>> for f32_f32_f32 {
    fn from(vector: Vector3<f32>) -> Self {
        (vector.x, vector.y, vector.z).into()
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
