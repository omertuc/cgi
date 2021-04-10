use crate::triangle::{Triangle, Vertex};

use crate::primitives::spatial::{Location, Orientation};

pub(crate) struct Cube {
    pub triangles: Vec<Triangle>,
}

impl Cube {
    pub(crate) fn new(location: Location, orientation: Orientation) -> Self {
        let mut triangles = vec![];

        let alpha = 0.5;

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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
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
            location,
            orientation,
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.5).into(),
                clr: (0.2, 0.2, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.0, 0.5).into(),
                clr: (0.1, 0.1, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.3, 0.3, 0.5, alpha).into(),
            },
            location,
            orientation,
        ));

        triangles.push(Triangle::new(
            Vertex {
                pos: (0.0, 0.0, 0.5).into(),
                clr: (0.2, 0.2, 0.4, alpha).into(),
            },
            Vertex {
                pos: (0.0, 0.5, 0.5).into(),
                clr: (0.1, 0.1, 0.3, alpha).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.5).into(),
                clr: (0.3, 0.3, 0.5, alpha).into(),
            },
            location,
            orientation,
        ));

        Cube { triangles }
    }
}
