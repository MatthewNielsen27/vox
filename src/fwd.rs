extern crate nalgebra as na;

pub type Vertex3D = na::Point3<f32>;

pub mod raster {
    #[derive(Copy, Clone)]
    pub struct Pixel {
        pub x: i32,
        pub y: i32
    }

    #[derive(Copy, Clone)]
    pub struct Triangle2D {
        pub points: [Pixel; 3]
    }

    #[derive(Copy, Clone)]
    pub struct ScanlineH {
        pub l: Pixel,
        pub r: Pixel
    }
}

#[derive(Copy, Clone)]
pub struct LinearIntensity {
    pub l: f32,
    pub r: f32
}
