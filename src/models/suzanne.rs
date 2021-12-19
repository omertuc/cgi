use std::convert::TryInto;

use nalgebra::{Vector3, Vector4};

use crate::primitives::light::consts::DARK_GRAY;
use crate::primitives::light::Color;
use crate::primitives::spatial::Location;
use crate::primitives::triangle::{Vertex, VertexData};
use std::mem::transmute;

pub(crate) struct Suzanne {
    pub verticies: Vec<VertexData>,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct VertexWithNormal {
    vx: f64,
    vy: f64,
    vz: f64,
    nx: f64,
    ny: f64,
    nz: f64,
}

impl Suzanne {
    pub(crate) fn new(color: Color) -> Self {
        let suzanne_cgi = include_bytes!("suzanne.cgi");

        let color_vec = Vector4::new(color.r, color.g, color.b, color.a);

        const VWN_SIZE: usize = std::mem::size_of::<VertexWithNormal>();

        let verticies: Vec<VertexData> = suzanne_cgi
            .chunks(VWN_SIZE)
            .map(|raw| unsafe {
                transmute::<[u8; VWN_SIZE], VertexWithNormal>(raw.try_into().unwrap())
            })
            .map(|vwn| {
                VertexData::new(
                    &Vertex {
                        pos: Location::new(vwn.vx as f32, vwn.vy as f32, vwn.vz as f32),
                        clr: DARK_GRAY,
                    },
                    Vector3::new(vwn.nx as f32, vwn.ny as f32, vwn.nz as f32),
                )
            })
            .collect();

        dbg!(verticies[0]);
        dbg!(verticies[1000]);

        let cube = Suzanne { verticies };

        cube
    }
}
