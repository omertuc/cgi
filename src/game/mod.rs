use std::f32::consts::TAU;

use image::{GenericImageView, Pixel};
use nalgebra::{Matrix4, Vector4};
use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::mouse::MouseWheelDirection;

use controls::GameKey;
use controls::KeyMap;

use crate::game::controls::{GameKeyStack, init_key_map};
use crate::models::cube::Cube;
use crate::models::world_model::{Model, Spatial};
use crate::primitives::camera::Camera;
use crate::primitives::input::{KeyStack, MouseMovement};
use crate::primitives::light::Color;
use crate::primitives::object_draw::ObjectsDraw;
use crate::primitives::projection::perspective;
use crate::primitives::spatial::{Location, Orientation};
use crate::primitives::spotlight::{spot_radius_to_cube_scale, Spotlight};
use crate::primitives::spotlight_draw::SpotlightDraw;
use crate::primitives::time::GameTime;
use crate::primitives::triangle::VertexData;
use crate::resources::Resources;

mod controls;

const MOVEMENT_PER_SECOND: f32 = 10f32;
const SPIN_PER_MOUSE_PIXEL: f32 = TAU / 300f32;
const ZOOM_PER_SCROLL_PIXEL: f32 = 0.1f32;
const RUN_MULTIPLIER: f32 = 10f32;
const WALK_MULTIPLIER: f32 = 0.1f32;
const LIGHT_SPIN_SPEED: f32 = TAU;

struct Settings {
    vsync: bool,
}

struct GameCube {
    cube: Cube,
    spatial: Spatial,
}

impl GameCube {
    fn new(spatial: Spatial, cube: Cube) -> Self {
        GameCube {
            cube,
            spatial,
        }
    }
}

impl Model for GameCube {
    fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>) {
        self.spatial.model()
    }
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

struct GameLight {
    spotlight: Spotlight,
    angle: f32,
    center: Location,
    location: Location,
    spin_radius: f32,
}

impl Model for GameLight {
    fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>) {
        Spatial {
            location: self.location,
            orientation: Orientation::default(),
            scale: spot_radius_to_cube_scale(self.spotlight.spot_radius),
        }.model()
    }
}

impl GameLight {
    fn location_from_angle(center: Location, angle: f32, spin_radius: f32) -> Location {
        center + Location {
            x: center.x + spin_radius * angle.cos(),
            y: center.y + spin_radius * angle.sin(),
            z: center.z,
        }
    }

    fn refresh_location(&mut self) {
        self.location = Self::location_from_angle(self.center, self.angle, self.spin_radius);
    }

    fn new(angle: f32, center: Location, spin_radius: f32, spotlight: Spotlight) -> Self {
        GameLight {
            spotlight,
            angle,
            center,
            spin_radius,
            location: Self::location_from_angle(center, angle, spin_radius)
        }
    }

    fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
        self.refresh_location()
    }
}

impl Game {
    pub(crate) fn process(&mut self, timer: u64) {
        let second_fraction =
            (self.game_time.update_ticks(timer) as f32) * self.game_time.tick_second_ratio;

        self.apply_camera_rotations(second_fraction);
        self.apply_camera_movement(second_fraction);
        self.camera = self.camera.normalize();

        self.wiggle_cubes(second_fraction);
        self.spin_lights(second_fraction);
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
        let rotspeed = std::f32::consts::TAU * second_fraction * 1.11f32;
        let movspeed = second_fraction * 3.11f32;
        let scalespeed = second_fraction * 1.8f32;

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

    fn spin_lights(&mut self, second_fraction: f32) {
        self.gamelights.iter_mut().for_each(|gamelight| {
            gamelight.set_angle((gamelight.angle + second_fraction * LIGHT_SPIN_SPEED) % TAU);
        });
    }

    pub(crate) fn draw(&self, gl: &gl::Gl) {
        let (view_translation, view_rotation) = self.camera.view();

        self.objects_draw
            .set_view(&view_translation, &view_rotation);

        self.objects_draw.set_spotlights(self.gamelights
            .iter()
            .map(|gamelight| (&gamelight.spotlight, &gamelight.location)));

        self.spotslights_draw
            .set_view(&view_translation, &view_rotation);

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
        vec![
            GameLight::new(
                0.0f32,
                Location {
                    x: 100.0,
                    y: 100.0,
                    z: 0.0,
                },
                100.0,
                Spotlight::new(
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    90.0,
                )),
            GameLight::new(
                TAU * 2.0 / 3.0,
                Location {
                    x: 100.0,
                    y: 100.0,
                    z: 0.0,
                },
                100.0,
                Spotlight::new(
                    Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    90.0,
                )),
            GameLight::new(
                TAU / 3.0,
                Location {
                    x: 100.0,
                    y: 100.0,
                    z: 0.0,
                },
                100.0,
                Spotlight::new(
                    Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    90.0,
                )),
            GameLight::new(
                TAU / 3.0,
                Location {
                    x: 100.0,
                    y: 100.0,
                    z: 0.0,
                },
                10.0,
                Spotlight::new(
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    90.0,
                )),
        ]
    }

    fn get_cubes() -> Vec<GameCube> {
        let img = image::load_from_memory(include_bytes!("rs.png")).unwrap();

        let mut cbs = vec![];

        let (w, h) = img.dimensions();

        for i in 0..w {
            for j in 0..h {
                let spatial =
                    Spatial::new(
                        Location {
                            x: 0f32 + (i as f32 * 1.5f32),
                            y: 0f32 + (j as f32 * 1.5f32),
                            z: 0.0,
                        },
                        Orientation::default(),
                        0.02 * i as f32,
                    );


                let color = Color {
                    r: (img.get_pixel(i, h - j - 1).to_rgb()[0] as f32) / 255f32,
                    g: (img.get_pixel(i, h - j - 1).to_rgb()[1] as f32) / 255f32,
                    b: (img.get_pixel(i, h - j - 1).to_rgb()[2] as f32) / 255f32,
                    a: 1.0,
                };

                let cube = Cube::new(color);

                cbs.push(GameCube::new(spatial, cube));
            }
        }

        cbs
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
        let img_verticies: Vec<VertexData> =
            img_cubes.iter().flat_map(|c| c.cube.verticies.clone()).collect();

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
