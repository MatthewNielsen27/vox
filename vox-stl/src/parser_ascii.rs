use std::fs::File;
use std::io::{BufRead, BufReader, Seek};

use crate::fwd::{Facet, Vertex, Normal, Vec3};

/// These are the states of the ascii .stl parser state-machine.
enum ParsingState {
    ExpectingSolidStart,
    ExpectingStartOfFacetOrEndOfSolid,
    ExpectingFacetEnd,
    ExpectingLoopStart,
    ExpectingLoopEnd,
    ExpectingVertex,
    Done
}

/// A simple parser for ascii STL files.
///
/// see: https://en.wikipedia.org/wiki/STL_(file_format)#ASCII_STL
///
pub fn facets_from_ascii_stl(file: &mut File) -> Result<Vec<Facet>, String> {
    file.rewind().unwrap(); // seek back to start

    let mut state = ParsingState::ExpectingSolidStart;

    let mut current_vertex_i = 0;
    let mut current_facet = Facet::default();

    let mut facets = vec![];

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line =  match line {
            Ok(l) => l,
            Err(why) => panic!("{}", why)
        };

        let raw = line.trim();

        match state {
            ParsingState::Done => { break; }

            ParsingState::ExpectingSolidStart => {
                if !raw.starts_with("solid") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                state = ParsingState::ExpectingStartOfFacetOrEndOfSolid;
            }

            ParsingState::ExpectingStartOfFacetOrEndOfSolid => {
                if raw.starts_with("facet normal") {
                    state = ParsingState::ExpectingLoopStart;

                    let parts = raw.split_whitespace().collect::<Vec<&str>>();
                    if parts.len() != 5 {
                        return Err(format!("[{}] expected: 'facet normal {{}} {{}} {{}}, got: {}", i, raw));
                    }

                    current_facet.normal = Normal::from(Vec3(
                        [
                            parts[2].parse::<f32>().unwrap(),
                            parts[3].parse::<f32>().unwrap(),
                            parts[4].parse::<f32>().unwrap()
                        ]
                    ));

                } else if raw.starts_with("endsolid") {
                    state = ParsingState::Done;
                } else {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
            }

            ParsingState::ExpectingFacetEnd => {
                if !raw.starts_with("endfacet") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                state = ParsingState::ExpectingStartOfFacetOrEndOfSolid;
            }

            ParsingState::ExpectingLoopStart => {
                if !raw.starts_with("outer loop") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                state = ParsingState::ExpectingVertex;
            }

            ParsingState::ExpectingLoopEnd => {
                if !raw.starts_with("endloop") {
                    return Err(format!("[{}] unexpected token {}", i, raw));
                }
                state = ParsingState::ExpectingFacetEnd;
            }

            ParsingState::ExpectingVertex => {
                let parts = raw.split_whitespace().collect::<Vec<&str>>();

                if parts.len() != 4 {
                    return Err(format!("[{}] expected vertex, got {}", i, raw));
                } else if parts[0] != "vertex" {
                    return Err(format!("[{}] expected vertex, got {}", i, raw));
                }

                current_facet.tri[current_vertex_i] = Vertex::from(Vec3(
                    [
                        parts[1].parse::<f32>().unwrap(),
                        parts[2].parse::<f32>().unwrap(),
                        parts[3].parse::<f32>().unwrap()
                    ]
                ));

                current_vertex_i += 1;

                if current_vertex_i == 3 {
                    current_vertex_i = 0;
                    state = ParsingState::ExpectingLoopEnd;
                    facets.push(current_facet.clone());
                }
            }
        }
    }

    Ok(facets)
}
