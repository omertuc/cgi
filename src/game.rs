use std::collections::HashSet;
use std::f32::consts::TAU;

use sdl2::keyboard::Scancode;

use crate::triangle;
use crate::resources::Resources;

const SPIN_PER_SECOND: f32 = TAU / 2f32;
const US_PER_SECOND: u64 = 1_000_000;

struct Settings {
    vsync: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GameKey {
    RollModifier,
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum GameKeyGroup {
    Horizontal,
    Vertical,
    Modifiers,
}

impl GameKey {
    fn groups(self) -> HashSet<GameKeyGroup> {
        match self {
            GameKey::RollModifier => { [GameKeyGroup::Modifiers] }
            GameKey::Right => { [GameKeyGroup::Horizontal] }
            GameKey::Left => { [GameKeyGroup::Horizontal] }
            GameKey::Up => { [GameKeyGroup::Vertical] }
            GameKey::Down => { [GameKeyGroup::Vertical] }
        }.iter().copied().collect()
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
struct KeyStack {
    stack: Vec<GameKey>,
}

impl KeyStack {
    fn new() -> KeyStack {
        KeyStack {
            stack: vec![],
        }
    }

    fn press(&self, key: GameKey) -> KeyStack {
        if self.is_pressed(key) {
            self.clone()
        } else {
            let mut new = self.normalize().clone();
            new.stack.push(key);
            new.into()
        }
    }

    fn depress(&self, key: GameKey) -> KeyStack {
        self.stack.clone().into_iter().filter(
            |other| &key != other).collect::<Vec<GameKey>>().into()
    }

    fn normalize(&self) -> KeyStack {
        if self.stack.len() == 0 {
            return KeyStack::new()
        }

        let mut encountered_groups = vec![].into_iter().collect();

        let mut new = KeyStack::new();

        for i in (0..self.stack.len()).rev() {
            let current = self.stack[i];
            let groups = current.groups();
            if groups.intersection(&encountered_groups).count() != 0 {
                continue
            }

            for group in groups {
                encountered_groups.insert(group);
            }

            new.stack.push(current)
        }

        new
    }

    fn is_normalized_pressed(&self, key: GameKey) -> bool {
        self.normalize().stack.contains(&key)
    }

    fn is_pressed(&self, key: GameKey) -> bool {
        self.stack.contains(&key)
    }
}

impl From<Vec<GameKey>> for KeyStack {
    fn from(other_vec: Vec<GameKey>) -> KeyStack {
        KeyStack {
            stack: other_vec
        }
    }
}

pub(crate) struct Game {
    triangle_draw: triangle::TrianglesDraw,
    triangles: Vec<triangle::Triangle>,

    // controls
    key_stack: KeyStack,

    // movement
    roll: f32,
    yaw: f32,
    pitch: f32,

    // sdl
    video_subsystem: sdl2::VideoSubsystem,

    // Time measurements
    previous_timer: u64,
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

        let triangles: Vec<triangle::Triangle> = (0..triangle_count).into_iter().map(
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
            tick_length_counter: (counter_per_us * tick_length_us),
            previous_timer: initial_time,
            partial_tick_counter: 0,
            tick_second_ratio: (tick_length_us as f32) / (US_PER_SECOND as f32),
            video_subsystem,
            settings: Settings {
                vsync: false,
            },
            key_stack: KeyStack::new(),
        };

        game.disable_vsync();

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

        if self.key_stack.is_normalized_pressed(GameKey::Down) {
            self.pitch = -speed;
        } else if self.key_stack.is_normalized_pressed(GameKey::Up) {
            self.pitch = speed;
        } else {
            self.pitch = 0f32;
        }

        if self.key_stack.is_normalized_pressed(GameKey::Right) {
            if self.key_stack.is_normalized_pressed(GameKey::RollModifier) {
                self.roll = speed;
            } else {
                self.yaw = speed;
            }
        } else if self.key_stack.is_normalized_pressed(GameKey::Left) {
            if self.key_stack.is_normalized_pressed(GameKey::RollModifier) {
                self.roll = -speed;
            } else {
                self.yaw = -speed;
            }
        } else {
            self.roll = 0f32;
            self.yaw = 0f32;
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
                ..
            } => {
                self.key_stack = if left_keys.contains(&code) {
                    self.key_stack.press(GameKey::Left)
                } else if right_keys.contains(&code) {
                    self.key_stack.press(GameKey::Right)
                } else if up_keys.contains(&code) {
                    self.key_stack.press(GameKey::Up)
                } else if down_keys.contains(&code) {
                    self.key_stack.press(GameKey::Down)
                } else if roll_modifier.contains(&code) {
                    self.key_stack.press(GameKey::RollModifier)
                } else {
                    self.key_stack.clone()
                }
            }
            sdl2::event::Event::KeyUp {
                scancode: Option::Some(code),
                ..
            } => {
                self.key_stack = if left_keys.contains(&code) {
                    self.key_stack.depress(GameKey::Left)
                } else if right_keys.contains(&code) {
                    self.key_stack.depress(GameKey::Right)
                } else if up_keys.contains(&code) {
                    self.key_stack.depress(GameKey::Up)
                } else if down_keys.contains(&code) {
                    self.key_stack.depress(GameKey::Down)
                } else if roll_modifier.contains(&code) {
                    self.key_stack.depress(GameKey::RollModifier)
                } else {
                    self.key_stack.clone()
                };

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
