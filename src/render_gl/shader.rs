use std;
use std::ffi::{CStr, CString};

use gl;
use nalgebra::{Matrix4, Vector3, Vector4};

use crate::resources;
use crate::resources::Resources;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad {
        name: String,
        #[cause]
        inner: resources::Error,
    },
    #[fail(display = "Cannot determine shader type for resource {}", name)]
    CannotDetermineShaderTypeForResource { name: String },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
    #[fail(display = "Uniform with name \"{}\" not found", name)]
    UniformNameError { name: String },
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Shader, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CannotDetermineShaderTypeForResource { name: name.into() })?;

        let source = res.load_cstring(name).map_err(|e| Error::ResourceLoad {
            name: name.into(),
            inner: e,
        })?;

        Shader::from_source(gl, &source, shader_kind).map_err(|message| Error::CompileError {
            name: name.into(),
            message,
        })
    }

    fn from_source(gl: &gl::Gl, source: &CStr, kind: gl::types::GLuint) -> Result<Shader, String> {
        let id = shader_from_source(gl, source, kind)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteShader(self.id) }
    }
}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Program, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let shaders = POSSIBLE_EXT
            .iter()
            .map(|file_extension| Shader::from_res(gl, res, &format!("{}{}", name, file_extension)))
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders[..]).map_err(|message| Error::LinkError {
            name: name.into(),
            message,
        })
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;

            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(program_id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(program_id, shader.id());
            }
        }

        Ok(Program {
            gl: gl.clone(),
            id: program_id,
        })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn get_uniform_loc(&self, uniform_name: &str) -> Result<i32, Error> {
        let loc;
        let cname = std::ffi::CString::new(uniform_name).expect("CString::new failed");
        unsafe {
            loc = self.gl.GetUniformLocation(self.id(), cname.as_ptr());
        }

        if loc == -1 {
            return Err(Error::UniformNameError {
                name: uniform_name.to_string(),
            });
        }

        Ok(loc)
    }

    pub fn set_vec4_uniform(&self, loc: i32, vec: &Vector4<f32>) {
        unsafe {
            self.gl.Uniform4f(loc, vec[0], vec[1], vec[2], vec[3]);
        }
    }

    pub fn set_vec3_uniform(&self, loc: i32, vec: &Vector3<f32>) {
        unsafe {
            self.gl.Uniform3f(loc, vec[0], vec[1], vec[2]);
        }
    }

    pub fn set_mat4_uniform(&self, loc: i32, mat: &Matrix4<f32>) {
        unsafe {
            self.gl.UniformMatrix4fv(loc, 1, gl::FALSE, mat.as_slice().as_ptr());
        }
    }

    pub fn set_vec4_array_uniform(&self, loc: i32, idx: usize, vec: &Vector4<f32>) {
        unsafe {
            self.gl.Uniform4fv(loc + idx as i32, 1, vec.as_slice().as_ptr());
        }
    }

    pub fn set_vec3_array_uniform(&self, loc: i32, idx: usize, vec: &Vector3<f32>) {
        unsafe {
            self.gl.Uniform3fv(loc + idx as i32, 1, vec.as_slice().as_ptr());
        }
    }

    pub fn set_float_array_uniform(&self, loc: i32, idx: usize, float: f32) {
        unsafe {
            self.gl.Uniform1fv(loc + idx as i32, 1, &float);
        }
    }

    pub fn set_float_uniform(&self, loc: i32, float: f32) {
        unsafe {
            self.gl.Uniform1f(loc, float);
        }
    }

    pub fn set_uint_uniform(&self, loc: i32, uint: usize) {
        unsafe {
            self.gl.Uniform1ui(loc, uint as u32);
        }
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) }
    }
}

fn shader_from_source(gl: &gl::Gl, source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl.CreateShader(kind) };

    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
