use std::f32::consts::TAU;

use image::{GenericImageView, Pixel};
use nalgebra::Vector4;
use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::mouse::MouseWheelDirection;

use controls::GameKey;
use controls::KeyMap;

use crate::game::controls::{init_key_map, GameKeyStack};
use crate::game::gamecube::GameCube;
use crate::game::gamelight::GameLight;
use crate::models::cube::Cube;
use crate::models::suzanne::Suzanne;
use crate::models::world_model::{Model, Spatial};
use crate::primitives::camera::Camera;
use crate::primitives::input::{KeyStack, MouseMovement};
use crate::primitives::light::consts::{BLUE, GREEN, RED, WHITE};
use crate::primitives::light::Color;
use crate::primitives::object_draw::ObjectsDraw;
use crate::primitives::projection::perspective;
use crate::primitives::spatial::{Location, Orientation};
use crate::primitives::spotlight::Spotlight;
use crate::primitives::spotlight_draw::SpotlightDraw;
use crate::primitives::time::GameTime;
use crate::primitives::triangle::VertexData;
use crate::resources::Resources;

mod controls;
mod gamecube;
mod gamelight;

const MOVEMENT_PER_SECOND: f32 = 10f32;
const SPIN_PER_MOUSE_PIXEL: f32 = TAU / 300f32;
const ZOOM_PER_SCROLL_PIXEL: f32 = 0.1f32;
const RUN_MULTIPLIER: f32 = 10f32;
const WALK_MULTIPLIER: f32 = 0.1f32;

struct Settings {
    vsync: bool,
}

pub(crate) struct Game {
    pub ongoing: bool,

    objects_draw: ObjectsDraw,
    spotslights_draw: SpotlightDraw,
    gamecubes: Vec<GameCube>,
    gamelights: Vec<GameLight>,

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
    rng: ThreadRng,
}

impl Game {
    pub(crate) fn process(&mut self, timer: u64) {
        let second_fraction =
            (self.game_time.update_ticks(timer) as f32) * self.game_time.tick_second_ratio;

        self.apply_camera_rotations(second_fraction);
        self.apply_camera_movement(second_fraction);
        self.camera = self.camera.normalize();

        self.wiggle_cubes(second_fraction);
        self.move_lights(second_fraction);
    }

    pub fn apply_camera_rotations(&mut self, second_fraction: f32) {
        self.camera.orientation.pitch += self.pitch_per_second * second_fraction;
        self.camera.orientation.yaw += self.yaw_per_second * second_fraction;
        self.camera.orientation.roll += self.roll_per_second * second_fraction;
    }

    pub fn apply_camera_movement(&mut self, second_fraction: f32) {
        self.camera.location.x += self.x_per_second * second_fraction;
        self.camera.location.y += self.y_per_second * second_fraction;
        self.camera.location.z += self.z_per_second * second_fraction;
    }

    fn wiggle_cubes(&mut self, second_fraction: f32) {
        let mut rng = self.rng.clone();
        let rotspeed = std::f32::consts::TAU * second_fraction * 0.0001;
        let movspeed = second_fraction * 1.1;
        let scalespeed = second_fraction * 9.1;

        self.gamecubes.iter_mut().for_each(|gamecube| {
            gamecube.spatial.orientation = Orientation {
                pitch: gamecube.spatial.orientation.pitch + rng.gen_range(0f32..rotspeed),
                roll: gamecube.spatial.orientation.roll + rng.gen_range(0f32..rotspeed),
                yaw: gamecube.spatial.orientation.yaw + rng.gen_range(0f32..rotspeed),
            };
            gamecube.spatial.location = Location {
                x: gamecube.spatial.location.x + rng.gen_range(-movspeed..movspeed),
                y: gamecube.spatial.location.y + rng.gen_range(-movspeed..movspeed),
                z: gamecube.spatial.location.z + rng.gen_range(-movspeed..movspeed),
            };
            gamecube.spatial.scale += rng.gen_range(-scalespeed..scalespeed);
        });

        self.rng = rng;
    }

    fn move_lights(&mut self, second_fraction: f32) {
        self.gamelights.iter_mut().for_each(|gamelight| {
            gamelight.set_angle((gamelight.angle + second_fraction * gamelight.spin_speed) % TAU);
            gamelight.set_location(Location::new(
                gamelight.center.x + gamelight.x_speed * second_fraction,
                gamelight.center.y + gamelight.y_speed * second_fraction,
                gamelight.center.z + gamelight.z_speed * second_fraction,
            ));
        });
    }

    pub(crate) fn draw(&self, gl: &gl::Gl) {
        let (view_rotation, view_translation, view_location) = self.camera.view();

        self.objects_draw
            .set_view(&view_rotation, &view_translation, &view_location);

        self.objects_draw.set_spotlights(
            self.gamelights
                .iter()
                .map(|gamelight| (&gamelight.spotlight, &gamelight.location)),
        );

        let num_vertices = self.gamecubes[0].cube.verticies.len();
        self.objects_draw.prepare_for_draws();
        self.gamecubes.iter().enumerate().for_each(|(i, cube)| {
            let (model_scale, model_translation, model_rotation) = cube.model();
            self.objects_draw.draw(
                &gl,
                model_scale,
                &model_translation,
                &model_rotation,
                num_vertices,
                num_vertices * i,
            );
        });

        self.spotslights_draw
            .set_view(&view_translation, &view_rotation);

        let num_vertices = self.gamelights[0].spotlight.cube.verticies.len();
        self.spotslights_draw.prepare_for_draws();
        self.gamelights
            .iter()
            .enumerate()
            .for_each(|(i, spotlight)| {
                let (model_scale, model_translation, model_rotation) = spotlight.model();

                self.spotslights_draw.set_solid_color(&Vector4::<f32>::new(
                    spotlight.spotlight.color.r,
                    spotlight.spotlight.color.g,
                    spotlight.spotlight.color.b,
                    spotlight.spotlight.color.a,
                ));

                self.spotslights_draw.draw(
                    &gl,
                    model_scale,
                    &model_translation,
                    &model_rotation,
                    num_vertices,
                    num_vertices * i,
                );
            });
    }

    fn get_lights() -> Vec<GameLight> {
        let spot_radius = 15.0;
        let spin_speed = TAU / 100.0;
        let z = 2.0;
        let center = Location { x: 0.0, y: 0.0, z };

        let mut game_lights = vec![];
        game_lights.push(GameLight::new(
            TAU * 0.0 / 3.0,
            Location::new(center.x, center.y, 10.0),
            20.0,
            TAU / 100.0,
            Spotlight::new(WHITE, 100.0),
        ));

        let step = 3;
        for i in (1..200).step_by(step) {
            let spin_radius = 1.0 * i as f32 / step as f32;
            let angle_offset = (TAU / 1.61803) * i as f32 / step as f32;
            game_lights.push(GameLight::new(
                TAU * 0.0 / 3.0 + angle_offset,
                center,
                spin_radius,
                spin_speed * i as f32 / step as f32,
                Spotlight::new(RED, spot_radius),
            ));
            game_lights.push(GameLight::new(
                TAU * 1.0 / 3.0 + angle_offset,
                center,
                spin_radius,
                spin_speed * i as f32 / step as f32,
                Spotlight::new(GREEN, spot_radius),
            ));
            game_lights.push(GameLight::new(
                TAU * 2.0 / 3.0 + angle_offset,
                center,
                spin_radius,
                spin_speed * i as f32 / step as f32,
                Spotlight::new(BLUE, spot_radius),
            ));
        }

        game_lights
    }

    fn get_cubes() -> Vec<GameCube> {
        let img = image::load_from_memory(include_bytes!("rs.png")).unwrap();

        let mut game_cubes = vec![];

        let (w, h) = img.dimensions();

        let step = 50;
        for i in (0..w).step_by(step) {
            for j in (0..h).step_by(step) {
                let spatial = Spatial::new(
                    Location {
                        x: 0f32 + (i as f32 / step as f32) * 3.3,
                        y: 0f32 + (j as f32 / step as f32) * 3.3,
                        z: 0.0,
                    },
                    Orientation::default(),
                    5.0,
                );

                let color = Color {
                    r: (img.get_pixel(i, h - j - 1).to_rgb()[0] as f32) / 255f32,
                    g: (img.get_pixel(i, h - j - 1).to_rgb()[1] as f32) / 255f32,
                    b: (img.get_pixel(i, h - j - 1).to_rgb()[2] as f32) / 255f32,
                    a: 1.0,
                };

                let cube = Suzanne::new(color);

                game_cubes.push(GameCube::new(spatial, cube));
            }
        }

        game_cubes
    }

    pub fn default_camera() -> Camera {
        Camera {
            location: Location {
                x: 130f32,
                y: 0f32,
                z: 130f32,
            },
            orientation: Orientation {
                pitch: TAU / 8.0,
                roll: 0f32,
                yaw: 0f32,
            },
        }
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
        let img_cubes = Self::get_cubes();
        let img_verticies: Vec<VertexData> = img_cubes
            .iter()
            .flat_map(|c| c.cube.verticies.clone())
            .collect();

        let gamelights = Self::get_lights();
        let spot_verticies: Vec<VertexData> = gamelights
            .iter()
            .flat_map(|g| g.spotlight.cube.verticies.clone())
            .collect();

        let spotlight_draw = SpotlightDraw::new(&res, &gl, spot_verticies)?;

        let mut game = Game {
            rng: rand::thread_rng(),

            ongoing: true,

            key_map: init_key_map(),
            objects_draw: ObjectsDraw::new(&res, &gl, img_verticies)?,
            spotslights_draw: spotlight_draw,
            gamecubes: img_cubes,
            gamelights,

            // Default rotation speed
            roll_per_second: 0f32,
            yaw_per_second: 0f32,
            pitch_per_second: 0f32,

            // Default movement speed
            x_per_second: 0f32,
            y_per_second: 0f32,
            z_per_second: 0f32,

            camera: Self::default_camera(),

            video_subsystem,

            settings: Settings { vsync: false },
            game_time: GameTime::new(timer_frequency, tick_length_us, initial_time),
            key_stack: KeyStack::new(),
            mouse_down: false,
        };

        game.enable_vsync();
        game.set_aspect_ratio(aspect);

        Ok(game)
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        let projection = &perspective(aspect);
        self.objects_draw.set_projection(projection);
        self.spotslights_draw.set_projection(projection);
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

    pub fn handle_keyboard_movement(&mut self, normalized: GameKeyStack) {
        let speed = MOVEMENT_PER_SECOND
            * if normalized.is_pressed(GameKey::Run) {
                RUN_MULTIPLIER
            } else if normalized.is_pressed(GameKey::Walk) {
                WALK_MULTIPLIER
            } else {
                1f32
            };

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

        self.handle_keyboard_movement(normalized);
    }

    pub fn mouse_moved(&mut self, movement: MouseMovement) {
        let x_diff = SPIN_PER_MOUSE_PIXEL * (movement.0 as f32);
        let y_diff = SPIN_PER_MOUSE_PIXEL * (movement.1 as f32);
        self.camera.orientation.yaw -= x_diff;
        self.camera.orientation.pitch -= y_diff;
    }

    pub fn mouse_scrolled(&mut self, movement: MouseWheelDirection, _x: i32, y: i32) {
        self.camera.location.y += (match movement {
            MouseWheelDirection::Normal => y as f32,
            MouseWheelDirection::Flipped => -y as f32,
            MouseWheelDirection::Unknown(..) => 0f32,
        }) * ZOOM_PER_SCROLL_PIXEL
            * if self.key_stack.normalize().is_pressed(GameKey::Run) {
                RUN_MULTIPLIER
            } else if self.key_stack.normalize().is_pressed(GameKey::Walk) {
                WALK_MULTIPLIER
            } else {
                1f32
            }
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
