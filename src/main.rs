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

    let mut window = video_subsystem
        .window("Game", 2560, 1440)
        .opengl()
        .resizable()
        .build().map_err(err_msg)?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;

    video_subsystem.gl_set_swap_interval(1);

    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s)
        as *const std::os::raw::c_void);

    println!("{:#?}", video_subsystem.current_display_mode(0).unwrap());

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

    let count = 3;

    let mut triangles: Vec<triangle::Triangle> = (0..count).into_iter().map(
        |i| triangle::Triangle::new((i as f32) * (std::f32::consts::TAU / count as f32))
    ).collect();

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

        triangles.iter_mut().for_each(|t| t.add_angle(0.01f32));
        triangle_draw.draw(&gl, triangles.iter().flat_map(triangle::Triangle::vertices).collect());

        window.gl_swap_window();
    }

    Ok(())
}
