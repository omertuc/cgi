use std::f32::consts::{PI, TAU};

use crate::{TICK_LENGTH_US, triangle};
use crate::resources::Resources;

const SPIN_PER_SECOND: f32 = TAU / 3f32;
const US_PER_SECOND: u64 = 1_000_000;

struct Settings {
    vsync: bool,
}

pub(crate) struct Game {
    triangle_draw: triangle::TrianglesDraw,
    triangles: Vec<triangle::Triangle>,

    // movement
    roll: f32,
    yaw: f32,
    pitch: f32,

    // sdl
    video_subsystem: sdl2::VideoSubsystem,

    // Time constants
    timer_frequency: u64,
    previous_timer: u64,
    tick_length_us: u64,
    partial_tick_counter: u64,
    tick_length_counter: u64,
    tick_second_ratio: f32,

    // Game settings
    settings: Settings,
}

impl Game {
    fn update_ticks(&mut self, timer: u64) -> u64 {
        let time_passed_counter = self.partial_tick_counter + (timer - self.previous_timer);
        let ticks = time_passed_counter / self.tick_length_counter;
        self.partial_tick_counter = time_passed_counter % self.tick_length_counter;
        self.previous_timer = timer;

        return ticks;
    }

    pub(crate) fn process(&mut self, timer: u64) {
        let ticks = self.update_ticks(timer);

        let pitch = self.pitch * self.tick_second_ratio;
        let yaw = self.yaw * self.tick_second_ratio;
        let roll = self.roll * self.tick_second_ratio;

        self.triangles.iter_mut().for_each(|t| t.add_pitch(pitch * ticks as f32));
        self.triangles.iter_mut().for_each(|t| t.add_yaw(yaw * ticks as f32));
        self.triangles.iter_mut().for_each(|t| t.add_roll(roll * ticks as f32));
    }

    pub(crate) fn draw(&self, gl: &gl::Gl) {
        self.triangle_draw.draw(&gl, self.triangles.iter().flat_map(triangle::Triangle::vertices).collect());
    }

    pub fn new(res: Resources, gl: &gl::Gl, initial_time: u64, timer_frequency: u64,
               tick_length_us: u64, video_subsystem: sdl2::VideoSubsystem) -> Result<Game, failure::Error> {
        let triangle_draw = triangle::TrianglesDraw::new(&res, &gl)?;
        let triangle_count = 1;

        let mut triangles: Vec<triangle::Triangle> = (0..triangle_count).into_iter().map(
            |triangle_index| (triangle_index as f32) * (TAU / triangle_count as f32)
        ).map(
            |angle| triangle::Triangle::new(angle, angle, angle)
        ).collect();

        let counter_per_us: u64 = US_PER_SECOND / timer_frequency;

        let mut game = Game {
            triangle_draw,
            triangles,
            roll: 0f32,
            yaw: 0f32,
            pitch: 0f32,
            tick_length_us,
            tick_length_counter: (counter_per_us * tick_length_us),
            previous_timer: initial_time,
            timer_frequency,
            partial_tick_counter: 0,
            tick_second_ratio: (tick_length_us as f32) / (US_PER_SECOND as f32),
            video_subsystem,
            settings: Settings {
                vsync: false,
            },
        };

        game.disable_vsync();

        Ok(game)
    }

    pub fn enable_vsync(&mut self) {
        self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync);
        self.settings.vsync = true;
    }

    pub fn disable_vsync(&mut self) {
        self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate);
        self.settings.vsync = false;
    }

    pub fn toggle_vsync(&mut self) {
        if self.settings.vsync {
            self.disable_vsync()
        } else {
            self.enable_vsync()
        }
    }

    pub fn input_handler(&mut self, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Option::Some(code),
                keymod: mode,
                ..
            } => {
                match code {
                    sdl2::keyboard::Keycode::D => {
                        match mode {
                            sdl2::keyboard::Mod::LSHIFTMOD => {
                                self.roll = SPIN_PER_SECOND;
                            }
                            _ => {
                                self.yaw = SPIN_PER_SECOND;
                            }
                        }
                    }
                    sdl2::keyboard::Keycode::A => {
                        match mode {
                            sdl2::keyboard::Mod::LSHIFTMOD => {
                                self.roll = -SPIN_PER_SECOND;
                            }
                            _ => {
                                self.yaw = -SPIN_PER_SECOND;
                            }
                        }
                    }
                    sdl2::keyboard::Keycode::W => {
                        self.pitch = SPIN_PER_SECOND;
                    }
                    sdl2::keyboard::Keycode::S => {
                        self.pitch = -SPIN_PER_SECOND;
                    }
                    _ => {}
                }
            }
            sdl2::event::Event::KeyUp {
                keycode: Option::Some(code),
                keymod: mode,
                ..
            } => {
                match code {
                    sdl2::keyboard::Keycode::D | sdl2::keyboard::Keycode::A => {
                        self.roll = 0f32;
                        self.yaw = 0f32;
                    }
                    sdl2::keyboard::Keycode::W | sdl2::keyboard::Keycode::S => {
                        self.pitch = 0f32;
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }
}
