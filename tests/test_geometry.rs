use nalgebra::{Point3, Vector3};

use vox::geometry;

#[test]
fn test_plane_ray_intersection() {
    let ray = geometry::Ray::from_points(
        &Point3::from([2.0, 2.0, 2.0]),
        &Point3::from([2.0, 2.0, -2.0])
    );

    let pln = geometry::Plane::from(
        &Vector3::from([0.0, 0.0, 1.0]),
        &Vector3::from([0.0, 0.0, 0.0])
    );

    let (intersection_type, intersection_point) = pln.ray_intersection(&ray);

    assert_eq!(intersection_type, geometry::IntersectionType::Single);
    assert!(intersection_point.is_some());
    assert_eq!(intersection_point.unwrap(), Point3::from([2.0, 2.0, 0.0]));
}

#[test]
fn test_plane_point_distance() {
    let pln = geometry::Plane::from(
        &Vector3::from([0.0, 0.0, 1.0]),
        &Vector3::from([0.0, 0.0, 0.0])
    );

    assert_eq!(pln.distance(&Point3::from([0.0, 0.0, 0.0])), 0.0);
    assert_eq!(pln.distance(&Point3::from([0.0, 0.0, 1.0])), 1.0);
}
