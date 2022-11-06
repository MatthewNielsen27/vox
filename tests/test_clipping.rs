use std::path::Path;
use nalgebra::{Point3, Vector3};

use vox_stl::stl;
use vox::clipping;
use vox::geometry::{Plane, Triangle};
use vox::model::Mesh;

#[test]
fn test_clipping_bounding_sphere() {
    // todo: We need a UnitSolids type thing
    let mesh = Mesh::from_facets(
        stl::parse_from_file(Path::new("resources/cube_ascii.stl")).unwrap()
    );
    assert_eq!(mesh.faces.len(), 12);
    assert_eq!(mesh.vertices.len(), 8);

    let vertices : Vec<Point3<f32>> = mesh.vertices.iter().map(|vert| vert.vtx.0).collect();

    let bs = clipping::BoundingSphere::from(&vertices);
    assert_eq!(bs.center, Point3::from([0.5, 0.5, 0.5]));
    assert_eq!(bs.radius, 0.8660254);

    // --
    // [Scenario] Clip plane is below the model, pointing upwards
    let cp = Plane::from(
        &Vector3::from([0.0, 0.0, 1.0]),
        &Vector3::from([0.0, 0.0, -1.0])
    );
    assert_eq!(clipping::get_clip_type(&bs, &cp), clipping::ClipType::NopeAllFront);

    // --
    // [Scenario] Clip plane is above the model, pointing upwards
    let cp = Plane::from(
        &Vector3::from([0.0, 0.0, 1.0]),
        &Vector3::from([0.0, 0.0, 2.0])
    );
    assert_eq!(clipping::get_clip_type(&bs, &cp), clipping::ClipType::NopeAllBehind);

    // --
    // [Scenario] Clip plane is in the middle of the model
    let cp = Plane::from(
        &Vector3::from([0.0, 0.0, 1.0]),
        &Vector3::from([0.0, 0.0, 0.5])
    );
    assert_eq!(clipping::get_clip_type(&bs, &cp), clipping::ClipType::Clip);
}

#[test]
fn test_clipping_triangle() {
    // [Scenario] The triangle has 1 vertex below the plane.
    {
        let tri = Triangle([
            Point3::from([0.0, 0.0, 1.0]),
            Point3::from([-1.0, 0.0, 1.0]),
            Point3::from([1.0, 0.0, -1.0])
        ]);

        let plane = Plane::from(
            &Vector3::from([0.0, 0.0, 1.0]),
            &Vector3::from([0.0, 0.0, 0.0])
        );

        let (tri_a, tri_b) = clipping::clip_triangle(&plane, &tri).unwrap();
        assert!(matches!(tri_a, clipping::ClippedTriangle::SingleReplacement{ .. }));
        assert!(matches!(tri_b.unwrap(), clipping::ClippedTriangle::DoubleReplacement{ .. }));
    }

    // [Scenario] The triangle has 2 vertices below the plane.
    {
        let tri = Triangle([
            Point3::from([0.0, 10.0, 1.0]),
            Point3::from([-1.0, 10.0, -1.0]),
            Point3::from([1.0, 10.0, -1.0])
        ]);

        let plane = Plane::from(
            &Vector3::from([0.0, 0.0, 1.0]),
            &Vector3::from([0.0, 0.0, 0.0])
        );

        let (clip, _) = clipping::clip_triangle(&plane, &tri).unwrap();
        assert!(matches!(clip, clipping::ClippedTriangle::DoubleReplacement{ .. }));
    }

    // [Scenario] The triangle is completely above the plane.
    {
        let tri = Triangle([
            Point3::from([0.0, 0.0, 1.0]),
            Point3::from([-1.0, 0.0, -1.0]),
            Point3::from([1.0, 0.0, -1.0])
        ]);

        let plane = Plane::from(
            &Vector3::from([0.0, 0.0, 1.0]),
            &Vector3::from([0.0, 0.0, -10.0])
        );

        let (clip, _) = clipping::clip_triangle(&plane, &tri).unwrap();
        assert!(matches!(clip, clipping::ClippedTriangle::NoClip));
    }

    // [Scenario] The triangle is completely below the plane.
    {
        let tri = Triangle([
            Point3::from([0.0, 0.0, -1.0]),
            Point3::from([-1.0, 0.0, -2.0]),
            Point3::from([1.0, 0.0, -2.0])
        ]);

        let plane = Plane::from(
            &Vector3::from([0.0, 0.0, 1.0]),
            &Vector3::from([0.0, 0.0, 0.0])
        );

        assert!(clipping::clip_triangle(&plane, &tri).is_none());
    }
}
