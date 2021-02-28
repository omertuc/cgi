use crate::resources::Resources;
use crate::triangle;

const SPIN_PER_TICK: f32 = 0.0001f32;

pub(crate) struct Game {
    triangle_draw: triangle::TrianglesDraw,
    triangles: Vec<triangle::Triangle>,
    spin_right: bool,
    spin_left: bool,
    timer_frequency: u64,
    previous_timer: u64,
    tick_length_us: u64,
    partial_tick_counter: u64,
    tick_length_counter: u64,
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

        if self.spin_right {
            self.triangles.iter_mut().for_each(|t| t.add_angle(SPIN_PER_TICK * ticks as f32));
        } else if self.spin_left {
            self.triangles.iter_mut().for_each(|t| t.add_angle(-SPIN_PER_TICK * ticks as f32));
        }
    }
}

impl Game {
    pub(crate) fn draw(&self, gl: &gl::Gl) {
        self.triangle_draw.draw(&gl, self.triangles.iter().flat_map(triangle::Triangle::vertices).collect());
    }
}

enum Command {
    SetSpin(f32),
    StopSpin,
}

impl Game {
    pub fn new(res: Resources, gl: &gl::Gl, initial_time: u64, freq: u64, tick_length: u64) -> Result<Game, failure::Error> {
        let triangle_draw = triangle::TrianglesDraw::new(&res, &gl)?;
        let triangle_count = 2;

        let mut triangles: Vec<triangle::Triangle> = (0..triangle_count).into_iter().map(
            |triangle_index| (triangle_index as f32) * (std::f32::consts::TAU / triangle_count as f32)
        ).map(
            |angle| triangle::Triangle::new(angle)
        ).collect();

        let us_per_second = 1_000_000;
        let counter_per_us: u64 = us_per_second / freq;

        Ok(Game {
            triangle_draw,
            triangles,
            spin_right: false,
            spin_left: false,
            tick_length_us: tick_length,
            tick_length_counter: (counter_per_us * tick_length),
            previous_timer: initial_time,
            timer_frequency: freq,
            partial_tick_counter: 0,
        })
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
                        self.spin_right = true;
                    }
                    sdl2::keyboard::Keycode::A => {
                        self.spin_left = true;
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
                    sdl2::keyboard::Keycode::D => {
                        self.spin_right = false;
                    }
                    sdl2::keyboard::Keycode::A => {
                        self.spin_left = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }
}
