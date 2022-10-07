use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::string::FromUtf8Error;

use crate::fwd::{Vertex3D};
use crate::model::{Mesh, VertexInfo, FaceInfo, VertexModel};

pub fn mesh_from_stl(path: &Path) -> Result<Mesh, String> {
    Ok(mesh_from_facets(facets_from_stl(path)?))
}

#[derive(Default, Copy, Clone)]
pub struct Facet {
    pub data: [Vertex3D; 3]
}

fn facets_from_stl(path: &Path) -> Result<Vec<Facet>, String> {
    match get_stl_encoding(path)? {
        StlEncoding::Ascii =>  { facets_from_ascii_stl(path) }
        StlEncoding::Binary => { facets_from_binary_stl(path) }
    }
}

enum StlEncoding {
    Ascii,
    Binary
}

/// Returns StlEncoding::Ascii if the file begins with 'solid', else StlEncoding::Binary
///
fn get_stl_encoding(path: &Path) -> Result<StlEncoding, String> {
    // todo: this could take an open File handle instead of a Path
    let mut file = File::open(&path).unwrap();

    let parsed_header = {
        let mut bytes = [0; 5];
        file.read_exact(&mut bytes);
        String::from_utf8(Vec::from(bytes))
    };

    match parsed_header {
        Err(msg) => {  Err(msg.to_string()) }

        Ok(header) => {
            let encoding = {
                if header == "solid" {
                    StlEncoding::Ascii
                } else {
                    StlEncoding::Binary
                }
            };

            Ok(encoding)
        }
    }
}

/// Defines the states of an STL parser.
///
enum ParseMode {
    ExpectingSolidStart,
    ExpectingStartOfFacetOrEndOfSolid,
    ExpectingFacetEnd,
    ExpectingLoopStart,
    ExpectingLoopEnd,
    ExpectingVertex,
    Done
}

/// A simple parser for binary STL files.
///
/// see: https://en.wikipedia.org/wiki/STL_(file_format)#Binary_STL
///
fn facets_from_binary_stl(_path: &Path) -> Result<Vec<Facet>, String> {
    panic!("not implemented!");
}

/// A simple parser for ascii STL files.
///
/// see: https://en.wikipedia.org/wiki/STL_(file_format)#ASCII_STL
///
fn facets_from_ascii_stl(path: &Path) -> Result<Vec<Facet>, String> {
    let mut file = File::open(&path).unwrap(); // todo: this could take an open File handle instead of a Path

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

fn to_hashable(v: Vertex3D) -> (OrderedFloat<f32>, OrderedFloat<f32>, OrderedFloat<f32>) {
    (OrderedFloat::from(v.x), OrderedFloat::from(v.y), OrderedFloat::from(v.z))
}

/// Builds face-vertex mesh from list of facets.
///
fn mesh_from_facets(facets: Vec<Facet>) -> Mesh {
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
                vertices.push(VertexInfo{ vtx: VertexModel(facet.data[i]), faces: vec![] });
            }

            let vert_i = vert_lookup.get(&key).unwrap();

            vertices.get_mut(*vert_i).unwrap().faces.push(face_i);

            *vert_i
        }).collect();

        faces.push(FaceInfo{ vertices: [vs[0], vs[1], vs[2]] });
    }

    Mesh{faces, vertices}
}
