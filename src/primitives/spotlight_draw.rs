use failure::Error;
use nalgebra::{Matrix4, Vector4};

use crate::primitives::triangle::VertexData;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};
use crate::render_gl::Program;
use crate::resources::Resources;

pub struct SpotlightUniforms {
    pub model_scale: i32,
    pub model_translation: i32,
    pub model_rotation: i32,
    pub view_translation: i32,
    pub view_rotation: i32,
    pub projection: i32,
    pub solid_color: i32,
}

impl SpotlightUniforms {
    fn new(program: &Program) -> Result<Self, Error> {
        Ok(Self {
            model_scale: program.get_uniform_loc("model_scale")?,
            model_translation: program.get_uniform_loc("model_translation")?,
            model_rotation: program.get_uniform_loc("model_rotation")?,
            view_rotation: program.get_uniform_loc("view_rotation")?,
            view_translation: program.get_uniform_loc("view_translation")?,
            projection: program.get_uniform_loc("projection")?,
            solid_color: program.get_uniform_loc("solid_color")?,
        })
    }
}

pub struct SpotlightDraw {
    pub program: Program,
    vbo: ArrayBuffer,
    vao: VertexArray,
    uniform_locs: SpotlightUniforms,
}

impl SpotlightDraw {
    pub fn new(res: &Resources, gl: &gl::Gl, verticies: Vec<VertexData>) -> Result<Self, Error> {
        let program = Program::from_res(gl, res, "shaders/triangle_spotlight")?;

        let vbo = ArrayBuffer::new(&gl);
        let vao = VertexArray::new(&gl);

        vao.bind();
        vbo.bind();
        VertexData::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        let uniform_locs = SpotlightUniforms::new(&program)?;

        let spotlight_draw = SpotlightDraw {
            program,
            vbo,
            vao,

            uniform_locs,
        };

        spotlight_draw.program.set_used();

        spotlight_draw.vbo.bind();
        spotlight_draw.vbo.static_draw_data(&verticies);
        spotlight_draw.vao.bind();

        Ok(spotlight_draw)
    }

    pub fn prepare_for_draws(&self) {
        self.program.set_used();
        self.vbo.bind();
        self.vao.bind();
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
            .set_float_uniform(self.uniform_locs.model_scale, model_scale);
        self.program
            .set_mat4_uniform(self.uniform_locs.model_translation, model_translation);
        self.program
            .set_mat4_uniform(self.uniform_locs.model_rotation, model_rotation);

        unsafe {
            gl.DrawArrays(gl::TRIANGLES, offset as i32, num_vertices as i32);
        }
    }

    #[allow(dead_code)]
    pub fn set_solid_color(&self, color: &Vector4<f32>) {
        self.program.set_used();
        self.program
            .set_vec4_uniform(self.uniform_locs.solid_color, color);
    }

    pub fn set_view(&self, view_translation: &Matrix4<f32>, view_rotation: &Matrix4<f32>) {
        self.program.set_used();
        self.program
            .set_mat4_uniform(self.uniform_locs.view_translation, &view_translation);
        self.program
            .set_mat4_uniform(self.uniform_locs.view_rotation, &view_rotation);
    }

    pub fn set_projection(&self, projection: &Matrix4<f32>) {
        self.program.set_used();
        self.program
            .set_mat4_uniform(self.uniform_locs.projection, &projection);
    }
}
