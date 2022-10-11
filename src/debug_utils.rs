
use rand::Rng;
use image::{ImageBuffer, RgbImage, Rgb};
use nalgebra::Point3;

use crate::fwd::raster;
use crate::geometry::Triangle;
use crate::grr;

// todo: see if we want to impl some 'upper-left' type stuff...
pub struct Canvas {
    img: RgbImage,
    pub size: (usize, usize)
}

pub fn random_col() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    let g: u8 = rng.gen();
    let b: u8 = rng.gen();
    Rgb::from([r, g, b])
}

impl Canvas {
    pub fn from(size: (usize, usize)) -> Self {
        Canvas {
            img: ImageBuffer::new(size.0 as u32, size.1 as u32),
            size
        }
    }

    /// Renders the triangle the canvas, ignoring Z
    pub fn render_triangle(&mut self, tri: &Triangle<Point3<f32>>, c: &Rgb<u8>) {
        let tri_2d = raster::Triangle2D {
            points: [
                self.to_pixel(&tri.0[0]),
                self.to_pixel(&tri.0[1]),
                self.to_pixel(&tri.0[2])
            ]
        };
        grr::render_triangle(&mut self.img, &tri_2d, *c);
    }

    fn to_pixel(&self, p: &Point3<f32>) -> raster::Pixel {
        let hdw = self.size.0 as i32 / 2;
        let hdh = self.size.1 as i32 / 2;
        raster::Pixel {
            x: (p.x as i32 + hdw),
            y: (p.y as i32 + hdh),
        }
    }

    /// saves the Canvas to a file
    pub fn save(&self, path: &str) {
        self.img.save(path);
    }
}
