use crate::fwd::{Vertex3D};

use nalgebra::Vector3;

#[derive(Clone)]
pub struct VertexInfo {
    pub vtx: Vertex3D,
    pub faces: Vec<usize>
}

#[derive(Copy, Clone)]
pub struct FaceInfo {
    pub vertices: [usize; 3],
    // pub normal: Vector3<f32>  todo: add normal
}

// This mesh repr is a mesh-vertex representation
#[derive(Clone)]
pub struct Mesh {
    pub faces: Vec<FaceInfo>,
    pub vertices: Vec<VertexInfo>
}
