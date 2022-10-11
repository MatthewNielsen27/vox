extern crate nalgebra as na;

pub type Vertex3D = na::Point3<f32>;

pub mod raster {
    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
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

    impl Triangle2D {
        pub fn from_points(points: &[(i32, i32); 3]) -> Self {
            Triangle2D {
                points: [
                    Pixel {x: points[0].0, y: points[0].1},
                    Pixel {x: points[1].0, y: points[1].1},
                    Pixel {x: points[2].0, y: points[2].1}
                ]
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct LinearIntensity {
    pub l: f32,
    pub r: f32
}
