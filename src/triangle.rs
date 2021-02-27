use failure;
use gl;

use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::u2_u10_u10_u10_rev_float,
    rot: data::f32_,
}

pub struct TrianglesDraw {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl TrianglesDraw {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<TrianglesDraw, failure::Error> {
        let default_angle = std::f32::consts::PI;
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vbo = buffer::ArrayBuffer::new(&gl);
        let vao = buffer::VertexArray::new(&gl);

        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        let triangle = TrianglesDraw {
            program,
            vbo,
            vao,
        };

        Ok(triangle)
    }

    pub fn draw(&self, gl: &gl::Gl, vertices: Vec<Vertex>) {
        self.vbo.bind();
        self.vbo.dynamic_draw_data(&vertices);

        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                vertices.len() as i32,
            );
        }
    }
}

pub struct Triangle {
    angle: f32
}

impl Triangle {
    pub fn new(angle: f32) -> Triangle {
        Triangle {
            angle
        }
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle
    }

    pub fn add_angle(&mut self, angle: f32) {
        self.angle += angle
    }

    pub fn vertices(&self) -> Vec<Vertex> {
    vec![
        Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.2, 0.2, 0.4, 0.3).into(), rot: self.angle.into() },
        Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.1, 0.1, 0.3, 0.3).into(), rot: self.angle.into() },
        Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.3, 0.3, 0.5, 0.3).into(), rot: self.angle.into() },
    ]
    }

}