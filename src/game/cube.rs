use nalgebra::{Matrix4, Rotation3, Translation3, Vector3, Vector4};

use crate::primitives::spatial::{Location, Orientation};
use crate::triangle::{Triangle, Vertex};

pub(crate) struct Cube {
    triangles: Vec<Triangle>,
    pub location: Location,
    pub orientation: Orientation,

    pub verticies: Vec<Vertex>,
}

impl Cube {
    fn refresh_verticies(&mut self) {
        self.verticies = self.triangles.iter().flat_map(Triangle::vertices).collect();
    }

    pub(crate) fn model(&self) -> (Matrix4<f32>, Matrix4<f32>) {
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

        (translation, rotation)
    }

    pub(crate) fn new(location: Location, orientation: Orientation, color: Vector4<f32>) -> Self {
        let mut triangles = vec![];

        let alpha = 1f32;

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                clr: (0.9, 0.2, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.9, 0.1, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.0).into(),
                clr: (0.9, 0.3, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                clr: (0.9, 0.2, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.9, 0.1, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.9, 0.3, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                clr: (0.2, 0.9, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.0, 0.5).into(),
                clr: (0.1, 0.9, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.5).into(),
                clr: (0.3, 0.9, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                clr: (0.2, 0.9, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.1, 0.9, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.5).into(),
                clr: (0.3, 0.9, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                clr: (0.2, 0.2, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.0).into(),
                clr: (0.1, 0.1, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.5).into(),
                clr: (0.3, 0.3, 0.9, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.0).into(),
                clr: (0.2, 0.2, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.0, 0.5).into(),
                clr: (0.1, 0.1, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.5).into(),
                clr: (0.3, 0.3, 0.9, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.9, 0.9, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.9, 0.9, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.9, 0.9, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.5, 0.0).into(),
                clr: (0.9, 0.9, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.5).into(),
                clr: (0.9, 0.9, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.9, 0.9, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.5, 0.0, 0.0).into(),
                clr: (0.2, 0.2, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.1, 0.1, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.5).into(),
                clr: (0.3, 0.3, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.2, 0.2, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                clr: (0.1, 0.1, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.5).into(),
                clr: (0.3, 0.3, 0.5, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.5).into(),
                clr: (0.2, 0.5, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.5).into(),
                clr: (0.1, 0.5, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.3, 0.5, 0.9, alpha).into(),
            },
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.5).into(),
                clr: (0.2, 0.5, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.5).into(),
                clr: (0.1, 0.5, 0.9, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.3, 0.5, 0.9, alpha).into(),
            },
        ));

        let mut cube = Cube {
            triangles: triangles
                .iter()
                .map(|t| Triangle {
                    a: Vertex {
                        pos: t.a.pos,
                        clr: t.a.clr * color,
                    },
                    b: Vertex {
                        pos: t.b.pos,
                        clr: t.b.clr * color,
                    },
                    c: Vertex {
                        pos: t.c.pos,
                        clr: t.c.clr * color,
                    },
                })
                .collect(),
            location,
            orientation,
            verticies: vec![],
        };

        cube.refresh_verticies();

        cube
    }
}
