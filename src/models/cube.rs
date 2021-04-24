use nalgebra::Vector4;

use crate::primitives::light::consts::WHITE;
use crate::primitives::light::Color;
use crate::primitives::spatial::Location;
use crate::primitives::triangle::{Triangle, Vertex, VertexData};
use std::convert::TryInto;

pub(crate) struct Cube {
    pub verticies: Vec<VertexData>,
}

impl Cube {
    pub(crate) fn new(color: Color) -> Self {
        let alpha = 1f32;

        let positions = [
            [(0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (1.0, 0.0, 0.0)],
            [(0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 0.0)],
            [(0.0, 0.0, 0.0), (0.0, 0.0, 1.0), (0.0, 1.0, 1.0)],
            [(0.0, 1.0, 0.0), (0.0, 0.0, 0.0), (0.0, 1.0, 1.0)],
            [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 1.0)],
            [(0.0, 0.0, 1.0), (0.0, 0.0, 0.0), (1.0, 0.0, 1.0)],
            [(1.0, 1.0, 0.0), (0.0, 1.0, 0.0), (1.0, 1.0, 1.0)],
            [(0.0, 1.0, 0.0), (0.0, 1.0, 1.0), (1.0, 1.0, 1.0)],
            [(1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (1.0, 0.0, 1.0)],
            [(1.0, 1.0, 0.0), (1.0, 1.0, 1.0), (1.0, 0.0, 1.0)],
            [(0.0, 0.0, 1.0), (1.0, 0.0, 1.0), (1.0, 1.0, 1.0)],
            [(0.0, 1.0, 1.0), (0.0, 0.0, 1.0), (1.0, 1.0, 1.0)],
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

        let offset = 0.5;

        let triangles: Vec<Triangle> = positions
            .iter()
            .map(|triangle_verticies| {
                triangle_verticies
                    .iter()
                    .map(|v| Location::from(*v) - offset)
            })
            .zip(colors)
            .map(|(p, _c)| {
                Triangle::new(
                    p.map(|v| Vertex {
                        pos: v,
                        clr: Vector4::<f32>::new(v.x, v.y, v.z, 1.0)
                            .component_mul(&color_vec)
                            .as_slice()
                            .into(),
                    })
                    .collect::<Vec<Vertex>>()
                    .try_into()
                    .unwrap(),
                )
            })
            .collect();

        let cube = Cube {
            verticies: triangles.iter().flat_map(Triangle::vertices).collect(),
        };

        cube
    }
}
