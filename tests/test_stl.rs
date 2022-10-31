use std::path::Path;

use vox::stl;

#[test]
fn ascii_stl_parse_mesh() {
    let mesh = stl::pparse_from_file(Path::new("resources/cube_ascii.stl")).unwrap();
    assert_eq!(mesh.faces.len(), 12);
    assert_eq!(mesh.vertices.len(), 8);
}
