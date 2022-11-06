use image::{ImageBuffer, RgbImage, Rgb};

use crate::fwd::Vertex3Ndc;
use crate::raster::Pixel;


/// [brief] This struct represents a simple rendering surface.
pub struct Surface {
    // The shape of the rendering surface (width, height)
    pub shape: (usize, usize),

    // This is the z-buffer
    z_buffer: Vec<f32>,

    // This is the buffer of pixels
    p_buffer: Vec<[u8; 3]>,

    // todo: z_minmax ??
    // todo: upper_left ??
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Self {
        Surface {
            shape: (width, height),
            z_buffer: vec![0.0; width * height],
            p_buffer: vec![[0, 0, 0]; width * height],
        }
    }

    /// [returns] the Surface as an RGB image
    pub fn to_img(&self) -> RgbImage {
        // todo: I'm sure we can optimize this to be a single memcpy.
        let mut img = ImageBuffer::new(self.shape.0 as u32, self.shape.1 as u32);
        for x in 0..self.shape.0 {
            for y in 0..self.shape.1 {
                img.put_pixel(x as u32, y as u32, Rgb::from(*self.get_pixel(x, y)));
            }
        }
        img
    }

    /// [returns] z-buffer value for (x,y) coordinate.
    pub fn get_z(&self, x: usize, y: usize) -> f32 {
        self.z_buffer[self.get_index(x,y)]
    }

    pub fn set_z(&mut self, x: usize, y: usize, z: f32) {
        let i = self.get_index(x,y);
        self.z_buffer[i] = z;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, col: &[u8; 3]) {
        let i = self.get_index(x,y);
        self.p_buffer[i] = *col;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &[u8; 3] {
        let i = self.get_index(x,y);
        &self.p_buffer[i]
    }

    /// [returns] buffer index for (x,y) coordinate.
    fn get_index(&self, x: usize, y: usize) -> usize {
        x + ((self.shape.0 - 1) * y)
    }

    pub fn to_pixel(&self, p: &Vertex3Ndc) -> (Pixel, f32) {
        assert!(p.is_valid());
        (
            Pixel {
                x: (((1.0 + p.0.x) * self.shape.0 as f32) / 2.0) as i32,
                y: (((1.0 + p.0.y) * self.shape.1 as f32) / 2.0) as i32,
            },
            p.0.z
        )
    }
}
