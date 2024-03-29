#[macro_use]
extern crate failure;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate render_gl_derive;

use std::path::Path;

use failure::err_msg;

use nalgebra::Vector3;
use resources::Resources;

mod debug;
mod game;
mod models;
mod primitives;
pub mod render_gl;
pub mod resources;

fn main() {
    if let Err(e) = run() {
        println!("{}", debug::failure_to_string(e));
    }
}

const TICK_LENGTH_US: u64 = 100;

fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets"))?;

    let sdl = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl.video().map_err(err_msg)?;

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(8);

    let window = video_subsystem
        .window("Game", 2560, 1440)
        .opengl()
        .resizable()
        .build()
        .map_err(err_msg)?;

    let _gl_context = window.gl_create_context().map_err(err_msg)?;

    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl.Enable(gl::BLEND);
        gl.Enable(gl::DEPTH_TEST);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let mut viewport = render_gl::Viewport::for_window(900, 700);
    viewport.set_used(&gl);

    let color_buffer = render_gl::ColorBuffer::from_color(Vector3::new(0.0, 0.0, 0.0));

    color_buffer.set_used(&gl);
    color_buffer.clear(&gl);

    let mut event_pump = sdl.event_pump().map_err(err_msg)?;

    let timer_subsystem = sdl.timer().map_err(err_msg)?;

    sdl.mouse().show_cursor(false);
    sdl.mouse().set_relative_mouse_mode(true);

    let mut game = game::Game::new(
        res,
        &gl,
        timer_subsystem.performance_counter(),
        dbg!(timer_subsystem.performance_frequency()),
        TICK_LENGTH_US,
        video_subsystem,
        (viewport.w / viewport.h) as f32,
    )?;

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
                    game.set_aspect_ratio(w as f32 / h as f32);
                }
                _ => {
                    game.input_handler(event);
                }
            }
        }

        color_buffer.clear(&gl);

        game.process(timer_subsystem.performance_counter());

        if game.ongoing {
            game.draw(&gl);
        } else {
            break;
        }

        window.gl_swap_window();
    }

    Ok(())
}
