use crate::primitives::spatial::{Location, Orientation};
use crate::triangle::{Triangle, Vertex};

pub(crate) struct Cube {
    triangles: Vec<Triangle>,
    pub location: Location,
    pub orientation: Orientation,
}

impl Cube {
    pub(crate) fn verticies(&self) -> Vec<Vertex> {
        self.triangles
            .clone()
            .iter()
            .flat_map(Triangle::vertices)
            .map(|v| v.oriented(self.orientation))
            .map(|v| v.translated(self.location))
            .collect()
    }

    pub(crate) fn new(location: Location, orientation: Orientation) -> Self {
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

        Cube {
            triangles,
            location,
            orientation,
        }
    }
}
