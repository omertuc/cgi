use nalgebra::{Matrix4, Rotation3, Translation3, Vector3, Vector4};

use crate::primitives::spatial::{Location, Orientation};
use crate::primitives::triangle::{Triangle, Vertex, VertexData};

pub(crate) struct Cube {
    pub location: Location,
    pub orientation: Orientation,
    pub scale: f32,
    pub verticies: Vec<VertexData>,
}

impl Cube {
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

        let triangles: Vec<Triangle> = positions
            .iter()
            .zip(colors)
            .map(|(p, _c)| {
                Triangle::new(
                    Vertex {
                        pos: p.0.into(),
                        clr: Vector4::<f32>::new(p.0 .0, p.0 .1, p.0 .2, 1.0)
                            .component_mul(&color)
                            .as_slice()
                            .into(),
                    },
                    Vertex {
                        pos: p.1.into(),
                        clr: Vector4::<f32>::new(p.1 .0, p.1 .1, p.1 .2, 1.0)
                            .component_mul(&color)
                            .as_slice()
                            .into(),
                    },
                    Vertex {
                        pos: p.2.into(),
                        clr: Vector4::<f32>::new(p.2 .0, p.2 .1, p.2 .2, 1.0)
                            .component_mul(&color)
                            .as_slice()
                            .into(),
                    },
                )
            })
            .collect();

        let cube = Cube {
            location,
            orientation,
            scale,
            verticies: triangles.iter().flat_map(Triangle::vertices).collect(),
        };

        cube
    }
}
