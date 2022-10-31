extern crate nalgebra as na;

use nalgebra::Point3;

pub struct CameraInfo {
    pub eye: Point3<f32>,
    pub target: Point3<f32>
}

impl CameraInfo {
    pub fn get_view_matrix(&self) -> na::Isometry3<f32> {
        return na::Isometry3::look_at_rh(&self.eye, &self.target, &na::Vector3::y());
    }
}
