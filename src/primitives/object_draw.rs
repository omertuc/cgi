use failure::Error;
use nalgebra::{Matrix4, Vector3, Vector4};

use crate::primitives::spatial::Location;
use crate::primitives::spotlight::Spotlight;
use crate::primitives::triangle::VertexData;
use crate::render_gl::buffer::{ArrayBuffer, VertexArray};
use crate::render_gl::Program;
use crate::resources::Resources;

pub struct ObjectUniforms {
    pub model_scale: i32,
    pub model_translation: i32,
    pub model_rotation: i32,
    pub view_translation: i32,
    pub view_rotation: i32,
    pub projection: i32,
    pub lights_count: i32,
    pub light_positions: i32,
    pub light_colors: i32,
    pub light_radiuses: i32,
}

impl ObjectUniforms {
    fn new(program: &Program) -> Result<Self, Error> {
        Ok(Self {
            model_scale: program.get_uniform_loc("model_scale")?,
            model_translation: program.get_uniform_loc("model_translation")?,
            model_rotation: program.get_uniform_loc("model_rotation")?,
            view_rotation: program.get_uniform_loc("view_rotation")?,
            view_translation: program.get_uniform_loc("view_translation")?,
            projection: program.get_uniform_loc("projection")?,
            lights_count: program.get_uniform_loc("lights_count")?,
            light_positions: program.get_uniform_loc("light_positions")?,
            light_colors: program.get_uniform_loc("light_colors")?,
            light_radiuses: program.get_uniform_loc("light_radiuses")?,
        })
    }
}

pub struct ObjectsDraw {
    pub program: Program,
    vbo: ArrayBuffer,
    vao: VertexArray,
    uniform_locs: ObjectUniforms,
}

impl ObjectsDraw {
    pub fn new(
        res: &Resources,
        gl: &gl::Gl,
        verticies: Vec<VertexData>,
    ) -> Result<ObjectsDraw, failure::Error> {
        let program = Program::from_res(gl, res, "shaders/triangle")?;

        let vbo = ArrayBuffer::new(&gl);
        let vao = VertexArray::new(&gl);

        vao.bind();
        vbo.bind();
        VertexData::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        let uniform_locs = ObjectUniforms::new(&program)?;

        let objects_draw = ObjectsDraw {
            program,
            vbo,
            vao,

            uniform_locs,
        };

        objects_draw.program.set_used();

        objects_draw.vbo.bind();
        objects_draw.vbo.static_draw_data(&verticies);

        Ok(objects_draw)
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

    pub(crate) fn set_spotlights<'a>(
        &self,
        lights: impl Iterator<Item = (&'a Spotlight, &'a Location)>,
    ) {
        self.program.set_used();

        let mut lights_count = 0;
        lights.enumerate().for_each(|(i, (light, location))| {
            self.program.set_vec3_array_uniform(
                self.uniform_locs.light_positions,
                i,
                &Vector3::new(location.x, location.y, location.z),
            );
            self.program.set_float_array_uniform(
                self.uniform_locs.light_radiuses,
                i,
                light.spot_radius,
            );
            self.program.set_vec4_array_uniform(
                self.uniform_locs.light_colors,
                i,
                &Vector4::new(light.color.r, light.color.g, light.color.b, light.color.a),
            );
            lights_count += 1;
        });

        self.program
            .set_uint_uniform(self.uniform_locs.lights_count, lights_count);
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
