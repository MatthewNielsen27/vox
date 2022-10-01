use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

use crate::fwd::{Vertex3D};
use crate::model::{Mesh, VertexInfo, FaceInfo};

enum ParseMode {
    ExpectingSolidStart,
    ExpectingStartOfFacetOrEndOfSolid,
    ExpectingFacetEnd,
    ExpectingLoopStart,
    ExpectingLoopEnd,
    ExpectingVertex,
    Done
}

#[derive(Default, Copy, Clone)]
pub struct Facet {
    pub data: [Vertex3D; 3]
}

use ordered_float::OrderedFloat;

/// To hash a vertex, we need to wrap it in an OrderedFloat because f32 doesn't have the
/// necessary comparison operators.
fn to_hashable(v: Vertex3D) -> [OrderedFloat<f32>; 3] {
    [OrderedFloat(v.x), OrderedFloat(v.y), OrderedFloat(v.z)]
}

pub fn mesh_from_stl(path: &Path) -> Result<Mesh, String> {
    // TODO: we need to determine if the mesh is an ascii STL or a binary STL.
    //
    // Next steps:
    //      - read the first N bytes of the file and look for a particular header.
    //      - dispatch the correct parser based on the file encoding (ascii or binary).
    //      - make the _ascii functions private as we should only need the one interface
    //        for parsing .stl files.
    match facets_from_ascii_stl(path) {
        Err(e) => Err(e),
        // Build up the mesh geometry and connectivity of the facets
        Ok(facets) => {
            let mut vertices : Vec<VertexInfo> = vec![];
            let mut vert_lookup = HashMap::new();
            let mut faces = vec![];

            for facet in facets {
                let face_i = faces.len();

                // Build up the list of vertices, de-duplicating them if need be.
                let vs : Vec<usize> = (0..3usize).map( |i| {
                    let key = to_hashable(facet.data[i]);

                    if !vert_lookup.contains_key(&key) {
                        vert_lookup.insert(key, vert_lookup.len());
                        vertices.push(VertexInfo{ vtx: facet.data[i], faces: vec![] });
                    }

                    let vert_i = vert_lookup.get(&key).unwrap();

                    vertices.get_mut(*vert_i).unwrap().faces.push(face_i);

                    *vert_i
                }).collect();

                faces.push(FaceInfo{ vertices: [vs[0], vs[1], vs[2]] });
            }

            Ok(Mesh{faces, vertices})
        }
    }
}

pub fn facets_from_ascii_stl(path: &Path) -> Result<Vec<Facet>, String> {
    let mut file = File::open(&path).unwrap();

    let mut mode = ParseMode::ExpectingSolidStart;

    let mut current_vertex_i = 0;
    let mut current_facet = Facet::default();

    let mut facets = vec![];

    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        let line =  match line {
            Ok(l) => l,
            Err(why) => panic!("{}", why)
        };

        let raw = line.trim();

        match mode {
            ParseMode::Done => { break; }

            ParseMode::ExpectingSolidStart => {
                if !raw.starts_with("solid") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                mode = ParseMode::ExpectingStartOfFacetOrEndOfSolid;
            }

            ParseMode::ExpectingStartOfFacetOrEndOfSolid => {
                if raw.starts_with("facet") {
                    mode = ParseMode::ExpectingLoopStart;
                } else if raw.starts_with("endsolid") {
                    mode = ParseMode::Done;
                } else {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
            }

            ParseMode::ExpectingFacetEnd => {
                if !raw.starts_with("endfacet") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                mode = ParseMode::ExpectingStartOfFacetOrEndOfSolid;
            }

            ParseMode::ExpectingLoopStart => {
                if !raw.starts_with("outer loop") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                mode = ParseMode::ExpectingVertex;
            }

            ParseMode::ExpectingLoopEnd => {
                if !raw.starts_with("endloop") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                mode = ParseMode::ExpectingFacetEnd;
            }

            ParseMode::ExpectingVertex => {
                let parts = raw.split_whitespace().collect::<Vec<&str>>();

                if parts.len() != 4 {
                    return Err(format!("[{}] expected vertex, got {}", i, raw));
                } else if parts[0] != "vertex" {
                    return Err(format!("[{}] expected vertex, got {}", i, raw));
                }

                current_facet.data[current_vertex_i] = Vertex3D::from(
                    [
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                        parts[3].parse().unwrap()
                    ]
                );

                current_vertex_i += 1;

                if current_vertex_i == 3 {
                    current_vertex_i = 0;
                    mode = ParseMode::ExpectingLoopEnd;
                    facets.push(current_facet.clone());
                }
            }
        }
    }

    Ok(facets)
}