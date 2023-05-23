use std::path::Path;

use vox_stl::stl;

#[test]
fn parse_ascii_stl() {
    {
        let facets = stl::parse_from_file(Path::new("../resources/models/ascii-cube.stl")).unwrap();
        assert_eq!(facets.len(), 12);
    }

    {
        let facets = stl::parse_from_file(Path::new("../resources/models/ascii-sphere.stl")).unwrap();
        assert_eq!(facets.len(), 960);
    }
}

#[test]
fn parse_binary_stl() {
    let facets = stl::parse_from_file(Path::new("../resources/models/binary-cube.stl")).unwrap();
    assert_eq!(facets.len(), 12);
}
