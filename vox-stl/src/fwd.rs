#[derive(Default, Copy, Clone)]
pub struct Vec3(pub [f32; 3]);

pub type Normal = Vec3;
pub type Vertex = Vec3;

#[derive(Default, Copy, Clone)]
pub struct Facet {
    pub tri: [Vertex; 3],
    pub normal: Normal
}
