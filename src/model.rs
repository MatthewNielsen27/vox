use std::sync::Arc;
use crate::fwd::{Vertex3D};

use nalgebra as na;

#[derive(Copy, Clone)]
pub struct VertexModel(pub Vertex3D);

#[derive(Copy, Clone)]
pub struct VertexWorld(pub Vertex3D);

impl VertexModel {
    pub fn to_world(&self, transform: &na::Isometry3<f32>) -> VertexWorld {
        VertexWorld(transform.transform_point(&self.0))
    }
}

#[derive(Clone)]
pub struct VertexInfo {
    pub vtx: VertexModel,
    pub faces: Vec<usize>
}

#[derive(Copy, Clone)]
pub struct FaceInfo {
    pub vertices: [usize; 3]
}

/// Defines a face-vertex mesh representation.
///
/// see: https://en.wikipedia.org/wiki/Polygon_mesh#Face-vertex_meshes
///
#[derive(Clone)]
pub struct Mesh {
    pub faces: Vec<FaceInfo>,
    pub vertices: Vec<VertexInfo>
}

impl Mesh {
    pub fn get_vertex(&self, i: usize) -> &VertexModel {
        &self.vertices[i].vtx
    }
}

#[derive(Clone)]
pub struct Model {
    pub mesh: Arc<Mesh>,
    pub transform: na::Isometry3<f32>
}

impl Model {
    pub fn triangles(&self) -> impl Iterator<Item = (&VertexModel,&VertexModel,&VertexModel)> {
        self.mesh.faces.iter().map(| face| {
            let v0 = &self.mesh.vertices[face.vertices[0]].vtx;
            let v1 = &self.mesh.vertices[face.vertices[1]].vtx;
            let v2 = &self.mesh.vertices[face.vertices[2]].vtx;
            (v0, v1, v2)
        })
    }

    pub fn triangles_world(&self) -> impl Iterator<Item = (VertexWorld,VertexWorld,VertexWorld)> + '_ {
        self.mesh.faces.iter().map(|face| {
            let v0 = &self.mesh.vertices[face.vertices[0]].vtx;
            let v1 = &self.mesh.vertices[face.vertices[1]].vtx;
            let v2 = &self.mesh.vertices[face.vertices[2]].vtx;
            (v0.to_world(&self.transform), v1.to_world(&self.transform), v2.to_world(&self.transform))
        })
    }
}
