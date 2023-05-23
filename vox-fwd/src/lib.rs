#![feature(tuple_trait)]

pub mod coordinate_systems;
pub mod mesh;
pub mod safety;

extern crate nalgebra as na;

pub type Pt3 = na::Point3<f32>;
pub type Vec3 = na::Vector3<f32>;
pub type Pt2 = na::Point2<f32>;

pub type Px2 = na::Point2<i32>;
pub type Px3 = na::Point3<i32>; // this is really a voxel...

pub type Transform3 = na::Transform3<f32>;

// impl Point3Ndc {
//     /// [returns] true if all points are within [-1.0, 1.0]
//     pub fn is_valid(&self) -> bool {
//         (-1.0 <= self.0.x && self.0.x <= 1.0)
//             &&  (-1.0 <= self.0.y && self.0.y <= 1.0)
//             &&  (-1.0 <= self.0.z && self.0.z <= 1.0)
//     }
// }
