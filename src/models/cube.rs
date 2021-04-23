use nalgebra::Vector4;

use crate::primitives::light::Color;
use crate::primitives::triangle::{Triangle, Vertex, VertexData};

pub(crate) struct Cube {
    pub verticies: Vec<VertexData>,
}

impl Cube {
    pub(crate) fn new(
        color: Color,
    ) -> Self {
        let alpha = 1f32;

        let positions = vec![
            (
                (0.0, 0.0, 0.0),
                (0.5, 0.5, 0.0),
                (0.5, 0.0, 0.0),
            ),
            (
                (0.0, 0.0, 0.0),
                (0.0, 0.5, 0.0),
                (0.5, 0.5, 0.0),
            ),
            (
                (0.0, 0.0, 0.0),
                (0.0, 0.0, 0.5),
                (0.0, 0.5, 0.5),
            ),
            (
                (0.0, 0.5, 0.0),
                (0.0, 0.0, 0.0),
                (0.0, 0.5, 0.5),
            ),
            (
                (0.0, 0.0, 0.0),
                (0.5, 0.0, 0.0),
                (0.5, 0.0, 0.5),
            ),
            (
                (0.0, 0.0, 0.5),
                (0.0, 0.0, 0.0),
                (0.5, 0.0, 0.5),
            ),
            (
                (0.5, 0.5, 0.0),
                (0.0, 0.5, 0.0),
                (0.5, 0.5, 0.5),
            ),
            (
                (0.0, 0.5, 0.0),
                (0.0, 0.5, 0.5),
                (0.5, 0.5, 0.5),
            ),
            (
                (0.5, 0.0, 0.0),
                (0.5, 0.5, 0.0),
                (0.5, 0.0, 0.5),
            ),
            (
                (0.5, 0.5, 0.0),
                (0.5, 0.5, 0.5),
                (0.5, 0.0, 0.5),
            ),
            (
                (0.0, 0.0, 0.5),
                (0.5, 0.0, 0.5),
                (0.5, 0.5, 0.5),
            ),
            (
                (0.0, 0.5, 0.5),
                (0.0, 0.0, 0.5),
                (0.5, 0.5, 0.5),
            ),
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

        let color_vec = Vector4::new(color.r, color.g, color.b, color.a);

        let triangles: Vec<Triangle> = positions
            .iter()
            .zip(colors)
            .map(|(p, _c)| {
                Triangle::new(
                    Vertex {
                        pos: p.0.into(),
                        clr: Vector4::<f32>::new(p.0.0, p.0.1, p.0.2, 1.0)
                            .component_mul(&color_vec)
                            .as_slice()
                            .into(),
                    },
                    Vertex {
                        pos: p.1.into(),
                        clr: Vector4::<f32>::new(p.1.0, p.1.1, p.1.2, 1.0)
                            .component_mul(&color_vec)
                            .as_slice()
                            .into(),
                    },
                    Vertex {
                        pos: p.2.into(),
                        clr: Vector4::<f32>::new(p.2.0, p.2.1, p.2.2, 1.0)
                            .component_mul(&color_vec)
                            .as_slice()
                            .into(),
                    },
                )
            })
            .collect();

        let cube = Cube {
            verticies: triangles.iter().flat_map(Triangle::vertices).collect(),
        };

        cube
    }
}
