use std::fs::File;

use crate::fwd::Facet;

/// A simple parser for binary STL files.
///
/// see: https://en.wikipedia.org/wiki/STL_(file_format)#Binary_STL
///
pub fn facets_from_binary_stl(_file: &mut File) -> Result<Vec<Facet>, String> {
    panic!("not implemented!");
}
