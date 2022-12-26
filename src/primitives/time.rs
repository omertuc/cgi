const US_PER_SECOND: u64 = 1_000_000;

pub struct GameTime {
    pub previous_timer: u64,
    pub partial_tick_counter: u64,
    pub tick_length_counter: u64,
    pub tick_second_ratio: f32,
}

impl GameTime {
    pub fn new(timer_frequency: u64, tick_length_us: u64, initial_time: u64) -> GameTime {
        let counter_per_us: u64 = timer_frequency / US_PER_SECOND;

        GameTime {
            tick_length_counter: (counter_per_us * tick_length_us),
            previous_timer: initial_time,
            partial_tick_counter: 0,
            tick_second_ratio: (tick_length_us as f32) / (US_PER_SECOND as f32),
        }
    }

    pub fn update_ticks(&mut self, timer: u64) -> u64 {
        let time_passed_counter = self.partial_tick_counter + (timer - self.previous_timer);
        let ticks = time_passed_counter / self.tick_length_counter;
        self.partial_tick_counter = time_passed_counter % self.tick_length_counter;
        self.previous_timer = timer;

        ticks
    }
}
