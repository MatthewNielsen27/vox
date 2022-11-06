use std::mem;
use nalgebra::{Point3, Vector3};

use crate::geometry::{IntersectionType, Plane, Ray, Triangle};

pub struct BoundingSphere {
    pub center: Point3<f32>,
    pub radius: f32
}

#[derive(PartialEq, Debug)]
pub enum ClipType {
    NopeAllBehind,
    NopeAllFront,
    Clip
}

// Returns the clipping planes for the given matrix
pub fn get_clipping_planes(mat: &nalgebra::Matrix4<f32>) -> Vec<Plane> {
    vec![
        // Left clipping plane
        Plane {
            n: Vector3::from(
                [
                    mat.m41 + mat.m11,
                    mat.m42 + mat.m12,
                    mat.m43 + mat.m13,
                ]
            ),
            d: mat.m44 + mat.m14
        }.normalized(),
        // Right clipping plane
        Plane {
            n: Vector3::from(
                [
                    mat.m41 - mat.m11,
                    mat.m42 - mat.m12,
                    mat.m43 - mat.m13,
                ]
            ),
            d: mat.m44 - mat.m14
        }.normalized(),
        // Top clipping plane
        Plane {
            n: Vector3::from(
                [
                    mat.m41 - mat.m21,
                    mat.m42 - mat.m22,
                    mat.m43 - mat.m23,
                ]
            ),
            d: mat.m44 - mat.m14
        }.normalized(),
        // Bottom clipping plane
        Plane {
            n: Vector3::from(
                [
                    mat.m41 + mat.m21,
                    mat.m42 + mat.m22,
                    mat.m43 + mat.m23,
                ]
            ),
            d: mat.m44 + mat.m14
        }.normalized(),
        // Near clipping plane
        Plane {
            n: Vector3::from(
                [
                    mat.m41 + mat.m31,
                    mat.m42 + mat.m32,
                    mat.m43 + mat.m33,
                ]
            ),
            d: mat.m44 + mat.m34
        }.normalized(),
        // Far clipping plane
        Plane {
            n: Vector3::from(
                [
                    mat.m41 - mat.m31,
                    mat.m42 - mat.m32,
                    mat.m43 - mat.m33,
                ]
            ),
            d: mat.m44 - mat.m34
        }.normalized()
    ]
}

/// returns the ClipType for a given BoundingSphere and a plane
pub fn get_clip_type(sphere: &BoundingSphere, plane: &Plane) -> ClipType {
    let d = plane.distance(&sphere.center);
    if d.abs() < sphere.radius {
        ClipType::Clip
    } else if d > 0.0 {
        ClipType::NopeAllFront
    } else {
        ClipType::NopeAllBehind
    }
}

#[derive(Debug, PartialEq)]
pub struct TriangleWith2Replaced {
    pub tri: Triangle<Point3<f32>>,
    pub replacement_a: (usize, (usize, usize)),
    pub replacement_b: (usize, (usize, usize))
}

#[derive(Debug, PartialEq)]
pub struct TriangleWith1Replaced {
    pub tri: Triangle<Point3<f32>>,
    pub replacement: (usize, (usize, usize))
}

#[derive(Debug, PartialEq)]
pub enum ClippedTriangle {
    NoClip,
    SingleReplacement(TriangleWith1Replaced),
    DoubleReplacement(TriangleWith2Replaced)
}

/// returns either 1, 2, or None triangles
///
/// todo: We need to return more information, or figure out how we can associate replaced vertices
///       without losing metadata... (i.e. shading value...)
pub fn clip_triangle(
    plane: &Plane,
    tri: &Triangle<Point3<f32>>
)
    -> Option<(ClippedTriangle, Option<ClippedTriangle>)>
{
    let d0 = plane.distance(&tri.0[0]);
    let d1 = plane.distance(&tri.0[1]);
    let d2 = plane.distance(&tri.0[2]);

    let points = [d0, d1, d2];

    let state = points.iter().filter(|x| **x >= 0.0).count();
    if state == 3 {
        // We have 3 point(s) above the plane
        Some((ClippedTriangle::NoClip, None))
    } else if state == 0 {
        // We have 3 point(s) below the plane
        None
    } else if state == 2 {
        // We have 1 point(s) below the plane
        //
        // In this case, we need to introduce 2 new vertices and an additional triangle.

        // Step 1:  We need to order the vertices based on distance, i_2 will the point below the
        //          line.
        let mut i_0 = 0;
        let mut i_1 = 1;
        let mut i_2 = 2;
        if points[1] > points[0] {
            mem::swap(&mut i_1, &mut i_0);
        }
        if points[2] > points[0] {
            mem::swap(&mut i_2, &mut i_0);
        }
        if points[2] > points[1] {
            mem::swap(&mut i_2, &mut i_1);
        }

        // Step 2:  We to determine where to put the two new vertices, and create the 2 new
        //          triangles
        let ray_ac = Ray { direction: (tri.0[i_0] - tri.0[i_2]).normalize(), point: tri.0[i_0] };
        let ray_bc = Ray { direction: (tri.0[i_1] - tri.0[i_2]).normalize(), point: tri.0[i_1] };

        let mut new_triangle_1 = tri.clone();
        let mut new_triangle_2 = tri.clone();

        match plane.ray_intersection(&ray_bc) {
            (IntersectionType::Single, Some(point)) => {
                // For this triangle, we'll consider the new 'C' to be the line 'BC'
                new_triangle_1.0[i_2] = point;

                // For this triangle, we'll consider the new 'B' to be the line 'BC'
                new_triangle_2.0[i_1] = point;
            },
            (_, _) => {
                panic!("Expected a single ray intersection with the plane.");
            }
        }
        match plane.ray_intersection(&ray_ac) {
            (IntersectionType::Single, Some(point)) => {
                // For this triangle, we'll consider the new 'C' to be the line 'AC'
                new_triangle_2.0[i_2] = point;
            },
            (_, _) => {
                panic!("Expected a single ray intersection with the plane.");
            }
        }

        let new_triangle_1 = TriangleWith1Replaced {
            tri: new_triangle_1,
            replacement: (i_2, (i_1, i_2)) // 'C' gets replaced with the intersection of `BC`
        };

        let new_triangle_2 = TriangleWith2Replaced {
            tri: new_triangle_2,
            replacement_a: (i_1, (i_1, i_2)), // 'B' gets replaced with the intersection of `BC`
            replacement_b: (i_2, (i_0, i_2))  // 'C' gets replaced with the intersection of `AC`
        };

        Some((ClippedTriangle::SingleReplacement(new_triangle_1), Some(ClippedTriangle::DoubleReplacement(new_triangle_2))))

    } else {
        // --
        // We have 2 point(s) below the plane
        //
        // In this case, we simply need to relocate 2 vertices and return
        // a single triangle.

        let mut new_tri = tri.clone();

        // We'll consider 'A' to be the 'highest' point
        let i_a = {
          if d0 > 0.0 {
              0
          } else if d1 > 0.0 {
              1
          } else {
              2
          }
        };

        let i_b = (i_a + 1) % 3;
        let i_c = (i_a + 2) % 3;

        // Now let's iterate over 'B' and 'C'
        for i in [i_b, i_c] {
            // Now we need to determine the location of the new 'B' vertex
            let ray = Ray::from_points(&tri.0[i_a], &tri.0[i]);

            match plane.ray_intersection(&ray) {
                (IntersectionType::Single, Some(point)) => {
                    new_tri.0[i] = point;
                },
                (_, _) => {
                    panic!("Expected a single ray intersection with the plane.");
                }
            }
        }

        let result = TriangleWith2Replaced {
            tri: new_tri,
            replacement_a: (i_b, (i_a, i_b)),
            replacement_b: (i_c, (i_a, i_c))
        };

        Some((ClippedTriangle::DoubleReplacement(result), None))
    }
}

impl BoundingSphere {
    /// returns a BoundingSphere for the given set of points
    pub fn from(points: &[Point3<f32>]) -> BoundingSphere {
        let mut center = Point3::<f32>::default();
        points.iter().for_each(|p| {
            // todo: maybe we need to impl the Add trait here so we can create a new point.
            center.x += p.x;
            center.y += p.y;
            center.z += p.z;
        });

        center /= points.len() as f32;

        let mut max_distance = 0.0;
        for p in points {
            let d = (center - p).norm();
            if d > max_distance {
                max_distance = d;
            }
        }

        BoundingSphere{ center, radius: max_distance }
    }
}
