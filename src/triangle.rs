use failure;
use gl;

use crate::render_gl::{self, buffer, data};
use crate::resources::Resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    pos: data::f32_f32_f32,
    clr: data::u2_u10_u10_u10_rev_float,
    rot: data::f32_,
}

pub struct Triangle {
    program: render_gl::Program,
    vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {
        let default_angle = std::f32::consts::PI;
        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vbo = buffer::ArrayBuffer::new(&gl);
        let vao = buffer::VertexArray::new(&gl);

        let triangle = Triangle {
            program,
            vbo,
            vao,
        };

        triangle.set_angle(&gl, default_angle);

        Ok(triangle)
    }

    pub fn set_angle(&self, gl: &gl::Gl, angle: f32) {
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into(), rot: angle.into() },
            Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into(), rot: angle.into() },
            Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into(), rot: angle.into() },
        ];

        self.vbo.bind();
        self.vbo.dynamic_draw_data(&vertices);
        self.vbo.unbind();

        self.vao.bind();
        self.vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        self.vbo.unbind();
        self.vao.unbind();

        self.render(&gl);
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3,
            )
        }
    }
}
