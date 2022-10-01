use std::path::Path;
use vox::stl;

#[test]
fn ascii_stl_parse_facets() {
    let facets = stl::facets_from_ascii_stl(Path::new("resources/cube_ascii.stl")).unwrap();
    assert_eq!(facets.len(), 12); // 2 triangles per side in a cube.
}

#[test]
fn ascii_stl_parse_mesh() {
    let mesh = stl::mesh_from_stl(Path::new("resources/cube_ascii.stl")).unwrap();
    assert_eq!(mesh.faces.len(), 12);
    assert_eq!(mesh.vertices.len(), 8);
}
