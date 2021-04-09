use crate::game::{Location, Orientation};

#[derive(Debug)]
pub(crate) struct Camera {
    pub location: Location,
    pub orientation: Orientation,
}
