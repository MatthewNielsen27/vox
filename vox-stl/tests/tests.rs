use std::path::Path;

use vox_stl::stl;

#[test]
fn ascii_stl_parse_mesh() {
    let facets = stl::parse_from_file(Path::new("assets/cube_ascii.stl")).unwrap();
    assert_eq!(facets.len(), 12);
}
