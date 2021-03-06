use std::collections::HashMap;
use std::f32::consts::TAU;

use sdl2::keyboard::Scancode;

use controls::{GameKey, KeyStack, MouseMovement};
use time::GameTime;

use crate::resources::Resources;
use crate::triangle;

mod controls;
mod time;

const SPIN_PER_SECOND: f32 = TAU / 2f32;
const SPIN_PER_MOUSE_PIXEL: f32 = TAU / 300f32;

struct Settings {
    vsync: bool,
}

pub(crate) struct Game {
    triangle_draw: triangle::TrianglesDraw,
    triangles: Vec<triangle::Triangle>,

    // controls
    key_stack: KeyStack,
    mouse_down: bool,

    // movement
    roll_per_second: f32,
    yaw_per_second: f32,
    pitch_per_second: f32,

    // sdl
    video_subsystem: sdl2::VideoSubsystem,

    game_time: GameTime,

    // Game settings
    settings: Settings,
}

pub fn init_key_map() -> HashMap<Scancode, GameKey> {
    let game_keys = hashmap! {
            [Scancode::A, Scancode::Left] => GameKey::Left,
            [Scancode::D, Scancode::Right] => GameKey::Right,
            [Scancode::W, Scancode::Up] => GameKey::Up,
            [Scancode::S, Scancode::Down] => GameKey::Down,
            [Scancode::LShift, Scancode::RShift] => GameKey::RollModifier,
        };


    // Flatten
    let mut final_map = HashMap::new();
    for (game_key, value) in game_keys {
        for key in game_key.iter() {
            final_map.insert(*key, value);
        }
    }

    final_map
}

impl Game {
    pub(crate) fn process(&mut self, timer: u64) {
        let ticks = self.game_time.update_ticks(timer);

        let pitch = self.pitch_per_second * self.game_time.tick_second_ratio;
        let yaw = self.yaw_per_second * self.game_time.tick_second_ratio;
        let roll = self.roll_per_second * self.game_time.tick_second_ratio;

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

        let triangles: Vec<triangle::Triangle> = (0..triangle_count).into_iter().map(
            |triangle_index| (triangle_index as f32) * (TAU / triangle_count as f32)
        ).map(
            |angle| triangle::Triangle::new(angle, angle, angle)
        ).collect();

        let mut game = Game {
            triangle_draw,
            triangles,
            roll_per_second: 0f32,
            yaw_per_second: 0f32,
            pitch_per_second: 0f32,
            video_subsystem,
            settings: Settings {
                vsync: false,
            },
            game_time: GameTime::new(timer_frequency, tick_length_us, initial_time),
            key_stack: KeyStack::new(),
            mouse_down: false,
        };

        game.enable_vsync();

        Ok(game)
    }

    pub fn enable_vsync(&mut self) {
        if let Ok(_) = self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::VSync) {
            self.settings.vsync = true;
        } else {
            println!("Failed to enable vsync")
        }
    }

    pub fn disable_vsync(&mut self) {
        if let Ok(_) = self.video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate) {
            self.settings.vsync = false;
        } else {
            println!("Failed to disable vsync")
        }
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

        if self.key_stack.normalize().is_pressed(GameKey::Up) {
            self.pitch_per_second = speed;
        } else if self.key_stack.normalize().is_pressed(GameKey::Down) {
            self.pitch_per_second = -speed;
        } else {
            self.pitch_per_second = 0f32;
        }

        if self.key_stack.normalize().is_pressed(GameKey::Right) {
            if self.key_stack.normalize().is_pressed(GameKey::RollModifier) {
                self.roll_per_second = speed;
            } else {
                self.yaw_per_second = speed;
            }
        } else if self.key_stack.normalize().is_pressed(GameKey::Left) {
            if self.key_stack.normalize().is_pressed(GameKey::RollModifier) {
                self.roll_per_second = -speed;
            } else {
                self.yaw_per_second = -speed;
            }
        } else {
            self.roll_per_second = 0f32;
            self.yaw_per_second = 0f32;
        }
    }

    pub fn mouse_moved(&mut self, movement: MouseMovement) {
        if self.mouse_down {
            if self.key_stack.normalize().is_pressed(GameKey::RollModifier) {
                self.triangles.iter_mut().for_each(|t| t.add_roll(SPIN_PER_MOUSE_PIXEL * movement.0 as f32));
            } else {
                self.triangles.iter_mut().for_each(|t| t.add_yaw(SPIN_PER_MOUSE_PIXEL * movement.0 as f32));
            }
            self.triangles.iter_mut().for_each(|t| t.add_pitch(SPIN_PER_MOUSE_PIXEL * movement.1 as f32));
        }
    }

    pub fn input_handler(&mut self, event: sdl2::event::Event) {
        let keymap = init_key_map();

        match event {
            sdl2::event::Event::MouseButtonDown {
                ..
            } => {
                self.mouse_down = true
            }
            sdl2::event::Event::MouseButtonUp {
                ..
            } => {
                self.mouse_down = false
            }
            sdl2::event::Event::MouseMotion {
                xrel,
                yrel,
                ..
            } => {
                self.mouse_moved((xrel, yrel))
            }
            sdl2::event::Event::KeyDown {
                scancode: Option::Some(code),
                ..
            } => {
                if let Some(key) = keymap.get(&code) {
                    self.key_stack = self.key_stack.press(*key)
                }
            }
            sdl2::event::Event::KeyUp {
                scancode: Option::Some(code),
                ..
            } => {
                if let Some(key) = keymap.get(&code) {
                    self.key_stack = self.key_stack.depress(*key);
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

        self.keyboard_handler();
    }
}
