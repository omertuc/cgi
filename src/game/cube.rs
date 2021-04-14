use nalgebra::{Matrix4, Rotation3, Translation3, Vector3, Vector4};

use crate::primitives::spatial::{Location, Orientation};
use crate::triangle::{Triangle, Vertex, VertexData};

pub(crate) struct Cube {
    triangles: Vec<Triangle>,
    pub location: Location,
    pub orientation: Orientation,
    pub scale: f32,

    pub verticies: Vec<VertexData>,
}

impl Cube {
    fn refresh_verticies(&mut self) {
        self.verticies = self.triangles.iter().flat_map(Triangle::vertices).collect();
    }

    pub(crate) fn model(&self) -> (f32, Matrix4<f32>, Matrix4<f32>) {
        let rotation = Rotation3::from_euler_angles(
            // TODO: for some reason these make more sense when roll is pitch,
            // pitch is yaw, and yaw is roll. Should probably investigate why.
            self.orientation.pitch,
            self.orientation.yaw,
            self.orientation.roll,
        )
        .to_homogeneous();

        let translation = Translation3::from(Vector3::new(
            self.location.x,
            self.location.y,
            self.location.z,
        ))
        .to_homogeneous();

        (self.scale, translation, rotation)
    }

    pub(crate) fn new(
        location: Location,
        orientation: Orientation,
        scale: f32,
        color: Vector4<f32>,
    ) -> Self {
        let alpha = 1f32;

        let positions = vec![
            ((0.5, 0.5, 0.0), (0.0, 0.0, 0.0), (0.5, 0.0, 0.0)),
            ((0.0, 0.5, 0.0), (0.0, 0.0, 0.0), (0.5, 0.5, 0.0)),
            ((0.0, 0.0, 0.5), (0.0, 0.0, 0.0), (0.0, 0.5, 0.5)),
            ((0.0, 0.0, 0.0), (0.0, 0.5, 0.0), (0.0, 0.5, 0.5)),
            ((0.5, 0.0, 0.0), (0.0, 0.0, 0.0), (0.5, 0.0, 0.5)),
            ((0.0, 0.0, 0.0), (0.0, 0.0, 0.5), (0.5, 0.0, 0.5)),
            ((0.5, 0.5, 0.0), (0.0, 0.5, 0.0), (0.5, 0.5, 0.5)),
            ((0.0, 0.5, 0.0), (0.0, 0.5, 0.5), (0.5, 0.5, 0.5)),
            ((0.5, 0.0, 0.0), (0.5, 0.5, 0.0), (0.5, 0.0, 0.5)),
            ((0.5, 0.5, 0.0), (0.5, 0.5, 0.5), (0.5, 0.0, 0.5)),
            ((0.0, 0.0, 0.5), (0.5, 0.0, 0.5), (0.5, 0.5, 0.5)),
            ((0.0, 0.5, 0.5), (0.0, 0.0, 0.5), (0.5, 0.5, 0.5)),
        ];

        let colors = vec![
            (
                (0.9, 0.1, 0.3, alpha),
                (0.9, 0.2, 0.4, alpha),
                (0.9, 0.3, 0.5, alpha),
            ),
            (
                (0.9, 0.1, 0.3, alpha),
                (0.9, 0.2, 0.4, alpha),
                (0.9, 0.3, 0.5, alpha),
            ),
            (
                (0.1, 0.9, 0.3, alpha),
                (0.2, 0.9, 0.4, alpha),
                (0.3, 0.9, 0.5, alpha),
            ),
            (
                (0.2, 0.9, 0.4, alpha),
                (0.1, 0.9, 0.3, alpha),
                (0.3, 0.9, 0.5, alpha),
            ),
            (
                (0.1, 0.1, 0.9, alpha),
                (0.2, 0.2, 0.9, alpha),
                (0.3, 0.3, 0.9, alpha),
            ),
            (
                (0.2, 0.2, 0.9, alpha),
                (0.1, 0.1, 0.9, alpha),
                (0.3, 0.3, 0.9, alpha),
            ),
            (
                (0.9, 0.9, 0.3, alpha),
                (0.9, 0.9, 0.4, alpha),
                (0.9, 0.9, 0.5, alpha),
            ),
            (
                (0.9, 0.9, 0.4, alpha),
                (0.9, 0.9, 0.3, alpha),
                (0.9, 0.9, 0.5, alpha),
            ),
            (
                (0.2, 0.2, 0.4, alpha),
                (0.1, 0.1, 0.3, alpha),
                (0.3, 0.3, 0.5, alpha),
            ),
            (
                (0.1, 0.1, 0.3, alpha),
                (0.2, 0.2, 0.4, alpha),
                (0.3, 0.3, 0.5, alpha),
            ),
            (
                (0.2, 0.5, 0.9, alpha),
                (0.1, 0.5, 0.9, alpha),
                (0.3, 0.5, 0.9, alpha),
            ),
            (
                (0.1, 0.5, 0.9, alpha),
                (0.2, 0.5, 0.9, alpha),
                (0.3, 0.5, 0.9, alpha),
            ),
        ];

        let mut cube = Cube {
            triangles: positions
                .iter()
                .zip(colors)
                .map(|(p, c)| {
                    Triangle::new(
                        Vertex {
                            pos: p.0.into(),
                            clr: Vector4::<f32>::new(c.0 .0, c.0 .1, c.0 .2, c.0 .3)
                                .component_mul(&color)
                                .as_slice()
                                .into(),
                        },
                        Vertex {
                            pos: p.1.into(),
                            clr: Vector4::<f32>::new(c.1 .0, c.1 .1, c.1 .2, c.1 .3)
                                .component_mul(&color)
                                .as_slice()
                                .into(),
                        },
                        Vertex {
                            pos: p.2.into(),
                            clr: Vector4::<f32>::new(c.2 .0, c.2 .1, c.2 .2, c.2 .3)
                                .component_mul(&color)
                                .as_slice()
                                .into(),
                        },
                    )
                })
                .collect(),
            location,
            orientation,
            scale,
            verticies: vec![],
        };

        cube.refresh_verticies();

        cube
    }
}
