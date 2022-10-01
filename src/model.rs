use std::sync::Arc;
use crate::fwd::{Vertex3D};

use nalgebra::{Vector3, Quaternion, Matrix4};

#[derive(Clone)]
pub struct VertexInfo {
    pub vtx: Vertex3D,
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

#[derive(Copy, Clone)]
pub struct Transformation {
    // Orientation transformation is represented
    pub rotation: Option<Matrix4<f32>>,
    pub scale: Option<f32>,
    pub translation: Option<Vector3<f32>>
}

impl Mesh {
    pub fn get_vertex(&self, i: usize) -> &Vertex3D { &self.vertices[i].vtx }
}

impl Transformation {
    // Returns true if the transformation is empty
    pub fn is_unity(&self) -> bool {
        self.rotation.is_none()
        &&  self.scale.is_none()
        &&  self.translation.is_none()
    }

    // Returns combines the transformation
    pub fn get(&self) -> Option<Matrix4<f32>> {
        if self.is_unity() {
            return None;
        }

        let mut mat = nalgebra::Matrix4::new_scaling(self.scale.unwrap_or(1.0));

        if self.translation.is_some() {
            mat = mat.append_translation(&self.translation.unwrap());
        }

        if self.rotation.is_some() {
            mat = mat * self.rotation.unwrap();
        }

        Some(mat)
    }
}

#[derive(Clone)]
pub struct Model {
    pub mesh: Arc<Mesh>,
    pub transform: Transformation
}
