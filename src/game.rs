use std::f32::consts::{PI, TAU};

use sdl2::keyboard::Scancode;

use crate::{TICK_LENGTH_US, triangle};
use crate::resources::Resources;

const SPIN_PER_SECOND: f32 = TAU;
const US_PER_SECOND: u64 = 1_000_000;

struct Settings {
    vsync: bool,
}

#[derive(Debug)]
struct KeyState {
    roll_modifier: bool,
    right: bool,
    left: bool,
    up: bool,
    down: bool,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState {
            roll_modifier: false,
            right: false,
            left: false,
            up: false,
            down: false,
        }
    }
}

pub(crate) struct Game {
    triangle_draw: triangle::TrianglesDraw,
    triangles: Vec<triangle::Triangle>,

    // controls
    key_state: KeyState,

    // movement
    roll: f32,
    yaw: f32,
    pitch: f32,

    // sdl
    video_subsystem: sdl2::VideoSubsystem,

    // Time measurements
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
            key_state: KeyState::new(),
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

    pub fn keyboard_handler(&mut self) {
        let speed = SPIN_PER_SECOND;

        if self.key_state.left {
            if self.key_state.roll_modifier {
                self.roll = speed;
            } else {
                self.yaw = speed;
            }
        }

        if self.key_state.right {
            if self.key_state.roll_modifier {
                self.roll = -speed;
            } else {
                self.yaw = -speed;
            }
        }

        if !self.key_state.left && !self.key_state.right {
            self.roll = 0f32;
            self.yaw = 0f32;
        }

        if self.key_state.up {
            self.pitch = speed;
        }

        if self.key_state.down {
            self.pitch = -speed;
        }

        if !(self.key_state.up || self.key_state.down) {
            self.pitch = 0f32;
        }
    }

    pub fn input_handler(&mut self, event: sdl2::event::Event) {
        let left_keys = [Scancode::A, Scancode::Left];
        let right_keys = [Scancode::D, Scancode::Right];
        let up_keys = [Scancode::W, Scancode::Up];
        let down_keys = [Scancode::S, Scancode::Down];
        let roll_modifier = [Scancode::LShift, Scancode::RShift];

        match event {
            sdl2::event::Event::KeyDown {
                scancode: Option::Some(code),
                keymod: mode,
                ..
            } => {
                if left_keys.contains(&code) {
                    self.key_state.left = true;
                }
                if right_keys.contains(&code) {
                    self.key_state.right = true;
                }
                if up_keys.contains(&code) {
                    self.key_state.up = true;
                }
                if down_keys.contains(&code) {
                    self.key_state.down = true;
                }
                if roll_modifier.contains(&code) {
                    self.key_state.roll_modifier = true;
                }
            }
            sdl2::event::Event::KeyUp {
                scancode: Option::Some(code),
                keymod: mode,
                ..
            } => {
                if left_keys.contains(&code) {
                    self.key_state.left = false;
                }
                if right_keys.contains(&code) {
                    self.key_state.right = false;
                }
                if up_keys.contains(&code) {
                    self.key_state.up = false;
                }
                if down_keys.contains(&code) {
                    self.key_state.down = false;
                }
                if roll_modifier.contains(&code) {
                    self.key_state.roll_modifier = false;
                }

                match code {
                    sdl2::keyboard::Scancode::V => {
                        self.toggle_vsync()
                    }
                    _ => {}
                }
            }
            _ => {}
        };

        println!("{:#?}", self.key_state);

        self.keyboard_handler();
    }
}
