use gl;
use nalgebra::{Vector3, Vector4};

use crate::primitives::light::Color;
use crate::primitives::spatial::Location;
use crate::render_gl::data;
use crate::render_gl::data::{f32_f32_f32, f32_f32_f32_f32};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexData {
    pub pos: data::f32_f32_f32,
    pub clr: data::u2_u10_u10_u10_rev_float,
    pub norm: data::f32_f32_f32,
}

impl VertexData {
    pub(crate) fn new(vertex: &Vertex, normal: Vector3<f32>) -> VertexData {
        VertexData {
            pos: vertex.pos.into(),
            clr: vertex.clr.into(),
            norm: normal.into(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Vertex {
    pub pos: Location,
    pub clr: Color,
}

#[derive(Debug, Clone)]
pub(crate) struct Triangle {
    pub a: VertexData,
    pub b: VertexData,
    pub c: VertexData,
}

impl Triangle {
    pub fn new(verticies: [Vertex; 3]) -> Triangle {
        let avec: Vector3<f32> = verticies[0].pos.into();
        let bvec: Vector3<f32> = verticies[1].pos.into();
        let cvec: Vector3<f32> = verticies[2].pos.into();

        let normal = (bvec - avec).cross(&(cvec - &avec));

        Triangle {
            a: VertexData::new(&verticies[0], normal),
            b: VertexData::new(&verticies[1], normal),
            c: VertexData::new(&verticies[2], normal),
        }
    }

    pub fn new_with_normals(verticies: [Vertex; 3], normal: Vector3<f32>) -> Triangle {
        Triangle {
            a: VertexData::new(&verticies[0], normal),
            b: VertexData::new(&verticies[1], normal),
            c: VertexData::new(&verticies[2], normal),
        }
    }

    pub fn vertices(&self) -> Vec<VertexData> {
        vec![self.a, self.b, self.c].into()
    }
}

impl From<Vector4<f32>> for f32_f32_f32_f32 {
    fn from(vector: Vector4<f32>) -> Self {
        (vector.x, vector.y, vector.z, vector.w).into()
    }
}

impl From<Vector3<f32>> for f32_f32_f32 {
    fn from(vector: Vector3<f32>) -> Self {
        (vector.x, vector.y, vector.z).into()
    }
}

impl From<f32_f32_f32_f32> for nalgebra::Vector4<f32> {
    fn from(f: f32_f32_f32_f32) -> Self {
        Vector4::new(f.d0, f.d1, f.d2, f.d3)
    }
}

impl From<f32_f32_f32_f32> for nalgebra::Vector3<f32> {
    fn from(f: f32_f32_f32_f32) -> Self {
        Vector3::new(f.d0, f.d1, f.d2)
    }
}
