extern crate nalgebra as na;

use nalgebra::Vector3;
use vox_fwd::{Pt3, Vec3, Px2};

pub struct CameraInfo {
    pub eye: Pt3,
    pub target: Pt3,
    pub view_matrix: na::Isometry3<f32>
}

impl CameraInfo {
    pub fn new(eye: Pt3, target: Pt3) -> Self {
        Self {
            eye: eye.clone(),
            target: target.clone(),
            view_matrix: na::Isometry3::look_at_rh(&eye, &target, &Vec3::y())
        }
    }

    pub fn screen_ray(&self, pt: &Px2) -> (Pt3, Vec3){
        let p_clip = Pt3::from([pt.x as f32, pt.y as f32, 0.0]);
        let d_clip = Vector3::<f32>::from([0.0, 0.0, 1.0]);

        (
            self.view_matrix.inverse_transform_point(&p_clip),
            self.view_matrix.inverse_transform_vector(&d_clip).normalize()
        )
    }
}
