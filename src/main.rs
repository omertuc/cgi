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

    unsafe {
        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut viewport = render_gl::Viewport::for_window(900, 700);
    viewport.set_used(&gl);

    let color_buffer = render_gl::ColorBuffer::from_color(
        na::Vector3::new(0.04, 0.05, 0.04));

    color_buffer.set_used(&gl);
    color_buffer.clear(&gl);

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;

    let triangle_draw = triangle::TrianglesDraw::new(&res, &gl)?;

    let mut triangles = vec![];
    let count = 100;
    for triangle_index in 1..count {
        triangles.push(triangle::Triangle::new(
            (triangle_index as f32) * (std::f32::consts::TAU / count as f32))
        );
    }

    let mut vis: isize = 0;
    let mut up: isize = 1;
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

        color_buffer.clear(&gl);

        for mut triangle in &mut triangles {
            triangle.add_angle(0.02f32)
        }

        triangle_draw.draw(&gl, triangles.iter().flat_map(triangle::Triangle::vertices).take(vis as usize).collect());

        vis += up;

        if vis == count {
            up = -1;
        } else if vis == 0 {
            up = 1;
        }

        window.gl_swap_window();
    }

    Ok(())
}
