mod triangle;

#[macro_use]
extern crate failure;
extern crate gl;
#[macro_use]
extern crate render_gl_derive;
extern crate sdl2;
extern crate vec_2_10_10_10;

use std::path::Path;

use render_gl::buffer;

use failure::err_msg;

use render_gl::data;
use resources::Resources;

use crate::render_gl::data::f32_f32_f32;
use crate::render_gl::buffer::BufferTypeArray;

pub mod resources;

pub mod render_gl;


fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e));
    }
}

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build().map_err(err_msg)?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s)
        as *const std::os::raw::c_void);

    let shader_program = render_gl::Program::from_res(
        &gl, &res, "shaders/triangle",
    )?;


    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }


    let mut event_pump = sdl.event_pump().map_err(err_msg)?;

    let triangle = triangle::Triangle::new(&res, &gl)?;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }

        triangle.render(&gl);

        window.gl_swap_window()
    }

    Ok(())
}

pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    for (i, cause) in e.iter_chain().collect::<Vec<_>>().into_iter().rev().enumerate() {
        if i > 0 {
            let _ = writeln!(&mut result, "   Which caused the following issue:");
        }
        let _ = write!(&mut result, "{}", cause);
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_str = format!("{}", backtrace);
            if backtrace_str.len() > 0 {
                let _ = writeln!(&mut result, " This happened at {}", backtrace);
            } else {
                let _ = writeln!(&mut result);
            }
        } else {
            let _ = writeln!(&mut result);
        }
    }

    result
}