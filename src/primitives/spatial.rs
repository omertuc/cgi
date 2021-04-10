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
