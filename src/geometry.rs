use nalgebra::{Vector3, Point3};
extern crate nalgebra as na;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Triangle<PointType>(pub [PointType; 3]);

pub struct Plane {
    pub n: Vector3<f32>,
    pub d: f32
}

pub struct Ray {
    pub direction: Vector3<f32>,
    pub point: Point3<f32>
}

impl Ray {
    pub fn from_points(p1: &Point3<f32>, p2: &Point3<f32>) -> Self {
        Ray {
            direction: p1 - p2,
            point: *p1
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum IntersectionType {
    Incidental,
    Single,
    None
}

impl Plane {
    pub fn normalized(&self) -> Self {
        let mag = self.n.norm();

        assert_ne!(mag, 0.0);

        Self {
            n: self.n / mag,
            d: self.d / mag
        }
    }

    /// returns the signed-distance from the point to the plane
    pub fn distance(&self, p: &Point3<f32>) -> f32 {
        let top : f32 = (self.n.x * p.x) + (self.n.y * p.y) + (self.n.z * p.z) + self.d;
        let bottom : f32 = self.n.norm();
        top / bottom
    }

    pub fn from(normal: &Vector3<f32>, point: &Vector3<f32>) -> Plane {
        let n = normal.normalize();
        let d = -1.0 * ((n.x * point.x) + (n.y * point.y) + (n.z * point.z));
        Plane { n, d }
    }

    /// returns a point on the plane either [x, 0, 0], [0, y, 0], or [0, 0, z] depending
    /// on whichever is safest (i.e. direction is nonzero).
    pub fn get_some_point(&self) -> Point3<f32> {
        if self.n.x != 0.0 {
            Point3::from([-1.0 * self.d / self.n.x, 0.0, 0.0])
        } else if self.n.y != 0.0  {
            Point3::from([0.0, -1.0 * self.d / self.n.y, 0.0])
        } else {
            Point3::from([0.0, 0.0,  -1.0 * self.d / self.n.z])
        }
    }

    /// returns the intersection of the Ray with Self
    pub fn ray_intersection(
        &self,
        ray: &Ray
    )
        -> (IntersectionType, Option<Point3<f32>>)
    {
        if self.n.dot(&ray.direction) == 0.0 {
            // todo: handle IntersectionType::Incidental in this case...
            return (IntersectionType::None, None);
        }

        // This calculation can be found in:
        //      https://rosettacode.org/wiki/Find_the_intersection_of_a_line_with_a_plane#Rust
        let diff = ray.point - self.get_some_point();
        let prod1 = diff.dot(&self.n);
        let prod2 = ray.direction.dot(&self.n);
        let prod3 = prod1 / prod2;

        let point = ray.point - ray.direction.scale(prod3);

        (IntersectionType::Single, Some(Point3::from([point.x, point.y, point.z])))
    }
}
