extern crate nalgebra as na;

use na::{Vector3, Rotation3};

// type Vertex2D = Vector2<i32>;
pub type Vertex3D = Vector3<f32>;

pub struct Triangle3D {
    pub vertices: Vector3<Vertex3D>
}

// todo: these need to be named to be more specific to raster-land

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub x: i32,
    pub y: i32
}

#[derive(Copy, Clone)]
pub struct Triangle2D {
    pub points: (Vertex2D, Vertex2D, Vertex2D)
}

#[derive(Copy, Clone)]
pub struct HLine{
    pub l: Vertex2D,
    pub r: Vertex2D
}

#[derive(Copy, Clone)]
pub struct LinearIntensity {
    pub l: f32,
    pub r: f32
}
