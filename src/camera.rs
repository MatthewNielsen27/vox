use image::flat::View;

extern crate nalgebra as na;
use na::{Vector3, Vector2, Rotation3};

use crate::fwd::Vertex2D;

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
    pub location: Vector3<f32>
}

impl Viewport {
    // todo: this only handles standard camera setup (i.e. centered at origin, looking at viewport)
    pub fn point_projection(&self, p: Vector3<f32>) -> Vector3<f32> {
        let x = (p.x * self.d) / p.z;
        let y = (p.y * self.d) / p.z;
        Vector3::from([x, y, self.d])
    }

    pub fn point_projection_canvas(&self, p: Vector3<f32>, canvas: &(u32, u32)) -> Vertex2D {
        let proj = self.point_projection(p);
        let x = (proj.x * canvas.0 as f32) / self.w;
        let y = (proj.y * canvas.1 as f32) / self.h;
        Vertex2D{x: x as i32, y: y as i32}
    }
}
