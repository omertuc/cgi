#[macro_use]
extern crate failure;
#[macro_use]
extern crate render_gl_derive;

use std::path::Path;

use failure::err_msg;
use nalgebra as na;

use resources::Resources;

pub mod render_gl;
pub mod resources;
mod triangle;
mod debug;


fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
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
        .window("Game", 100, 100)
        .opengl()
        .resizable()
        .build().map_err(err_msg)?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s)
        as *const std::os::raw::c_void);

    let mut viewport = render_gl::Viewport::for_window(900, 700);
    viewport.set_used(&gl);

    let color_buffer = render_gl::ColorBuffer::from_color(
        na::Vector3::new(0.3, 0.3, 0.5));

    color_buffer.set_used(&gl);
    color_buffer.clear(&gl);

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;

    let triangle = triangle::Triangle::new(&res, &gl)?;
    let triangle2 = triangle::Triangle::new(&res, &gl)?;
    let triangle3 = triangle::Triangle::new(&res, &gl)?;
    let triangle4 = triangle::Triangle::new(&res, &gl)?;
    let triangle5 = triangle::Triangle::new(&res, &gl)?;

    let mut rot = 0.0f32;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                _ => {}
            }
        }

        rot += 0.01f32;

        color_buffer.clear(&gl);
        triangle.set_angle(&gl, rot);
        triangle2.set_angle(&gl, rot + 0.5f32);
        triangle3.set_angle(&gl, rot + 1f32);
        triangle4.set_angle(&gl, rot + 1.5f32);
        triangle5.set_angle(&gl, rot + 2.5f32);

        window.gl_swap_window();
    }

    Ok(())
}
