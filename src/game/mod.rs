use std::collections::{HashMap, HashSet};
use std::f32::consts::TAU;

use sdl2::keyboard::Scancode;

use controls::{KeyStack, MouseMovement};
use time::GameTime;

use crate::game::camera::Camera;
use crate::game::controls::Groups;
use crate::resources::Resources;
use crate::triangle;
use crate::triangle::{Triangle, Vertex};

mod camera;
mod controls;
mod time;

const SPIN_PER_SECOND: f32 = TAU / 2f32;
const MOVEMENT_PER_SECOND: f32 = TAU / 2f32;
const SPIN_PER_MOUSE_PIXEL: f32 = TAU / 300f32;

struct Settings {
    vsync: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameKey {
    NoOp,
    RollModifier,
    CameraModifier,
    Right,
    Left,
    Up,
    Down,
    VsyncToggle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameKeyGroup {
    Horizontal,
    Vertical,
}

impl Groups for GameKey {
    type GroupType = GameKeyGroup;
    fn groups(&self) -> HashSet<GameKeyGroup> {
        match self {
            GameKey::Right | GameKey::Left => [GameKeyGroup::Horizontal],
            GameKey::Up | GameKey::Down => [GameKeyGroup::Vertical],
            _ => {
                return HashSet::<GameKeyGroup>::new();
            }
        }
        .iter()
        .copied()
        .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Location {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Orientation {
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}

impl From<(f32, f32, f32)> for Location {
    fn from(tuple: (f32, f32, f32)) -> Self {
        Location {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

type KeyMap = HashMap<Scancode, GameKey>;
type GameKeyStack = KeyStack<GameKey>;

pub(crate) struct Game {
    triangle_draw: triangle::TrianglesDraw,
    triangles: Vec<triangle::Triangle>,

    // camera
    camera: Camera,

    // controls
    key_map: KeyMap,
    key_stack: GameKeyStack,
    mouse_down: bool,

    // rotation
    roll_per_second: f32,
    yaw_per_second: f32,
    pitch_per_second: f32,

    // movement
    x_per_second: f32,
    y_per_second: f32,
    z_per_second: f32,

    // sdl
    video_subsystem: sdl2::VideoSubsystem,

    game_time: GameTime,

    // Game settings
    settings: Settings,
}

pub fn init_key_map() -> HashMap<Scancode, GameKey> {
    let game_keys = hashmap! {
        &[Scancode::A, Scancode::Left][..] => GameKey::Left,
        &[Scancode::D, Scancode::Right][..] => GameKey::Right,
        &[Scancode::W, Scancode::Up][..] => GameKey::Up,
        &[Scancode::S, Scancode::Down][..] => GameKey::Down,
        &[Scancode::V][..] => GameKey::VsyncToggle,
        &[Scancode::LShift, Scancode::RShift][..] => GameKey::RollModifier,
        &[Scancode::LCtrl, Scancode::RCtrl][..] => GameKey::CameraModifier,
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
        let second_fraction =
            (self.game_time.update_ticks(timer) as f32) * self.game_time.tick_second_ratio;

        let extra_pitch = self.pitch_per_second * second_fraction;
        let extra_yaw = self.yaw_per_second * second_fraction;
        let extra_roll = self.roll_per_second * second_fraction;

        let extra_x = self.x_per_second * second_fraction;
        let extra_y = self.y_per_second * second_fraction;
        let extra_z = self.z_per_second * second_fraction;

        self.camera.location.x += extra_x;
        self.camera.location.y += extra_y;
        self.camera.location.z += extra_z;

        self.camera.orientation.pitch += extra_pitch;
        self.camera.orientation.yaw += extra_yaw;
        self.camera.orientation.roll += extra_roll;
    }

    pub(crate) fn draw(&self, gl: &gl::Gl) {
        println!("{:#?}", self.camera);
        self.triangle_draw.draw(
            &gl,
            self.triangles
                .clone()
                .into_iter()
                .map(Triangle::rotated)
                .map(Triangle::translated)
                .map(|t| t.view_from(self.camera.location, self.camera.orientation))
                .flat_map(triangle::Triangle::vertices)
                .collect(),
        );
    }

    pub fn new(
        res: Resources,
        gl: &gl::Gl,
        initial_time: u64,
        timer_frequency: u64,
        tick_length_us: u64,
        video_subsystem: sdl2::VideoSubsystem,
    ) -> Result<Game, failure::Error> {
        let triangle_draw = triangle::TrianglesDraw::new(&res, &gl)?;
        let triangle_count = 200;

        let location = (0f32, 0f32, 0f32);

        let triangles: Vec<triangle::Triangle> = (0..triangle_count)
            .into_iter()
            .map(|triangle_index| (triangle_index as f32) * (TAU / triangle_count as f32))
            .map(|angle| {
                triangle::Triangle::new(
                    Vertex {
                        pos: (0.5, -0.5, 0.0).into(),
                        clr: (0.2, 0.2, 0.4, 0.3).into(),
                    },
                    Vertex {
                        pos: (-0.5, -0.5, 0.0).into(),
                        clr: (0.1, 0.1, 0.3, 0.3).into(),
                    },
                    Vertex {
                        pos: (0.0, 0.5, 0.0).into(),
                        clr: (0.3, 0.3, 0.5, 0.3).into(),
                    },
                    location.into(),
                    Orientation {
                        pitch: angle,
                        roll: angle,
                        yaw: angle,
                    }
                )
            })
            .collect();

        let mut game = Game {
            key_map: init_key_map(),
            triangle_draw,
            triangles,

            // Default rotation speed
            roll_per_second: 0f32,
            yaw_per_second: 0f32,
            pitch_per_second: 0f32,

            // Default movement speed
            x_per_second: 0f32,
            y_per_second: 0f32,
            z_per_second: 0f32,

            camera: Camera {
                location: Location {
                    x: 0f32,
                    y: 0f32,
                    z: 0f32,
                },
                orientation: Orientation {
                    pitch: 0f32,
                    roll: 0f32,
                    yaw: 0f32,
                },
            },

            video_subsystem,

            settings: Settings { vsync: false },
            game_time: GameTime::new(timer_frequency, tick_length_us, initial_time),
            key_stack: KeyStack::new(),
            mouse_down: false,
        };

        game.enable_vsync();

        Ok(game)
    }

    pub fn enable_vsync(&mut self) {
        if let Ok(_) = self
            .video_subsystem
            .gl_set_swap_interval(sdl2::video::SwapInterval::VSync)
        {
            self.settings.vsync = true;
        } else {
            println!("Failed to enable vsync")
        }
    }

    pub fn disable_vsync(&mut self) {
        if let Ok(_) = self
            .video_subsystem
            .gl_set_swap_interval(sdl2::video::SwapInterval::Immediate)
        {
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

    pub fn handle_movement(&mut self, normalized: GameKeyStack) {
        let speed = MOVEMENT_PER_SECOND;

        if normalized.is_pressed(GameKey::Up) {
            self.y_per_second = speed;
        } else if normalized.is_pressed(GameKey::Down) {
            self.y_per_second = -speed;
        } else {
            self.y_per_second = 0f32;
        }

        if normalized.is_pressed(GameKey::Right) {
            self.x_per_second = speed;
        } else if normalized.is_pressed(GameKey::Left) {
            self.x_per_second = -speed;
        } else {
            self.x_per_second = 0f32;
        }
    }

    pub fn handle_rotation(&mut self, normalized: GameKeyStack) {
        let speed = SPIN_PER_SECOND;

        if normalized.is_pressed(GameKey::Up) {
            self.pitch_per_second = speed;
        } else if normalized.is_pressed(GameKey::Down) {
            self.pitch_per_second = -speed;
        } else {
            self.pitch_per_second = 0f32;
        }

        if normalized.is_pressed(GameKey::Right) {
            if normalized.is_pressed(GameKey::RollModifier) {
                self.roll_per_second = speed;
            } else {
                self.yaw_per_second = speed;
            }
        } else if normalized.is_pressed(GameKey::Left) {
            if normalized.is_pressed(GameKey::RollModifier) {
                self.roll_per_second = -speed;
            } else {
                self.yaw_per_second = -speed;
            }
        } else {
            self.roll_per_second = 0f32;
            self.yaw_per_second = 0f32;
        }
    }

    pub fn keyboard_handler(&mut self) {
        let normalized = self.key_stack.normalize();

        if normalized.is_pressed(GameKey::VsyncToggle) {
            self.key_stack = self.key_stack.depress(GameKey::VsyncToggle);
            self.toggle_vsync();
        }

        if normalized.is_pressed(GameKey::CameraModifier) {
            self.handle_movement(normalized);
        } else {
            self.handle_rotation(normalized);
        }
    }

    pub fn mouse_moved(&mut self, movement: MouseMovement) {
        if self.mouse_down {
            let x_diff = SPIN_PER_MOUSE_PIXEL * (movement.0 as f32);
            let y_diff = SPIN_PER_MOUSE_PIXEL * (movement.1 as f32);
            if self.key_stack.normalize().is_pressed(GameKey::RollModifier) {
                self.camera.orientation.roll += x_diff;
            } else {
                self.camera.orientation.yaw += x_diff;
            }

            self.camera.orientation.pitch += y_diff;
        }
    }

    pub fn input_handler(&mut self, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => self.mouse_down = true,
            sdl2::event::Event::MouseButtonUp { .. } => self.mouse_down = false,
            sdl2::event::Event::MouseMotion { xrel, yrel, .. } => self.mouse_moved((xrel, yrel)),
            sdl2::event::Event::KeyDown {
                scancode: Option::Some(code),
                repeat,
                ..
            } => {
                if !repeat {
                    self.key_stack = self
                        .key_stack
                        .press(*self.key_map.get(&code).unwrap_or(&GameKey::NoOp));
                }
            }
            sdl2::event::Event::KeyUp {
                scancode: Option::Some(code),
                ..
            } => {
                self.key_stack = self
                    .key_stack
                    .depress(*self.key_map.get(&code).unwrap_or(&GameKey::NoOp));
            }
            _ => {}
        };

        self.keyboard_handler();
    }
}
