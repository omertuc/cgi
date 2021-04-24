use crate::models::cube::Cube;
use crate::models::world_model::{Model, Spatial};
use nalgebra::Matrix4;

pub(crate) struct GameCube {
    pub(crate) cube: Cube,
    pub(crate) spatial: Spatial,
}

impl GameCube {
    pub fn new(spatial: Spatial, cube: Cube) -> Self {
        GameCube { cube, spatial }
    }
}

impl Model for GameCube {
    fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>) {
        self.spatial.model()
    }
}
