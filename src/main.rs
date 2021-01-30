extern crate gl;
extern crate sdl2;

use std::ffi::{CStr, CString};

fn shader_from_source(source: &std::ffi::CStr,
                      kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {}

    Ok(id);

    let mut len: gl::types::GLint = 0;

    unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));

    let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

    unsafe {
        gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
        );
    }

    return Err(error.to_string_lossy().into_owned());
}

fn main() {
    println!("Hello, worl!");

    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .borderless()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s)
        as *const std::os::raw::c_void);

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window()
    }
}
