use std::sync::Arc;
use crate::fwd::{Vertex3};

use nalgebra as na;

#[derive(Copy, Clone)]
pub struct VertexModel(pub Vertex3);

#[derive(Copy, Clone)]
pub struct VertexWorld(pub Vertex3);

impl VertexModel {
    pub fn to_world(&self, world_from_model: &na::Isometry3<f32>) -> VertexWorld {
        VertexWorld(
            world_from_model.transform_point(&self.0)
        )
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

use vox_stl::fwd::{Facet, Vertex};
use std::collections::HashMap;
use ordered_float::OrderedFloat;

impl Mesh {
    pub fn from_facets(facets: Vec<Facet>) -> Self {
        let mut vertices : Vec<VertexInfo> = vec![];
        let mut vert_lookup = HashMap::new();
        let mut faces = vec![];

        let to_ordered = |v: Vertex| -> (OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>) {
            (OrderedFloat::from(v.0[0]), OrderedFloat::from(v.0[1]), OrderedFloat::from(v.0[2]))
        };

        for facet in facets {
            let face_i = faces.len();

            // Build up the list of vertices, de-duplicating them if need be.
            let vs : Vec<usize> = (0..3usize).map( |i| {
                let key = to_ordered(facet.tri[i]);

                let vert = VertexModel(Vertex3::from(facet.tri[i].0));

                if !vert_lookup.contains_key(&key) {
                    vert_lookup.insert(key, vert_lookup.len());
                    vertices.push(VertexInfo{ vtx: vert, faces: vec![] });
                }

                let vert_i = vert_lookup.get(&key).unwrap();

                vertices.get_mut(*vert_i).unwrap().faces.push(face_i);

                *vert_i
            }).collect();

            faces.push(FaceInfo{ vertices: [vs[0], vs[1], vs[2]] });
        }

        Self{faces, vertices}
    }
}
