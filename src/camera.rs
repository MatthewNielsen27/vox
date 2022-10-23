extern crate nalgebra as na;

use crate::fwd::{Vertex3};

// https://gabrielgambetta.com/computer-graphics-from-scratch/09-perspective-projection.html

pub struct Viewport {
    // Viewport distance from camera (along Z+)
    pub z_minmax: (f32, f32),
    //  size of viewport (width, height) in pixels
    pub size: (i32, i32),
    pub upper_left: (i32, i32) // (x,y)
}

impl Viewport {
    // https://en.wikipedia.org/wiki/Graphics_pipeline#Window-Viewport_transformation
    pub fn transform_point(&self, p: &Vertex3) -> Vertex3 {
        Vertex3::from(
            [
                self.upper_left.0 as f32 + (((1.0 + p.x) * self.size.0 as f32) / 2.0),
                self.upper_left.1 as f32 + (((1.0 + p.y) * self.size.1 as f32) / 2.0),
                self.z_minmax.0 + (p.z * (self.z_minmax.1 - self.z_minmax.0))
            ]
        )
    }
}
