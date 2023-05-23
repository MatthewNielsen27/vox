use std::fs::File;
use std::path::Path;
use image::{Rgb, RgbImage};

extern crate nalgebra as na;

pub struct MatcapTexture {
    contents: RgbImage,
    center: (u32, u32)
}

impl MatcapTexture {
    pub fn from_file(p: &Path) -> Self {
        let i = image::open(p).unwrap();

        let center = (i.width() / 2, i.height() / 2);

        Self {
            contents: i.into_rgb8(),
            center
        }
    }

    pub fn sample_normal(&self, pt: &na::Vector3<f32>) -> Rgb<u8> {
        let uv = (
            ((pt.x * 0.48) * self.contents.width() as f32) as i32 + self.center.0 as i32,
            ((pt.y * -0.48) * self.contents.height() as f32) as i32 + self.center.1 as i32,
        );

        *self.contents.get_pixel(uv.0 as u32, uv.1 as u32)
    }
}
