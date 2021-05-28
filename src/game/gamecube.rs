use crate::models::cube::Cube;
use crate::models::world_model::{Model, Spatial};
use nalgebra::Matrix4;
use crate::models::suzanne::Suzanne;

pub(crate) struct GameCube {
    pub(crate) cube: Suzanne,
    pub(crate) spatial: Spatial,
}

impl GameCube {
    pub fn new(spatial: Spatial, cube: Suzanne) -> Self {
        GameCube { cube, spatial }
    }
}

impl Model for GameCube {
    fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>) {
        self.spatial.model()
    }
}
