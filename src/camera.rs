use image::flat::View;

extern crate nalgebra as na;
use na::{Vector3, Vector2, Rotation3};
use nalgebra::Point3;

use crate::fwd::{raster, Vertex3D};

// https://gabrielgambetta.com/computer-graphics-from-scratch/09-perspective-projection.html

pub struct Viewport {
    // Viewport distance from camera (along Z+)
    pub d: f32,
    // Viewport width
    pub w: f32,
    // Viewport height
    pub h: f32
}

pub struct Camera {
    pub location: na::Point3<f32>
}

impl Camera {
    // Returns the `view` of the camera looking at a certain point
    pub fn looking_at(&self, location: na::Point3<f32>) -> na::Isometry3<f32> {
        na::Isometry3::look_at_rh(&self.location, &location, &Vector3::y())
    }
}

impl Viewport {
    // todo: this only handles standard camera setup (i.e. centered at origin, looking at viewport)
    pub fn point_projection(&self, p: Vertex3D) -> Vertex3D {
        let x = (p.x * self.d) / p.z;
        let y = (p.y * self.d) / p.z;
        Vertex3D::from([x, y, self.d])
    }

    pub fn point_projection_canvas(&self, p: Vertex3D, canvas: &(u32, u32)) -> raster::Pixel {
        let proj = self.point_projection(p);
        let x = (proj.x * canvas.0 as f32) / self.w;
        let y = (proj.y * canvas.1 as f32) / self.h;
        raster::Pixel{x: x as i32, y: y as i32}
    }
}
