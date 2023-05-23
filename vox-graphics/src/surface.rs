use image::{ImageBuffer, RgbImage, Rgb, DynamicImage};
use image::imageops::FilterType;

use vox_fwd::{Pt3};
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

    supersampling: bool
}

impl Surface {
    pub fn clear(&mut self) {
        self.p_buffer.fill([0,0,0]);
        self.z_buffer.fill(0.0);
    }

    pub fn new(width: usize, height: usize, supersampling: bool) -> Self {
        let width = if supersampling { width * 3 } else { width };
        let height = if supersampling { height * 3 } else { height };

        Surface {
            shape: (width, height),
            z_buffer: vec![0.0; width * height],
            p_buffer: vec![[0, 0, 0]; width * height],
            supersampling
        }
    }

    /// [returns] the Surface as an RGB image
    pub fn to_img(&self) -> RgbImage {
        // todo: I'm sure we can optimize this to be a single memcpy.
        let mut img = DynamicImage::new_rgb8(self.shape.0 as u32, self.shape.1 as u32);
        for x in 0..self.shape.0 {
            for y in 0..self.shape.1 {
                img.as_mut_rgb8().unwrap().put_pixel(x as u32, y as u32, Rgb::from(*self.get_pixel(x, y)));
            }
        }

        if self.supersampling {
            img.resize((self.shape.0 / 3) as u32, (self.shape.1 / 3) as u32, FilterType::Lanczos3).into_rgb8()
        } else {
           img.into_rgb8()
        }
    }

    pub fn fill_buffer(&self, buf: &mut Vec<u32>) {
        // assert_eq!(self.p_buffer.len(), buf.len());

        let result = self.to_img();

        for (i, px) in result.pixels().enumerate() {
            buf[i] = ((px[2] as u32) << 16) | ((px[1] as u32) << 8) | px[0] as u32;
        }
    }

    /// [returns] z-buffer value for (x,y) coordinate.
    pub fn get_z(&self, x: usize, y: usize) -> f32 {
        let i = self.get_index(x,y);
        self.z_buffer[i]
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
        x + (self.shape.0 * y)
    }

    pub fn to_pixel(&self, p: &Pt3) -> (Pixel, f32) {
        (
            Pixel {
                x: (((1.0 + p.x) * self.shape.0 as f32) / 2.0) as i32,
                y: (((1.0 + p.y) * self.shape.1 as f32) / 2.0) as i32,
            },
            p.z
        )
    }
}
