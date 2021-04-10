use std::f32::consts::TAU;

use image::{GenericImageView, Pixel};
use nalgebra::Matrix4;
use rand::Rng;
use sdl2::mouse::MouseWheelDirection;

use controls::GameKey;
use controls::KeyMap;

use crate::game::controls::{init_key_map, GameKeyStack};
use crate::primitives::camera::Camera;
use crate::primitives::input::{KeyStack, MouseMovement};
use crate::primitives::projection::perspective;
use crate::primitives::spatial::{Location, Orientation};
use crate::primitives::time::GameTime;
use crate::resources::Resources;
use crate::triangle;

mod controls;
mod cube;

const SPIN_PER_SECOND: f32 = TAU / 2f32;
const MOVEMENT_PER_SECOND: f32 = TAU / 2f32;
const SPIN_PER_MOUSE_PIXEL: f32 = TAU / 300f32;
const ZOOM_PER_SCROLL_PIXEL: f32 = 0.1f32;

struct Settings {
    vsync: bool,
}

pub(crate) struct Game {
    // Transformation matrices
    view: Matrix4<f32>,
    projection: Matrix4<f32>,

    pub ongoing: bool,

    triangle_draw: triangle::TrianglesDraw,
    cubes: Vec<cube::Cube>,

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

        let rotspeed = std::f32::consts::TAU * second_fraction * 0.3f32;
        let mut rng = rand::thread_rng();
        self.cubes.iter_mut().for_each(|c| {
            c.orientation = Orientation {
                pitch: c.orientation.pitch + rng.gen_range(-rotspeed..rotspeed),
                roll: c.orientation.roll + rng.gen_range(-rotspeed..rotspeed),
                yaw: c.orientation.yaw + rng.gen_range(-rotspeed..rotspeed),
            }
        });

        self.camera.location.x += extra_x;
        self.camera.location.y += extra_y;
        self.camera.location.z += extra_z;

        self.camera.orientation.pitch += extra_pitch;
        self.camera.orientation.yaw += extra_yaw;
        self.camera.orientation.roll += extra_roll;

        self.camera = self.camera.normalize();

        self.view = self.camera.view();

        self.refresh_view_projection();
    }

    pub(crate) fn draw(&self, gl: &gl::Gl) {
        self.cubes.iter().for_each(|cube| {
            let (model_translation, model_rotation) = &cube.model();
            self.triangle_draw.draw(
                &gl,
                &cube.verticies,
                &model_translation,
                &model_rotation,
            )
        });
    }

    pub fn new(
        res: Resources,
        gl: &gl::Gl,
        initial_time: u64,
        timer_frequency: u64,
        tick_length_us: u64,
        video_subsystem: sdl2::VideoSubsystem,
        aspect: f32,
    ) -> Result<Game, failure::Error> {
        let triangle_draw = triangle::TrianglesDraw::new(&res, &gl)?;

        let orientation: Orientation = Orientation {
            pitch: 0f32,
            roll: 0f32,
            yaw: 0f32,
        };

        let img = image::open("/home/omer/Desktop/bw.png").unwrap();

        let mut cbs = vec![];

        let (w, h) = img.dimensions();

        for i in 0..w {
            for j in 0..h {
                if j > 200 {
                    break;
                }

                cbs.push(cube::Cube::new(
                    (0f32 + (i as f32 * 0.6f32), 0f32 + (j as f32 * 0.6f32), 0f32).into(),
                    orientation,
                    (
                        (img.get_pixel(i, j).to_rgb()[0] as f32) / 255f32,
                        (img.get_pixel(i, j).to_rgb()[1] as f32) / 255f32,
                        (img.get_pixel(i, j).to_rgb()[2] as f32) / 255f32,
                    ),
                ));
            }

            if i > 200 {
                break;
            }
        }

        let camera = Camera {
            location: Location {
                x: 0f32,
                y: 0f32,
                z: 2f32,
            },
            orientation: Orientation {
                pitch: 0f32,
                roll: 0f32,
                yaw: 0f32,
            },
        };

        let mut game = Game {
            projection: perspective(aspect),
            view: camera.view(),

            ongoing: true,

            key_map: init_key_map(),
            triangle_draw,
            cubes: cbs,

            // Default rotation speed
            roll_per_second: 0f32,
            yaw_per_second: 0f32,
            pitch_per_second: 0f32,

            // Default movement speed
            x_per_second: 0f32,
            y_per_second: 0f32,
            z_per_second: 0f32,

            camera,

            video_subsystem,

            settings: Settings { vsync: false },
            game_time: GameTime::new(timer_frequency, tick_length_us, initial_time),
            key_stack: KeyStack::new(),
            mouse_down: false,
        };

        game.enable_vsync();
        game.set_aspect_ratio(aspect);
        game.refresh_view_projection();

        Ok(game)
    }

    pub fn refresh_view_projection(&mut self) {
        self.triangle_draw.set_view_projection(&self.view, &self.projection);
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.projection = perspective(aspect);
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

        if normalized.is_pressed(GameKey::Forward) {
            self.y_per_second = speed;
        } else if normalized.is_pressed(GameKey::Backwards) {
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

        if normalized.is_pressed(GameKey::Forward) {
            self.z_per_second = -speed;
        } else if normalized.is_pressed(GameKey::Backwards) {
            self.z_per_second = speed;
        } else {
            self.z_per_second = 0f32;
        }

        if normalized.is_pressed(GameKey::Right) {
            self.x_per_second = speed;
        } else if normalized.is_pressed(GameKey::Left) {
            self.x_per_second = -speed;
        } else {
            self.x_per_second = 0f32;
        }
    }

    pub fn keyboard_handler(&mut self) {
        let normalized = self.key_stack.normalize();

        if normalized.is_pressed(GameKey::VsyncToggle) {
            self.key_stack = self.key_stack.depress(GameKey::VsyncToggle);
            self.toggle_vsync();
        }

        if normalized.is_pressed(GameKey::Quit) {
            self.key_stack = self.key_stack.depress(GameKey::Quit);
            self.ongoing = false;
        }

        if normalized.is_pressed(GameKey::CameraModifier) {
            self.handle_movement(normalized);
        } else {
            self.handle_rotation(normalized);
        }
    }

    pub fn mouse_moved(&mut self, movement: MouseMovement) {
        let x_diff = SPIN_PER_MOUSE_PIXEL * (movement.0 as f32);
        let y_diff = SPIN_PER_MOUSE_PIXEL * (movement.1 as f32);
        if self.key_stack.normalize().is_pressed(GameKey::RollModifier) {
            self.camera.orientation.roll -= x_diff;
        } else {
            self.camera.orientation.yaw -= x_diff;
        }

        self.camera.orientation.pitch -= y_diff;
    }

    pub fn mouse_scrolled(&mut self, movement: MouseWheelDirection, _x: i32, y: i32) {
        self.camera.location.z += (match movement {
            MouseWheelDirection::Normal => -y as f32,
            MouseWheelDirection::Flipped => y as f32,
            MouseWheelDirection::Unknown(..) => 0f32,
        }) * ZOOM_PER_SCROLL_PIXEL
    }

    pub fn input_handler(&mut self, event: sdl2::event::Event) {
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => self.mouse_down = true,
            sdl2::event::Event::MouseButtonUp { .. } => self.mouse_down = false,
            sdl2::event::Event::MouseMotion { xrel, yrel, .. } => self.mouse_moved((xrel, yrel)),
            sdl2::event::Event::MouseWheel {
                direction, x, y, ..
            } => self.mouse_scrolled(direction, x, y),
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
